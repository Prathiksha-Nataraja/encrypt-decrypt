use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct CLIAPP {
    #[clap(subcommand)]
    pub actions: UserSubCommand,
}

#[derive(Debug, Subcommand)]
pub enum UserSubCommand {
    Encrypt{ key: String , file_path: String},
    Decrypt{ key: String , file_path: String},
    Generate,
}
