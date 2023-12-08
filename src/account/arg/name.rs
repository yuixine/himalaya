use clap::Parser;

/// The account name argument parser.
#[derive(Debug, Parser)]
pub struct AccountNameArg {
    /// The name of the account.
    ///
    /// An account name corresponds to an entry in the table at the
    /// root level of your TOML configuration file.
    #[arg(name = "account_name", value_name = "ACCOUNT")]
    pub name: String,
}

/// The account name flag parser.
#[derive(Debug, Parser)]
pub struct AccountNameFlag {
    /// Override the default account.
    ///
    /// An account name corresponds to an entry in the table at the
    /// root level of your TOML configuration file.
    #[arg(long = "account", short = 'a', global = true)]
    #[arg(name = "account_name", value_name = "NAME")]
    pub name: Option<String>,
}
