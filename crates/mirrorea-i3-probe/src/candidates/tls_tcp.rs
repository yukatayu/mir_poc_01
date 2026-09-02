//! Candidate A: TLS over a length-framed reliable TCP byte stream.
//!
//! This module transports only the private I3-0 probe bytes. It neither
//! decodes them into semantic authority nor turns a TLS connection, peer, or
//! certificate into an admission decision; the common harness owns those
//! retained-contract checks after it receives the raw bytes below.

use std::{io, sync::Arc};

use rustls::{
    ClientConfig, RootCertStore, ServerConfig,
    pki_types::{CertificateDer, PrivateKeyDer, ServerName},
};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};
use tokio_rustls::{TlsAcceptor, TlsConnector};

use crate::{
    ClientChildProbeReplyReceipt, MAX_PRIVATE_FRAME_BYTES,
    candidates::{
        CandidateCaseInput, CandidateExecutionError, CandidateExecutionErrorKind,
        CandidateTransportDisposition, CandidateTransportObservation,
    },
    process_harness::{
        CandidateChildError, ChildProcessControl, ChildProcessEvent, ChildProcessHarness,
        ChildRole, ChildTransportFailureClass,
    },
    receiver_canary::ReceiverChildIngress,
};

const CONNECTION_CLOSE_MECHANISM: &str = "tls-tcp-connection-close";
const FRAMED_STREAM_MECHANISM: &str = "tls-tcp-framed-reliable-stream";

/// Runs one real TLS/TCP server/client child pair through the common
/// supervisor. The child processes report only transport-local bytes/failure
/// classes; retained Core/artifact admission stays in `process_harness`.
pub(crate) fn execute(
    input: CandidateCaseInput,
    harness: &mut ChildProcessHarness,
) -> Result<CandidateTransportObservation, CandidateExecutionError> {
    let server = harness.spawn_server().map_err(map_child_error)?;
    let endpoint = match harness.next_event(server).map_err(map_child_error)? {
        ChildProcessEvent::Ready { endpoint } => endpoint,
        ChildProcessEvent::TransportComplete { .. }
        | ChildProcessEvent::ReceiverChildReport { .. }
        | ChildProcessEvent::ClientProbeReplyReceipts { .. }
        | ChildProcessEvent::TransportFailure { .. } => {
            return Err(CandidateExecutionError::new(
                CandidateExecutionErrorKind::ChildProtocolRejected,
            ));
        }
    };
    let client = harness.spawn_client(endpoint).map_err(map_child_error)?;

    // The actual client child parses every reply it receives into
    // reference-only receipts. This wait is bounded exclusively by the common
    // harness deadline; it is not a retry loop.
    let client_event = harness.next_event(client).map_err(map_child_error)?;
    if !matches!(
        client_event,
        ChildProcessEvent::ClientProbeReplyReceipts { .. }
    ) {
        return Err(CandidateExecutionError::new(
            CandidateExecutionErrorKind::ChildProtocolRejected,
        ));
    }

    let server_event = harness.next_event(server).map_err(map_child_error)?;
    observation_from_child_process_events(input.case(), server_event, client_event)
}

/// Performs one candidate child role. A role contains only an in-memory
/// certificate/key or trust root delivered through private stdin control;
/// nothing is read from a file, command line, environment, or OS trust store.
pub(crate) fn execute_child(
    control: ChildProcessControl,
) -> Result<ChildProcessEvent, CandidateExecutionError> {
    install_ring_provider()?;
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .map_err(|_| transport_failure())?;
    match control.role() {
        ChildRole::Server => runtime.block_on(run_server(control)),
        ChildRole::Client => runtime.block_on(run_client(control)),
    }
}

