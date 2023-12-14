use anyhow::Result;
use clap::Parser;
use log::info;

use crate::{
    account::arg::name::AccountNameFlag, backend::Backend, cache::arg::disable::CacheDisableFlag,
    config::TomlConfig, folder::arg::name::FolderNameArg, printer::Printer,
};

/// Watch a folder for changes.
///
/// This command allows you to watch a new folder using the given
/// name.
#[derive(Debug, Parser)]
pub struct FolderWatchCommand {
    #[command(flatten)]
    pub folder: FolderNameArg,

    #[command(flatten)]
    pub cache: CacheDisableFlag,

    #[command(flatten)]
    pub account: AccountNameFlag,
}

impl FolderWatchCommand {
    pub async fn execute(self, printer: &mut impl Printer, config: &TomlConfig) -> Result<()> {
        info!("executing folder watch command");

        let folder = &self.folder.name;

        let some_account_name = self.account.name.as_ref().map(String::as_str);
        let (toml_account_config, account_config) = config
            .clone()
            .into_account_configs(some_account_name, self.cache.disable)?;
        let backend = Backend::new(toml_account_config, account_config.clone(), false).await?;

        printer.print_log(format!("Start watching folder {folder} for changes…"))?;

        backend.watch_emails(&folder).await
    }
}
