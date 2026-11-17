use clap::Parser;

#[derive(Debug, Parser)]
#[command(name = "phantom", version, about = "Ephemeral P2P messaging")]
struct Cli {}

fn main() {
    let _cli = Cli::parse();
}