fn observation_from_child_process_events(
    case: crate::CandidateCase,
    server_event: ChildProcessEvent,
    client_event: ChildProcessEvent,
) -> Result<CandidateTransportObservation, CandidateExecutionError> {
    let disposition = match (&server_event, case) {
        (ChildProcessEvent::ReceiverChildReport { .. }, _) => match case {
            crate::CandidateCase::ConnectWithoutSemanticAdmission => {
                CandidateTransportDisposition::Connected
            }
            crate::CandidateCase::DeterministicFragmentedRoundTrip => {
                CandidateTransportDisposition::CompleteFrame
            }
            crate::CandidateCase::DisconnectAfterAdmissionBeforeResult => {
                CandidateTransportDisposition::DisconnectAfterAdmissionBeforeResult
            }
            crate::CandidateCase::DuplicateAcrossReconnect => {
                CandidateTransportDisposition::DuplicateAcrossReconnect
            }
            crate::CandidateCase::TamperedSemanticAdmissionReference => {
                CandidateTransportDisposition::TamperedSemanticAdmissionReference
            }
            crate::CandidateCase::ObserverSafeEvidence => {
                CandidateTransportDisposition::ObserverSafeEvidence
            }
            crate::CandidateCase::TruncatedFrame
            | crate::CandidateCase::OversizedFrame
            | crate::CandidateCase::DisconnectBeforeAdmission => {
                return Err(transport_failure());
            }
        },
        (ChildProcessEvent::TransportFailure { class, .. }, _) => match (case, *class) {
            (crate::CandidateCase::TruncatedFrame, ChildTransportFailureClass::TruncatedFrame) => {
                CandidateTransportDisposition::TruncatedFrame
            }
            (crate::CandidateCase::OversizedFrame, ChildTransportFailureClass::OversizedFrame) => {
                CandidateTransportDisposition::OversizedFrame
            }
            (
                crate::CandidateCase::DisconnectBeforeAdmission,
                ChildTransportFailureClass::DisconnectBeforeAdmission,
            ) => CandidateTransportDisposition::DisconnectBeforeAdmission,
            _ => return Err(transport_failure()),
        },
        (
            ChildProcessEvent::Ready { .. }
            | ChildProcessEvent::ClientProbeReplyReceipts { .. }
            | ChildProcessEvent::TransportComplete { .. },
            _,
        ) => {
            return Err(CandidateExecutionError::new(
                CandidateExecutionErrorKind::ChildProtocolRejected,
            ));
        }
    };
    CandidateTransportObservation::from_child_process_events(
        mechanism_for(case),
        disposition,
        server_event,
        client_event,
    )
}

fn map_child_error(error: CandidateChildError) -> CandidateExecutionError {
    let kind = match error {
        CandidateChildError::Protocol | CandidateChildError::CandidateUnavailable => {
            CandidateExecutionErrorKind::ChildProtocolRejected
        }
        CandidateChildError::Lifecycle => CandidateExecutionErrorKind::ChildLifecycleFailed,
        CandidateChildError::Deadline => CandidateExecutionErrorKind::DeadlineExceeded,
    };
    CandidateExecutionError::new(kind)
}

fn mechanism_for(case: crate::CandidateCase) -> &'static str {
    match case {
        crate::CandidateCase::DisconnectBeforeAdmission
        | crate::CandidateCase::DisconnectAfterAdmissionBeforeResult => CONNECTION_CLOSE_MECHANISM,
        crate::CandidateCase::ConnectWithoutSemanticAdmission
        | crate::CandidateCase::DeterministicFragmentedRoundTrip
        | crate::CandidateCase::TruncatedFrame
        | crate::CandidateCase::OversizedFrame
        | crate::CandidateCase::DuplicateAcrossReconnect
        | crate::CandidateCase::TamperedSemanticAdmissionReference
        | crate::CandidateCase::ObserverSafeEvidence => FRAMED_STREAM_MECHANISM,
    }
}

fn install_ring_provider() -> Result<(), CandidateExecutionError> {
    // Each server/client role is an independently spawned OS process. No
    // fallback provider or permissive verification configuration is installed.
    rustls::crypto::ring::default_provider()
        .install_default()
        .map_err(|_| transport_failure())
}

