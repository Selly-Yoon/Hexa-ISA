use hexa_core::meta_field::FallbackPolicy;

#[derive(Debug)]
pub enum FallbackResult {
    SafeDefault(u64),
    Retried,
    Escalated,
    Aborted(String),
}

pub fn handle_fallback(
    policy: &FallbackPolicy,
    reason: &str,
) -> FallbackResult {
    match policy {
        FallbackPolicy::Abort    => FallbackResult::Aborted(reason.to_string()),
        FallbackPolicy::Reduce   => FallbackResult::SafeDefault(0),
        FallbackPolicy::Retry    => FallbackResult::Retried,
        FallbackPolicy::Escalate => FallbackResult::Escalated,
    }
}
