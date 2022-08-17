use bdk::Error;
use clap::Parser;
use log::error;

use coldbox::cli::{Cli, CliCommand, KeyCommand, WalletCommand};

fn main() {
    env_logger::init();

    let cli = Cli::parse();
    let network = cli.network;

    match cli.command {
        CliCommand::Key(key) => match key.command {
            KeyCommand::Change(cmd) => {
                let result = cmd.change();
                output(result);
            },
            KeyCommand::Generate(cmd) => {
                let result = cmd.generate(network);
                output(result);
            },
            KeyCommand::Derive(cmd) => {
                let result = cmd.derive(network);
                output(result);
            },
            KeyCommand::Restore(cmd) => {
                let result = cmd.restore(network);
                output(result);
            },
        },
        CliCommand::Wallet(wallet) => match wallet.command {
            WalletCommand::Balance(_cmd) => {}
        },
    }
}

fn output(result: Result<serde_json::Value, Error>) {
    match result {
        Ok(value) => println!("{}", serde_json::to_string_pretty(&value).unwrap()),
        Err(e) => {
            match e {
                Error::ChecksumMismatch => error!("Descriptor checksum mismatch. Are you using a different descriptor for an already defined wallet name? (if you are not specifying the wallet name it is automatically named based on the descriptor)"),
                e => error!("{}", e.to_string()),
            }
        },
    }
}
