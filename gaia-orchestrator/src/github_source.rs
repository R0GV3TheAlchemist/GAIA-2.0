//! Local policy primitives for the read-only GitHub source capability.
//! No provider credentials, network transport, or mutation operations live here.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SourceOperation {
    RepositoryGet,
    TreeList,
    FileRead,
    CommitGet,
    PullRequestRead,
    IssueRead,
    SearchCode,
    SearchDocs,
    SnapshotCreate,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GitHubSourcePolicy {
    allowed_repositories: Vec<String>,
    allowed_paths: Vec<String>,
    denied_path_patterns: Vec<String>,
    max_file_bytes: u64,
    max_response_bytes: u64,
    max_results: u64,
    requests_per_minute: u64,
}

impl Default for GitHubSourcePolicy {
    fn default() -> Self {
        crate::load_github_source_policy(include_str!(
            "../../gaia-spec/policies/github-source.default.json",
        ))
        .expect("checked-in GitHub source policy must be valid")
    }
}

impl GitHubSourcePolicy {
    pub(crate) fn from_loaded(
        allowed_repositories: Vec<String>,
        allowed_paths: Vec<String>,
        denied_path_patterns: Vec<String>,
        max_file_bytes: u64,
        max_response_bytes: u64,
        max_results: u64,
        requests_per_minute: u64,
    ) -> Self {
        Self {
            allowed_repositories,
            allowed_paths,
            denied_path_patterns,
            max_file_bytes,
            max_response_bytes,
            max_results,
            requests_per_minute,
        }
    }

    pub fn max_file_bytes(&self) -> u64 {
        self.max_file_bytes
    }

    pub fn max_response_bytes(&self) -> u64 {
        self.max_response_bytes
    }

    pub fn max_results(&self) -> u64 {
        self.max_results
    }

    pub fn requests_per_minute(&self) -> u64 {
        self.requests_per_minute
    }

    pub fn allows_repository(&self, repository: &str) -> bool {
        self.allowed_repositories
            .iter()
            .any(|item| item == repository)
    }

    pub fn allows_operation(&self, operation: SourceOperation) -> bool {
        matches!(
            operation,
            SourceOperation::RepositoryGet
                | SourceOperation::TreeList
                | SourceOperation::FileRead
                | SourceOperation::CommitGet
                | SourceOperation::PullRequestRead
                | SourceOperation::IssueRead
                | SourceOperation::SearchCode
                | SourceOperation::SearchDocs
                | SourceOperation::SnapshotCreate
        )
    }

    pub fn allows_path(&self, path: &str) -> bool {
        !is_sensitive_path(path)
            && !self
                .denied_path_patterns
                .iter()
                .any(|pattern| path_denied(pattern, path))
            && self
                .allowed_paths
                .iter()
                .any(|rule| path_matches(rule, path))
    }

    pub fn authorize(
        &self,
        repository: &str,
        path: &str,
        operation: SourceOperation,
    ) -> Result<(), String> {
        if !self.allows_repository(repository) {
            return Err("repository denied by source policy".into());
        }
        if !self.allows_operation(operation) {
            return Err("operation denied by read-only source policy".into());
        }
        if !self.allows_path(path) {
            return Err("path denied by source policy".into());
        }
        Ok(())
    }
}

fn path_matches(rule: &str, path: &str) -> bool {
    rule.strip_suffix("/**")
        .map(|prefix| path == prefix || path.starts_with(&format!("{prefix}/")))
        .unwrap_or(rule == path)
}

fn path_denied(pattern: &str, path: &str) -> bool {
    let path = path.replace('\\', "/");
    if let Some(rest) = pattern.strip_prefix("**/") {
        if let Some(suffix) = rest.strip_prefix("*.") {
            return path
                .to_ascii_lowercase()
                .ends_with(&format!(".{suffix}").to_ascii_lowercase());
        }
        if let Some(mid) = rest.strip_suffix("/**") {
            let needle = format!("/{mid}/");
            return path == *mid || path.starts_with(&format!("{mid}/")) || path.contains(&needle);
        }
        return path == rest || path.ends_with(&format!("/{rest}"));
    }
    if let Some(prefix) = pattern.strip_suffix(".*") {
        return path.starts_with(prefix) && path.len() > prefix.len();
    }
    if let Some(prefix) = pattern.strip_suffix("/**") {
        return path == prefix || path.starts_with(&format!("{prefix}/"));
    }
    pattern == path
}

fn is_sensitive_path(path: &str) -> bool {
    let lower = path.to_ascii_lowercase();
    lower == ".env"
        || lower.starts_with(".env.")
        || lower.ends_with(".pem")
        || lower.ends_with(".key")
        || lower == "id_rsa"
        || lower.ends_with("/id_rsa")
        || lower.starts_with("secrets/")
        || lower.contains("/secrets/")
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceCacheKey(pub String);

impl SourceCacheKey {
    pub fn from_immutable_revision(
        repository: &str,
        commit_sha: &str,
        path: &str,
        content_hash: &str,
    ) -> Result<Self, String> {
        if !valid_sha(commit_sha) {
            return Err("resolved commit must be 40 lower-case hex characters".into());
        }
        if !valid_content_hash(content_hash) {
            return Err("content hash must be sha256 plus 64 lower-case hex characters".into());
        }
        Ok(Self(format!(
            "github:{repository}:{commit_sha}:{path}:{content_hash}"
        )))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceAuditInput {
    pub repository: String,
    pub resolved_commit_sha: String,
    pub path: String,
    pub outcome: String,
    pub content_hash: Option<String>,
}

impl SourceAuditInput {
    pub fn allowed(repository: &str, commit_sha: &str, path: &str, content_hash: &str) -> Self {
        Self {
            repository: repository.into(),
            resolved_commit_sha: commit_sha.into(),
            path: path.into(),
            outcome: "allowed".into(),
            content_hash: Some(content_hash.into()),
        }
    }

    pub fn denied(repository: &str, commit_sha: &str, path: &str) -> Self {
        Self {
            repository: repository.into(),
            resolved_commit_sha: commit_sha.into(),
            path: path.into(),
            outcome: "denied".into(),
            content_hash: None,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct FakeGitHubSourceProvider;

impl FakeGitHubSourceProvider {
    pub fn read(
        &self,
        policy: &GitHubSourcePolicy,
        repository: &str,
        path: &str,
    ) -> Result<String, String> {
        policy.authorize(repository, path, SourceOperation::FileRead)?;
        Ok(format!("fake-github-source:{repository}:{path}"))
    }
}

fn valid_sha(value: &str) -> bool {
    value.len() == 40
        && value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
}

fn valid_content_hash(value: &str) -> bool {
    value.strip_prefix("sha256:").is_some_and(|hash| {
        hash.len() == 64
            && hash
                .bytes()
                .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
    })
}
