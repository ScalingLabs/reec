use crate::{
    commands::{autocomplete, config, info, prove, stack, test, utils, wallet},
    config::load_selected_config,
};
use clap::{Parser, Subcommand};

pub const VERSION_STRING: &str = env!("CARGO_PKG_VERSION");

#[derive(Parser)]
#[command(name="Reec_l2_cli", author, version=VERSION_STRING, about, long_about = None)]
pub struct ReecL2CLI {
    #[command(subcommand)]
    command: ReecL2Command,
}

#[derive(Subcommand)]
enum ReecL2Command {
    #[clap(subcommand, about = "Stack related commands.")]
    Stack(stack::Command),
    #[clap(
        subcommand,
        about = "Wallet interaction commands. The configured wallet could operate both with the L1 and L2 networks.",
        visible_alias = "w"
    )]
    Wallet(wallet::Command),
    #[clap(
        subcommand,
        about = "Different utilities for developers.",
        visible_alias = "u"
    )]
    Utils(utils::Command),
    #[clap(subcommand, about = "CLI config commands.")]
    Config(config::Command),
    #[clap(subcommand, about = "Run tests.")]
    Test(test::Command),
    #[clap(subcommand, about = "Generate shell completion scripts.")]
    Autocomplete(autocomplete::Command),
    #[clap(subcommand, about = "Gets L2's information.")]
    Info(info::Command),
    #[clap(about = "Read a test chain from disk and prove a block.")]
    Prove(prove::Command),
}

pub async fn start() -> eyre::Result<()> {
    let ReecL2CLI { command } = ReecL2CLI::parse();
    if let ReecL2Command::Config(cmd) = command {
        return cmd.run().await;
    }
    if let ReecL2Command::Prove(cmd) = command {
        return cmd.run();
    }

    let cfg = load_selected_config().await?;
    match command {
        ReecL2Command::Stack(cmd) => cmd.run(cfg).await?,
        ReecL2Command::Wallet(cmd) => cmd.run(cfg).await?,
        ReecL2Command::Utils(cmd) => cmd.run().await?,
        ReecL2Command::Autocomplete(cmd) => cmd.run()?,
        ReecL2Command::Config(_) => unreachable!(),
        ReecL2Command::Test(cmd) => cmd.run(cfg).await?,
        ReecL2Command::Info(cmd) => cmd.run(cfg).await?,
        ReecL2Command::Prove(_) => unreachable!(),
    };
    Ok(())
}