async fn run_server(
    mut control: ChildProcessControl,
) -> Result<ChildProcessEvent, CandidateExecutionError> {
    let listener = TcpListener::bind("127.0.0.1:0")
        .await
        .map_err(|_| transport_failure())?;
    let endpoint = listener
        .local_addr()
        .map_err(|_| transport_failure())?
        .to_string();
    emit_ready(endpoint)?;

    let acceptor = TlsAcceptor::from(Arc::new(server_config(&mut control)?));
    let mut receiver_child = control.receiver_child_canary().map_err(map_child_error)?;
    let connections =
        usize::from(control.case() == crate::CandidateCase::DuplicateAcrossReconnect) + 1;
    let mut received_frames = Vec::with_capacity(connections);

    for _ in 0..connections {
        let (tcp, _) = listener.accept().await.map_err(|_| transport_failure())?;
        let mut stream = acceptor
            .accept(tcp)
            .await
            .map_err(|_| transport_failure())?;
        match read_server_connection(control.case(), &mut stream).await? {
            ServerRead::Complete(frame) => {
                // Decode/revalidation and stored-decision handling live in the
                // receiver child. TLS executes only its returned reply action.
                let ingress = receiver_child.receive_complete_frame(&frame);
                received_frames.push(frame);
                match ingress {
                    ReceiverChildIngress::Reply(reply)
                        if control.case()
                            == crate::CandidateCase::DisconnectAfterAdmissionBeforeResult =>
                    {
                        // The common result-path cut is after admission,
                        // linearization, and storage, but before any reply
                        // byte leaves this TLS stream.
                        wait_for_peer_close(&mut stream).await?;
                        receiver_child.cut_result_path_before_reply();
                        drop(reply);
                    }
                    ReceiverChildIngress::Reply(reply) => {
                        write_server_reply(&mut stream, reply.as_bytes()).await?;
                    }
                    ReceiverChildIngress::Rejected => {
                        // A rejected ingress deliberately has no application
                        // reply. The peer may already have completed its
                        // TLS close after sending the complete frame, so this
                        // is a best-effort close rather than a new transport
                        // failure or a reason to retry semantic work.
                        let _ = stream.shutdown().await;
                    }
                }
            }
            ServerRead::NoFrame => match control.case() {
                crate::CandidateCase::DisconnectBeforeAdmission => {
                    return Ok(ChildProcessEvent::TransportFailure {
                        class: ChildTransportFailureClass::DisconnectBeforeAdmission,
                        received_capture: Vec::new(),
                    });
                }
                crate::CandidateCase::ConnectWithoutSemanticAdmission => {
                    break;
                }
                _ => return Err(transport_failure()),
            },
            ServerRead::Truncated(received_capture) => {
                return Ok(ChildProcessEvent::TransportFailure {
                    class: ChildTransportFailureClass::TruncatedFrame,
                    received_capture,
                });
            }
            ServerRead::Oversized(received_capture) => {
                return Ok(ChildProcessEvent::TransportFailure {
                    class: ChildTransportFailureClass::OversizedFrame,
                    received_capture,
                });
            }
        }
    }

    Ok(ChildProcessEvent::ReceiverChildReport {
        received_frames,
        report: receiver_child.report(),
    })
}

async fn run_client(
    control: ChildProcessControl,
) -> Result<ChildProcessEvent, CandidateExecutionError> {
    let endpoint = control
        .endpoint()
        .ok_or_else(transport_failure)?
        .to_string();
    let connector = TlsConnector::from(Arc::new(client_config(&control)?));
    let connections =
        usize::from(control.case() == crate::CandidateCase::DuplicateAcrossReconnect) + 1;

    let mut receipts = Vec::with_capacity(connections);
    // This loop is an explicit second occurrence for the sole duplicate case,
    // not a connect retry. Every other case performs exactly one connection.
    for connection_index in 0..connections {
        let tcp = TcpStream::connect(&endpoint)
            .await
            .map_err(|_| transport_failure())?;
        let server_name =
            ServerName::try_from("localhost".to_string()).map_err(|_| transport_failure())?;
        let mut stream = connector
            .connect(server_name, tcp)
            .await
            .map_err(|_| transport_failure())?;
        write_client_connection(control.case(), control.frame(), &mut stream).await?;
        if client_expects_reply(control.case()) {
            let reply = read_private_reply(&mut stream).await?;
            let receipt =
                ClientChildProbeReplyReceipt::from_received_reply(connection_index + 1, &reply)
                    .ok_or_else(transport_failure)?;
            receipts.push(receipt);
        } else {
            // This includes the explicit ambiguous-delivery path: the client
            // closes after ingress before receiving any application reply.
            stream.shutdown().await.map_err(|_| transport_failure())?;
        }
    }

    Ok(ChildProcessEvent::ClientProbeReplyReceipts { receipts })
}

async fn write_client_connection(
    case: crate::CandidateCase,
    frame: &[u8],
    stream: &mut tokio_rustls::client::TlsStream<TcpStream>,
) -> Result<(), CandidateExecutionError> {
    match case {
        crate::CandidateCase::ConnectWithoutSemanticAdmission
        | crate::CandidateCase::DisconnectBeforeAdmission => Ok(()),
        crate::CandidateCase::DeterministicFragmentedRoundTrip => {
            write_fragmented(stream, frame).await
        }
        crate::CandidateCase::TruncatedFrame => write_truncated_frame(stream, frame).await,
        crate::CandidateCase::OversizedFrame => {
            let bytes = oversized_client_prefix()?;
            write_and_flush(stream, &bytes).await
        }
        crate::CandidateCase::DisconnectAfterAdmissionBeforeResult
        | crate::CandidateCase::DuplicateAcrossReconnect
        | crate::CandidateCase::ObserverSafeEvidence
        | crate::CandidateCase::TamperedSemanticAdmissionReference => {
            write_and_flush(stream, frame).await
        }
    }
}

