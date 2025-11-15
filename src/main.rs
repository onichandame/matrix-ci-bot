use std::process;

use anyhow::anyhow;
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
    let event = get_workflow_event()?;
    log("Processing workflow status update", github::LogLevel::Info);
    let client = Client::builder()
        .homeserver_url(cli.home_server)
        .build()
        .await?;
    client
        .matrix_auth()
        .login_username(cli.username, &cli.password)
        .await?;
    log("Matrix client logged in", github::LogLevel::Info);
    let room = client
        .get_room(&RoomId::parse(cli.room_id)?)
        .ok_or(anyhow!("Room not found"))?;
    log("Found Matrix room", github::LogLevel::Info);
    let (status_text, status_color) = match event.workflow_run.status {
        github::WorkflowStatus::Requested => ("Pending", "gray"),
        github::WorkflowStatus::InProgress => ("Running", "blue"),
        github::WorkflowStatus::Completed => match event.workflow_run.conclusion {
            Some(github::WorkflowConclusion::Success) => ("Succeeded", "green"),
            Some(github::WorkflowConclusion::Failure) => ("Failed", "red"),
            None => ("Unknown", "black"),
        },
    };
    let fallback_content = format!(
        "CI Workflow {}
    Repo: {}
    Workflow: {}
    Branch: {}
    Commit: {}
    Run: {}
    [ci-run:{}]",
        status_text,
        event.workflow_run.repository.name,
        event.workflow_run.name,
        event.workflow_run.head_branch,
        event.workflow_run.head_sha,
        event.workflow_run.html_url,
        event.workflow_run.id
    );
    let rich_content = format!(
        "🛠️ <strong>CI Workflow <span style=\"color:{}\">{}</span></strong><br>
    • <strong>Repo:</strong> {}<br>
    • <strong>Workflow:</strong> {}<br>
    • <strong>Branch:</strong> {}<br>
    • <strong>Commit:</strong> {}<br>
    • <strong>Run:</strong> <a href=\"{}\">{}</a><br><br>
    <code>[ci-run:{}]</code>",
        status_color,
        status_text,
        event.workflow_run.repository.name,
        event.workflow_run.name,
        event.workflow_run.head_branch,
        event.workflow_run.head_sha,
        event.workflow_run.html_url,
        event.workflow_run.html_url,
        event.workflow_run.id
    );
    let content = RoomMessageEventContent::notice_html(fallback_content, rich_content);
    room.send(content).await?;
    log("Workflow status sent", github::LogLevel::Info);
    Ok(())
}
