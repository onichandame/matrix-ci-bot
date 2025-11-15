#[derive(clap::Parser)]
#[command(version, about)]
pub(crate) struct Cli {
    #[arg(long)]
    pub(crate) home_server: String,
    #[arg(long)]
    pub(crate) username: String,
    #[arg(long)]
    pub(crate) password: String,
    #[arg(long)]
    pub(crate) room_id: String,
}
