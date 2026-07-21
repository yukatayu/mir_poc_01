#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum HostAdapterOperation {
    ReadInt,
    WriteInt,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum HostAdapterInput {
    None,
    OneInt64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum HostAdapterOutput {
    Unit,
    Int64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct HostAdapterPolicy {
    pub operation: HostAdapterOperation,
    pub input: HostAdapterInput,
    pub output: HostAdapterOutput,
    pub required_capability: &'static str,
}

pub(super) fn resolve_host_adapter_policy(
    effect_name: &str,
    boundary_ref: &str,
) -> Option<HostAdapterPolicy> {
    match (effect_name, boundary_ref) {
        ("read_int", "host_input") => Some(HostAdapterPolicy {
            operation: HostAdapterOperation::ReadInt,
            input: HostAdapterInput::None,
            output: HostAdapterOutput::Int64,
            required_capability: "HostRead",
        }),
        ("write_int", "host_output") => Some(HostAdapterPolicy {
            operation: HostAdapterOperation::WriteInt,
            input: HostAdapterInput::OneInt64,
            output: HostAdapterOutput::Unit,
            required_capability: "HostWrite",
        }),
        _ => None,
    }
}
