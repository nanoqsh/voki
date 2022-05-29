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
    let listen = tokio::spawn(listen(args.address(), sender));
    let manage = tokio::spawn(manage(receiver));
    let _ = tokio::join!(listen, manage);
}