async fn read_server_connection(
    case: crate::CandidateCase,
    stream: &mut tokio_rustls::server::TlsStream<TcpStream>,
) -> Result<ServerRead, CandidateExecutionError> {
    match case {
        crate::CandidateCase::ConnectWithoutSemanticAdmission
        | crate::CandidateCase::DisconnectBeforeAdmission => {
            let mut byte = [0_u8; 1];
            let read = stream
                .read(&mut byte)
                .await
                .map_err(|_| transport_failure())?;
            if read == 0 {
                Ok(ServerRead::NoFrame)
            } else {
                Err(transport_failure())
            }
        }
        crate::CandidateCase::TruncatedFrame => read_complete_frame(stream, true).await,
        crate::CandidateCase::OversizedFrame => read_complete_frame(stream, false).await,
        crate::CandidateCase::DisconnectAfterAdmissionBeforeResult => {
            // Return immediately after a complete ingress. The receiver
            // child must perform admission/linearization/storage before the
            // explicit result-path cut waits for the peer close.
            read_complete_frame(stream, false).await
        }
        crate::CandidateCase::DeterministicFragmentedRoundTrip
        | crate::CandidateCase::DuplicateAcrossReconnect
        | crate::CandidateCase::TamperedSemanticAdmissionReference
        | crate::CandidateCase::ObserverSafeEvidence => read_complete_frame(stream, false).await,
    }
}

async fn read_complete_frame(
    stream: &mut tokio_rustls::server::TlsStream<TcpStream>,
    allow_truncated: bool,
) -> Result<ServerRead, CandidateExecutionError> {
    let prefix = match read_capture(stream, 4).await? {
        ReadCapture::Complete(prefix) => prefix,
        ReadCapture::EndOfStream(capture) if allow_truncated => {
            return Ok(ServerRead::Truncated(capture));
        }
        ReadCapture::EndOfStream(_) => return Err(transport_failure()),
    };
    let prefix: [u8; 4] = prefix.try_into().map_err(|_| transport_failure())?;
    let declared = usize::try_from(u32::from_be_bytes(prefix)).map_err(|_| transport_failure())?;
    // Validate the advertised length before allocating a body buffer.
    if declared > MAX_PRIVATE_FRAME_BYTES {
        return Ok(ServerRead::Oversized(prefix.to_vec()));
    }
    let body = match read_capture(stream, declared).await? {
        ReadCapture::Complete(body) => body,
        ReadCapture::EndOfStream(body) if allow_truncated => {
            let mut capture = prefix.to_vec();
            capture.extend_from_slice(&body);
            return Ok(ServerRead::Truncated(capture));
        }
        ReadCapture::EndOfStream(_) => return Err(transport_failure()),
    };
    let mut frame = Vec::with_capacity(4 + body.len());
    frame.extend_from_slice(&prefix);
    frame.extend_from_slice(&body);
    Ok(ServerRead::Complete(frame))
}

/// Reads at most the requested transport octets without inventing bytes when
/// TLS reaches a clean close. The caller chooses the bounded length only after
/// validating the private frame prefix.
async fn read_capture(
    stream: &mut tokio_rustls::server::TlsStream<TcpStream>,
    expected: usize,
) -> Result<ReadCapture, CandidateExecutionError> {
    let mut capture = Vec::with_capacity(expected);
    let mut buffer = [0_u8; 4096];
    while capture.len() < expected {
        let remaining = expected - capture.len();
        let chunk_len = remaining.min(buffer.len());
        let read = stream
            .read(&mut buffer[..chunk_len])
            .await
            .map_err(|_| transport_failure())?;
        if read == 0 {
            return Ok(ReadCapture::EndOfStream(capture));
        }
        capture.extend_from_slice(&buffer[..read]);
    }
    Ok(ReadCapture::Complete(capture))
}

async fn write_fragmented(
    stream: &mut tokio_rustls::client::TlsStream<TcpStream>,
    frame: &[u8],
) -> Result<(), CandidateExecutionError> {
    let first = frame.first_chunk::<1>().ok_or_else(transport_failure)?;
    write_and_flush(stream, first).await?;
    let remainder = &frame[1..];
    let split = remainder.len().min(7);
    if split != 0 {
        write_and_flush(stream, &remainder[..split]).await?;
    }
    if split != remainder.len() {
        write_and_flush(stream, &remainder[split..]).await?;
    }
    Ok(())
}

