//! Parser-only Stage 3 evidence for the private provider adapter.
//!
//! Parser and bounded-reader evidence for the private provider adapter.
//!
//! The parser tests remain lexical-only. The reader tests use finite synthetic
//! `Read` implementations directly at the post-open helper boundary; actual
//! filesystem absence remains covered by the source-real process test.

use std::io::{self, Cursor, Read, Write};

use super::*;

struct PartialThenErrorReader {
    first_read_completed: bool,
}

impl Read for PartialThenErrorReader {
    fn read(&mut self, buffer: &mut [u8]) -> io::Result<usize> {
        if self.first_read_completed {
            return Err(io::Error::other("synthetic bounded reader failure"));
        }
        buffer[0] = b'4';
        self.first_read_completed = true;
        Ok(1)
    }
}

struct FirstReadNotFound;

impl Read for FirstReadNotFound {
    fn read(&mut self, _buffer: &mut [u8]) -> io::Result<usize> {
        Err(io::Error::new(
            io::ErrorKind::NotFound,
            "synthetic bounded reader absence",
        ))
    }
}

#[test]
fn provider_terminal_observer_body_writer_accepts_exact_cap_without_excess_capacity() {
    let mut writer = BoundedProviderTerminalAuditBodyWriter::new()
        .expect("the bounded private observer writer reserves its fixed protocol body capacity");
    let exact_body = vec![b'x'; MAX_PROVIDER_TERMINAL_AUDIT_BYTES];
    assert_eq!(
        writer
            .write(&exact_body)
            .expect("one exact-cap synthetic observer body fits"),
        MAX_PROVIDER_TERMINAL_AUDIT_BYTES,
        "the bounded writer accepts exactly the declared body limit"
    );
    let body = writer
        .into_body()
        .expect("the exact-cap body remains a valid bounded observer frame body");
    assert_eq!(
        body.len(),
        MAX_PROVIDER_TERMINAL_AUDIT_BYTES,
        "the completed body keeps the exact fixed limit"
    );
    assert!(
        body.capacity() <= MAX_PROVIDER_TERMINAL_AUDIT_BYTES,
        "the private writer never returns capacity beyond the protocol body bound"
    );
}

#[test]
fn provider_terminal_observer_body_writer_rejects_cap_plus_one_before_growth() {
    let mut writer = BoundedProviderTerminalAuditBodyWriter::new()
        .expect("the bounded private observer writer reserves its fixed protocol body capacity");
    let oversized = vec![b'x'; MAX_PROVIDER_TERMINAL_AUDIT_BYTES + 1];
    let error = writer
        .write(&oversized)
        .expect_err("a cap-plus-one synthetic body must reject before it extends the writer");
    assert_eq!(
        error.kind(),
        io::ErrorKind::WriteZero,
        "cap-plus-one is a typed bounded-write rejection"
    );
    let body = writer
        .into_body()
        .expect("the rejected cap-plus-one input leaves a valid empty writer body");
    assert!(
        body.is_empty(),
        "the rejected write does not append a partial observer body"
    );
    assert!(
        body.capacity() <= MAX_PROVIDER_TERMINAL_AUDIT_BYTES,
        "the failed write cannot trigger growth beyond the fixed protocol body bound"
    );
}

#[test]
fn provider_canonical_i64_parser_accepts_exact_decimal_with_optional_final_lf() {
    for (bytes, expected) in [
        (b"0" as &[u8], 0_i64),
        (b"0\n" as &[u8], 0_i64),
        (b"41" as &[u8], 41_i64),
        (b"41\n" as &[u8], 41_i64),
        (b"-7" as &[u8], -7_i64),
        (b"-7\n" as &[u8], -7_i64),
    ] {
        assert_eq!(
            parse_canonical_i64(bytes),
            Some(expected),
            "the parser accepts only the exact canonical decimal spelling, with one optional final LF"
        );
    }

    for bound in [i64::MIN, i64::MAX] {
        let decimal = bound.to_string();
        let decimal_with_lf = format!("{decimal}\n");
        assert_eq!(parse_canonical_i64(decimal.as_bytes()), Some(bound));
        assert_eq!(
            parse_canonical_i64(decimal_with_lf.as_bytes()),
            Some(bound),
            "canonical i64 bounds retain the same optional-final-LF rule"
        );
    }
}

#[test]
fn provider_canonical_i64_parser_rejects_noncanonical_or_non_i64_input() {
    let positive_overflow = (i64::MAX as i128 + 1).to_string();
    let negative_overflow = (i64::MIN as i128 - 1).to_string();
    let fullwidth_one = "\u{ff11}";
    let rejected: &[&[u8]] = &[
        b"",
        b"+1",
        b"01",
        b"-0",
        b" 1",
        b"1 ",
        b"\t1",
        b"1\r",
        b"1\r\n",
        b"1\n\n",
        b"1x",
        positive_overflow.as_bytes(),
        negative_overflow.as_bytes(),
        fullwidth_one.as_bytes(),
        b"\xff",
    ];
    for bytes in rejected {
        assert_eq!(
            parse_canonical_i64(bytes),
            None,
            "noncanonical or out-of-range input must not become a provider value"
        );
    }
}

#[test]
fn provider_bounded_reader_counts_a_33_byte_early_invalid_read_once() {
    let mut reader = Cursor::new([b'0'; PROVIDER_READ_SENTINEL_BYTES]);
    let mut bytes = [0_u8; PROVIDER_READ_SENTINEL_BYTES];
    let mut audit = Sys5I3ProviderTerminalAudit::empty();
    let outcome = read_bounded_provider_stream(&mut reader, &mut bytes, &mut audit);

    assert!(
        matches!(
            outcome.outcome,
            PrivateProviderOutcome::ProviderInvalidResult
        ) && outcome.physical_adapter_entered
            && outcome.actual_read
            && audit.actual_read_count == 1,
        "a bounded sentinel-overflow read remains one actual adapter read even when invalid"
    );
}

#[test]
fn provider_bounded_reader_retains_actual_read_accounting_after_a_later_error() {
    let mut reader = PartialThenErrorReader {
        first_read_completed: false,
    };
    let mut bytes = [0_u8; PROVIDER_READ_SENTINEL_BYTES];
    let mut audit = Sys5I3ProviderTerminalAudit::empty();
    let outcome = read_bounded_provider_stream(&mut reader, &mut bytes, &mut audit);

    assert!(
        matches!(outcome.outcome, PrivateProviderOutcome::AdapterUnavailable)
            && outcome.physical_adapter_entered
            && outcome.actual_read
            && audit.actual_read_count == 1,
        "a successful partial read remains counted when the next bounded read errors"
    );
}

#[test]
fn provider_bounded_reader_does_not_count_an_initial_not_found_error_as_a_read() {
    let mut reader = FirstReadNotFound;
    let mut bytes = [0_u8; PROVIDER_READ_SENTINEL_BYTES];
    let mut audit = Sys5I3ProviderTerminalAudit::empty();
    let outcome = read_bounded_provider_stream(&mut reader, &mut bytes, &mut audit);

    assert!(
        matches!(
            outcome.outcome,
            PrivateProviderOutcome::ProviderResourceNotFound
        ) && outcome.physical_adapter_entered
            && !outcome.actual_read
            && audit.actual_read_count == 0,
        "an initial bounded NotFound error retains adapter entry but no successful read accounting"
    );
}
