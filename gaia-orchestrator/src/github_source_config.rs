//! Strict local validation for the checked-in read-only GitHub source policy.
//! This module performs no network access and accepts no credentials.

use serde::Deserialize;

use crate::GitHubSourcePolicy;

const MAX_BYTES: u64 = 1_048_576;
const MAX_RESULTS: u64 = 50;
const MAX_REQUESTS_PER_MINUTE: u64 = 60;

#[derive(Debug, Deserialize)]
struct SourcePolicyConfig {
    schema_version: String,
    mode: String,
    allowed_repositories: Vec<String>,
    allowed_paths: Vec<String>,
    denied_path_patterns: Vec<String>,
    limits: Limits,
    audit: Audit,
}

#[derive(Debug, Deserialize)]
struct Limits {
    max_file_bytes: u64,
    max_response_bytes: u64,
    max_results: u64,
    requests_per_minute: u64,
    allowed_content_types: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct Audit {
    enabled: bool,
    chain_required: bool,
}

/// Validates a local source-policy document and returns the hardened,
/// read-only policy used by the local fake source provider.
pub fn load_github_source_policy(json: &str) -> Result<GitHubSourcePolicy, String> {
    let config: SourcePolicyConfig =
        serde_json::from_str(json).map_err(|error| format!("invalid source policy JSON: {error}"))?;

    if config.schema_version != "1.0" {
        return Err("unsupported source policy schema version".into());
    }
    if config.mode != "read-only" {
        return Err("source policy mode must be read-only".into());
    }
    if config.allowed_repositories.is_empty() || config.allowed_paths.is_empty() {
        return Err("source policy must allow at least one repository and path".into());
    }
    if config.denied_path_patterns.is_empty() {
        return Err("source policy must deny sensitive path patterns".into());
    }
    if !config.audit.enabled || !config.audit.chain_required {
        return Err("source policy requires enabled chained audit".into());
    }
    if config.limits.max_file_bytes == 0
        || config.limits.max_response_bytes == 0
        || config.limits.max_results == 0
        || config.limits.requests_per_minute == 0
        || config.limits.allowed_content_types.is_empty()
    {
        return Err("source policy limits must be non-zero and content types non-empty".into());
    }
    if config.limits.max_file_bytes > MAX_BYTES
        || config.limits.max_response_bytes > MAX_BYTES
        || config.limits.max_results > MAX_RESULTS
        || config.limits.requests_per_minute > MAX_REQUESTS_PER_MINUTE
    {
        return Err("source policy exceeds local safety limits".into());
    }

    Ok(GitHubSourcePolicy::from_loaded(
        config.allowed_repositories,
        config.allowed_paths,
        config.denied_path_patterns,
        config.limits.max_file_bytes,
        config.limits.max_response_bytes,
        config.limits.max_results,
        config.limits.requests_per_minute,
    ))
}
