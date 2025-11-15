#[derive(clap::Parser)]
#[command(version, about)]
pub(crate) struct Cli {
    #[arg(long, env)]
    pub(crate) home_server: String,
    #[arg(long, env)]
    pub(crate) username: String,
    #[arg(long, env)]
    pub(crate) password: String,
    #[arg(long, env)]
    pub(crate) room_id: String,
}
