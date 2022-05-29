mod args;
mod event;
mod listen;
mod manage;

use self::{args::Args, listen::listen, manage::manage};
use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    use clap::Parser;

    let args = Args::parse();

    let (sender, receiver) = mpsc::channel(16);
    tokio::select! {
        _ = listen(args.address(), sender) => {},
        _ = manage(receiver) => {},
    }
}
