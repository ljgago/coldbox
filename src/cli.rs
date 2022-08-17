use bdk::bitcoin::util::bip32::{DerivationPath, ExtendedPrivKey};
use bdk::bitcoin::Network;
use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(
    author,
    version,
    about,
    long_about = None,
)]
pub struct Cli {
    /// Sets the network
    #[clap(
        short,
        long,
        value_parser,
        value_names = &["NETWORK"],
        possible_values = ["bitcoin", "testnet", "signet", "regtest"],
        default_value = "testnet",
    )]
    pub network: Network,

    #[clap(subcommand)]
    pub command: CliCommand,
}

// 1st command level
#[derive(Debug, Subcommand)]
pub enum CliCommand {
    /// Key management
    Key(Key),

    /// Wallet operations
    Wallet(Wallet),
}

#[derive(Debug, Args)]
pub struct Key {
    #[clap(subcommand)]
    pub command: KeyCommand,
}

#[derive(Debug, Args)]
pub struct Wallet {
    #[clap(subcommand)]
    pub command: WalletCommand,
}

// 2nd command level
#[derive(Debug, Subcommand)]
pub enum KeyCommand {
    /// Change the version type (e.g. xpub -> zpub)
    Change(Change),

    /// Generate the derivation path
    Derive(Derive),

    /// Generates new random seed mnemonic phrase and corresponding master extended key
    Generate(Generate),

    /// Restore a master extended key from seed backup mnemonic words
    Restore(Restore),
}

#[derive(Debug, Subcommand)]
pub enum WalletCommand {
    /// Wallet
    Balance(Balance),
}

// 3th argument level
#[derive(Debug, Args)]
pub struct Change {
    /// Target format
    #[clap(
        short,
        long,
        value_parser,
        value_names = &["FORMAT"],
    )]
    pub format: String,

    /// Key source
    #[clap(
        short,
        long,
        value_parser,
        value_names = &["KEY"],
    )]
    pub key: String,
}

#[derive(Debug, Args)]
pub struct Derive {
    /// Derivation path (e.g. "m/84'/0'/0'" or "m/84h/0h/0h")
    #[clap(
        short,
        long,
        value_parser,
        value_names = &["PATH"],
        default_value = "m/84'/0'/0'",
    )]
    pub path: DerivationPath,

    /// Private key
    #[clap(
        short,
        long,
        value_parser,
        value_names = &["XPRV"],
    )]
    pub xprv: ExtendedPrivKey,
}

#[derive(Debug, Args)]
pub struct Generate {
    /// Dice roll of random seed
    #[clap(
        short,
        long,
        value_parser,
        value_names = &["NUMBER"],
        default_value = "99",
    )]
    pub dicerolls: usize,

    /// Entropy level based on number of random seed mnemonic words
    #[clap(
        short,
        long,
        value_parser,
        value_names = &["NUMBER"],
        possible_values = ["12", "24"],
        default_value = "12",
    )]
    pub entropy: u8,

    /// Seed password
    #[clap(
        short,
        long,
        value_parser,
        value_names = &["PASSWORD"],
    )]
    pub password: Option<String>,
}

#[derive(Debug, Args)]
pub struct Restore {
    /// Seed mnemonic words, must be quoted (eg. "word1 word2 ...")
    #[clap(
        short,
        long,
        value_parser,
        value_names = &["MNEMONIC"],
    )]
    pub mnemonic: String,

    /// Seed password
    #[clap(
        short,
        long,
        value_parser,
        value_names = &["PASSWORD"],
    )]
    pub password: Option<String>,

}

#[derive(Debug, Args)]
pub struct Balance {}
