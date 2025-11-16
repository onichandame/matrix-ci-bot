use core::fmt;

#[derive(serde::Deserialize)]
pub(crate) struct GithubEvent {
    pub(crate) workflow_run: WorkflowRun,
}

#[derive(serde::Deserialize)]
pub(crate) struct WorkflowRun {
    pub(crate) status: WorkflowStatus,
    pub(crate) name: String,
    pub(crate) head_branch: String,
    pub(crate) head_sha: String,
    pub(crate) id: u64,
    pub(crate) repository: Repository,
    pub(crate) actor: Actor,
    pub(crate) html_url: String,
    pub(crate) conclusion: Option<WorkflowConclusion>,
}

#[derive(serde::Deserialize)]
pub(crate) struct Actor {
    pub(crate) login: String,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum WorkflowStatus {
    Queued,
    InProgress,
    Completed,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub(crate) enum WorkflowConclusion {
    Success,
    Failure,
}

#[derive(serde::Deserialize)]
pub(crate) struct Repository {
    pub(crate) full_name: String,
    pub(crate) html_url: String,
}
pub(crate) enum LogLevel {
    Info,
    Error,
}

#[derive(thiserror::Error, Debug)]
pub(crate) enum GithubError {
    #[error("{0}")]
    EnvVar(#[from] std::env::VarError),
    #[error("{0}")]
    Io(#[from] std::io::Error),
    #[error("{0}")]
    Json(#[from] serde_json::Error),
}

impl fmt::Display for LogLevel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            LogLevel::Info => "info",
            LogLevel::Error => "error",
        };
        write!(f, "{}", s)
    }
}

pub(crate) fn get_workflow_event() -> Result<GithubEvent, GithubError> {
    let event_name = std::env::var("GITHUB_EVENT_NAME")?;
    if event_name != "workflow_run" {
        panic!("Expected workflow_run event, got: {}", event_name)
    }
    let event_path = std::env::var("GITHUB_EVENT_PATH")?;
    let event_str = std::fs::read_to_string(event_path)?;
    let event: GithubEvent = serde_json::from_str(&event_str)?;
    Ok(event)
}

pub(crate) fn log(msg: &str, level: LogLevel) {
    println!("::{}::{}", level, msg);
}