async fn write_truncated_frame(
    stream: &mut tokio_rustls::client::TlsStream<TcpStream>,
    frame: &[u8],
) -> Result<(), CandidateExecutionError> {
    let body = frame.get(4..).ok_or_else(transport_failure)?;
    if body.len() < 2 {
        return Err(transport_failure());
    }
    let partial_body_length = (body.len() / 2).max(1);
    write_and_flush(stream, &frame[..4]).await?;
    write_and_flush(stream, &body[..partial_body_length]).await
}

async fn write_and_flush(
    stream: &mut tokio_rustls::client::TlsStream<TcpStream>,
    bytes: &[u8],
) -> Result<(), CandidateExecutionError> {
    stream
        .write_all(bytes)
        .await
        .map_err(|_| transport_failure())?;
    stream.flush().await.map_err(|_| transport_failure())
}

async fn write_server_reply(
    stream: &mut tokio_rustls::server::TlsStream<TcpStream>,
    reply: &[u8],
) -> Result<(), CandidateExecutionError> {
    stream
        .write_all(reply)
        .await
        .map_err(|_| transport_failure())?;
    stream.flush().await.map_err(|_| transport_failure())?;
    stream.shutdown().await.map_err(|_| transport_failure())
}

async fn wait_for_peer_close(
    stream: &mut tokio_rustls::server::TlsStream<TcpStream>,
) -> Result<(), CandidateExecutionError> {
    let mut byte = [0_u8; 1];
    if stream
        .read(&mut byte)
        .await
        .map_err(|_| transport_failure())?
        != 0
    {
        return Err(transport_failure());
    }
    Ok(())
}

async fn read_private_reply(
    stream: &mut tokio_rustls::client::TlsStream<TcpStream>,
) -> Result<Vec<u8>, CandidateExecutionError> {
    let mut reply = Vec::new();
    let mut buffer = [0_u8; 1024];
    loop {
        let read = stream
            .read(&mut buffer)
            .await
            .map_err(|_| transport_failure())?;
        if read == 0 {
            return Ok(reply);
        }
        if reply
            .len()
            .checked_add(read)
            .is_none_or(|length| length > MAX_PRIVATE_FRAME_BYTES)
        {
            return Err(transport_failure());
        }
        reply.extend_from_slice(&buffer[..read]);
    }
}

const fn client_expects_reply(case: crate::CandidateCase) -> bool {
    matches!(
        case,
        crate::CandidateCase::DeterministicFragmentedRoundTrip
            | crate::CandidateCase::DuplicateAcrossReconnect
            | crate::CandidateCase::ObserverSafeEvidence
    )
}

fn oversized_client_prefix() -> Result<Vec<u8>, CandidateExecutionError> {
    let declared = u32::try_from(MAX_PRIVATE_FRAME_BYTES + 1).map_err(|_| transport_failure())?;
    Ok(declared.to_be_bytes().to_vec())
}

fn server_config(
    control: &mut ChildProcessControl,
) -> Result<ServerConfig, CandidateExecutionError> {
    let certificate = CertificateDer::from(control.certificate_der().to_vec());
    let private_key = control
        .take_transport_private_key()
        .ok_or_else(transport_failure)?;
    ServerConfig::builder()
        .with_no_client_auth()
        .with_single_cert(vec![certificate], PrivateKeyDer::Pkcs8(private_key))
        .map_err(|_| transport_failure())
}

fn client_config(control: &ChildProcessControl) -> Result<ClientConfig, CandidateExecutionError> {
    let mut roots = RootCertStore::empty();
    roots
        .add(CertificateDer::from(control.certificate_der().to_vec()))
        .map_err(|_| transport_failure())?;
    Ok(ClientConfig::builder()
        .with_root_certificates(roots)
        .with_no_client_auth())
}

fn emit_ready(endpoint: String) -> Result<(), CandidateExecutionError> {
    let event = ChildProcessEvent::Ready { endpoint };
    serde_json::to_writer(io::stdout(), &event).map_err(|_| transport_failure())?;
    use std::io::Write as _;
    io::stdout()
        .write_all(b"\n")
        .and_then(|_| io::stdout().flush())
        .map_err(|_| transport_failure())
}

fn transport_failure() -> CandidateExecutionError {
    CandidateExecutionError::new(CandidateExecutionErrorKind::TransportFailed)
}

enum ServerRead {
    Complete(Vec<u8>),
    NoFrame,
    Truncated(Vec<u8>),
    Oversized(Vec<u8>),
}

enum ReadCapture {
    Complete(Vec<u8>),
    EndOfStream(Vec<u8>),
}
