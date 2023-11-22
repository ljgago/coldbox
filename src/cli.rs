use bdk::bitcoin::bip32::{DerivationPath, ExtendedPrivKey};
use bdk::bitcoin::Network;
use clap::builder::{TypedValueParser, PossibleValuesParser};
use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(
    author,
    version,
    about,
    long_about = None,
    verbatim_doc_comment,
)]
pub struct Cli {
    /// Sets the network
    #[arg(
        short,
        long,
        value_enum,
        value_parser = PossibleValuesParser::new(["bitcoin", "testnet", "signet", "regtest"])
            .try_map(|s| s.parse::<Network>()),
        value_name = "NETWORK",
        default_value = "testnet",
    )]
    pub network: Network,

    #[command(subcommand)]
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
    #[command(subcommand)]
    pub command: KeyCommand,
}

#[derive(Debug, Args)]
pub struct Wallet {
    /// Sets the descriptor to use for the external addresses
    #[arg(short, long, value_parser, value_name = "DESCRIPTOR")]
    pub descriptor: String,

    /// Sets the Electrum server to use
    #[clap(
        short,
        long,
        value_parser,
        value_name = "ELECTRUM_URL",
        default_value = "ssl://electrum.blockstream.info:60002"
    )]
    pub server: String,

    /// Adds verbosity, returns PSBT in JSON format alongside serialized, displays expanded objects
    #[arg(short, long, value_parser, default_value = "false", global = true)]
    pub verbose: bool,

    #[command(subcommand)]
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

    /// Sign
    Sign(Sign),
}

// 3th argument level
#[derive(Debug, Args)]
pub struct Change {
    /// Target format
    #[arg(short, long, value_name = "FORMAT")]
    pub format: String,

    /// Key source
    #[arg(short, long, value_name = "KEY")]
    pub key: String,
}

#[derive(Debug, Args)]
pub struct Derive {
    /// Derivation path (e.g. "m/84'/0'/0'" or "m/84h/0h/0h")
    #[arg(short, long, value_name = "PATH", default_value = "m/84'/0'/0'")]
    pub path: DerivationPath,

    /// Private key
    #[arg(short, long, value_name = "XPRV")]
    pub xprv: ExtendedPrivKey,
}

#[derive(Debug, Args)]
pub struct Generate {
    /// Dice roll of random seed
    #[arg(short, long, value_parser, value_name = "NUMBER", default_value_t = 99)]
    pub dicerolls: usize,

    /// Entropy level based on number of random seed mnemonic words
    #[arg(
        short,
        long,
        value_parser = PossibleValuesParser::new(["12", "24"])
            .try_map(|s| s.parse::<u8>()),
        value_name = "NUMBER",
        default_value_t = 12,
    )]
    pub entropy: u8,

    /// Seed password
    #[arg(short, long, value_name = "PASSWORD")]
    pub password: Option<String>,
}

#[derive(Debug, Args)]
pub struct Restore {
    /// Seed mnemonic words, must be quoted (eg. "word1 word2 ...")
    #[arg(short, long, value_name = "MNEMONIC")]
    pub mnemonic: String,

    /// Seed password
    #[arg(short, long, value_name = "PASSWORD")]
    pub password: Option<String>,
}

#[derive(Debug, Args)]
pub struct Balance {}

#[derive(Debug, Args)]
pub struct Sign {
    /// Sets the PSBT to sign
    #[arg(short, long, value_name = "BASE64_PSBT")]
    pub psbt: String,
}
