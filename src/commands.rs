use clap::{CommandFactory, Parser};
use cosmian_kms_cli::{error::result::CliResult, Cli, CliCommands};
use cosmian_kms_client::ClientConf;
use cosmian_logger::{log_utils::log_init, reexport::tracing};

pub async fn gui_main() -> CliResult<()> {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() < 2 {
        let cmd = <Cli as CommandFactory>::command().name("Cosmian KMS");
        klask::run_app(cmd, klask::Settings::default(), |_| {});
        return Ok(());
    }

    log_init(None);
    let opts = Cli::parse();

    if let CliCommands::Markdown(action) = opts.command {
        let command = <Cli as CommandFactory>::command();
        action.process(&command)?;
        return Ok(());
    }

    let conf_path = ClientConf::location(opts.conf)?;

    match opts.command {
        CliCommands::Login(action) => action.process(&conf_path).await?,
        CliCommands::Logout(action) => action.process(&conf_path)?,

        command => {
            let conf = ClientConf::load(&conf_path)?;
            let kms_rest_client = conf.initialize_kms_client(
                opts.url.as_deref(),
                opts.accept_invalid_certs,
                opts.json,
            )?;

            match command {
                CliCommands::Locate(action) => action.process(&kms_rest_client).await?,
                #[cfg(not(feature = "fips"))]
                CliCommands::Cc(action) => action.process(&kms_rest_client).await?,
                CliCommands::Ec(action) => action.process(&kms_rest_client).await?,
                CliCommands::Rsa(action) => action.process(&kms_rest_client).await?,
                CliCommands::Sym(action) => action.process(&kms_rest_client).await?,
                CliCommands::AccessRights(action) => action.process(&kms_rest_client).await?,
                CliCommands::Certificates(action) => action.process(&kms_rest_client).await?,
                CliCommands::NewDatabase(action) => action.process(&kms_rest_client).await?,
                CliCommands::ServerVersion(action) => action.process(&kms_rest_client).await?,
                CliCommands::Attributes(action) => action.process(&kms_rest_client).await?,
                CliCommands::Google(action) => action.process(&conf_path, &kms_rest_client).await?,
                _ => {
                    tracing::error!("unexpected command");
                }
            }
        }
    }

    Ok(())
}
