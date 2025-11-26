use std::process;

use chrono::{DateTime, Utc};
use clap::Parser;
use matrix_sdk::{
    Client,
    ruma::{RoomId, events::room::message::RoomMessageEventContent},
};
use tokio::main;

use crate::{
    cli::Cli,
    github::{get_workflow_event, log},
};

mod cli;
mod github;

#[main]
async fn main() {
    match handle().await {
        Ok(_) => {
            log("Success!", github::LogLevel::Info);
        }
        Err(e) => {
            log(format!("Failed! {}", e).as_str(), github::LogLevel::Error);
            process::exit(1);
        }
    }
}

async fn handle() -> anyhow::Result<()> {
    let cli = Cli::parse();
    log("Processing workflow status update", github::LogLevel::Info);
    let client = Client::builder()
        .homeserver_url(cli.home_server)
        .build()
        .await?;
    client
        .matrix_auth()
        .login_username(cli.username, &cli.password)
        .await?;
    let room_id = RoomId::parse(&cli.room_id)?;
    log("Matrix client logged in", github::LogLevel::Info);
    let room = client.join_room_by_id(&room_id).await?;
    log("Found Matrix room", github::LogLevel::Info);
    let event = get_workflow_event()?;
    let (status_icon, status_text) = match event.workflow_run.status {
        github::WorkflowStatus::Queued => ("⏳", "Pending"),
        github::WorkflowStatus::InProgress => ("🔄", "Running"),
        github::WorkflowStatus::Completed => match event.workflow_run.conclusion {
            Some(github::WorkflowConclusion::Success) => ("✅", "Succeeded"),
            Some(github::WorkflowConclusion::Failure) => ("❌", "Failed"),
            None => ("❔", "Unknown"),
        },
    };
    let duration = match (
        &event.workflow_run.run_started_at,
        &event.workflow_run.completed_at,
        event.workflow_run.status,
    ) {
        (Some(start), Some(end), github::WorkflowStatus::Completed) => {
            Some(format_duration(start, end))
        }
        _ => None,
    };

    let fallback_content = format!(
        "{} CI Workflow {}
    Repo: {}
    Workflow: {}
    Branch: {}
    Commit: {}
    Run: {}
    Actor: {}
    {}
    [ci-run:{}]",
        status_icon,
        status_text,
        event.workflow_run.repository.full_name,
        event.workflow_run.name,
        event.workflow_run.head_branch,
        event.workflow_run.head_sha,
        event.workflow_run.html_url,
        event.workflow_run.actor.login,
        duration
            .as_ref()
            .map(|d| format!("Duration: {}", d))
            .unwrap_or_default(),
        event.workflow_run.id
    );

    let rich_content = format!(
        "{} <strong>CI Workflow {}</strong><br>
    • <strong>Repo:</strong> <a href=\"{}\"><code>{}</code></a><br>
    • <strong>Workflow:</strong> {}<br>
    • <strong>Branch:</strong> {}<br>
    • <strong>Commit:</strong> {}<br>
    • <strong>Run:</strong> <a href=\"{}\">{}</a><br>
    • <strong>Actor:</strong> <code>{}</code><br>
    {}
    <br><code>[ci-run:{}]</code>",
        status_icon,
        status_text,
        event.workflow_run.repository.html_url,
        event.workflow_run.repository.full_name,
        event.workflow_run.name,
        event.workflow_run.head_branch,
        event.workflow_run.head_sha,
        event.workflow_run.html_url,
        event.workflow_run.html_url,
        event.workflow_run.actor.login,
        duration
            .as_ref()
            .map(|d| format!("<strong>Duration:</strong> {}<br>", d))
            .unwrap_or_default(),
        event.workflow_run.id
    );

    let content = RoomMessageEventContent::notice_html(fallback_content, rich_content);
    room.send(content).await?;
    log("Workflow status sent", github::LogLevel::Info);
    Ok(())
}

fn format_duration(start: &DateTime<Utc>, end: &DateTime<Utc>) -> String {
    let duration = *end - *start;
    let total_seconds = duration.num_seconds();

    let minutes = total_seconds / 60;
    let seconds = total_seconds % 60;

    format!("{}m {}s", minutes, seconds)
}
