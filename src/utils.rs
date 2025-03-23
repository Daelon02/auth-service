use crate::errors::Result;
use crate::middleware::auth::AuthMiddleware;
use crate::opts::app::AppState;
use crate::opts::cmd_opts::Opts;
use crate::services::actix_requests::requests::{change_password, login, profile, register};
use crate::ApiDoc;
use actix_web::web;
use actix_web::web::{Data, ServiceConfig};
use colored::Colorize;
use config::{Config, File};
use log::{Level, LevelFilter};
use std::collections::HashMap;
use std::path::PathBuf;
use std::str::FromStr;
use std::thread::ThreadId;
use structopt::StructOpt;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

pub fn init_logging() -> Result<()> {
    // Logging lib errors and all app logs
    let log_level = LevelFilter::Debug;

    // This is the main logging dispatch
    let mut main_logging_dispatch = fern::Dispatch::new().level(log_level);

    let stdout_dispatch = fern::Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}][{}::{}] {}",
                chrono::Utc::now().format("[%Y-%m-%d][%H:%M:%S%.3f]"),
                parse_thread_id(&std::thread::current().id()),
                match record.level() {
                    Level::Error => format!("{}", record.level()).red(),
                    Level::Warn => format!("{}", record.level()).red().italic(),
                    Level::Info => format!("{}", record.level()).green(),
                    Level::Debug => format!("{}", record.level()).yellow(),
                    Level::Trace => format!("{}", record.level()).bold(),
                },
                record.target(),
                record
                    .line()
                    .map(|v| v.to_string())
                    .unwrap_or_else(|| "".to_owned()),
                message
            ))
        })
        .chain(std::io::stdout());
    // LevelFilter::from_str()
    main_logging_dispatch = main_logging_dispatch.chain(stdout_dispatch);

    let log_level_for: HashMap<String, String> = HashMap::new();

    for (module, log_level) in log_level_for.into_iter() {
        let log_level = LevelFilter::from_str(&log_level)?;
        main_logging_dispatch = main_logging_dispatch.level_for(module, log_level);
    }

    main_logging_dispatch.apply()?;

    log::info!("Logging level {} enabled", log_level);

    Ok(())
}

fn parse_thread_id(id: &ThreadId) -> String {
    let id_str = format!("{:?}", id);

    let parsed = (|| {
        let start_idx = id_str.find('(')?;
        let end_idx = id_str.rfind(')')?;
        Some(id_str[start_idx + 1..end_idx].to_owned())
    })();

    parsed.unwrap_or(id_str)
}

pub fn configure_routes(cfg: &mut ServiceConfig) {
    let openapi = ApiDoc::openapi();

    cfg.service(
        web::scope("/user")
            .wrap(AuthMiddleware)
            .service(web::resource("/change_password").route(web::post().to(change_password)))
            .service(web::resource("/profile").route(web::get().to(profile))),
    )
    .service(
        web::scope("")
            .service(web::resource("/register").route(web::post().to(register)))
            .service(web::resource("/login").route(web::post().to(login)))
            .service(SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", openapi)),
    );
}
pub fn configure_data(app_state: AppState) -> Box<dyn FnOnce(&mut ServiceConfig)> {
    Box::new(move |cfg: &mut ServiceConfig| {
        cfg.app_data(Data::new(app_state.database))
            .app_data(Data::new(app_state.auth0));
    })
}

#[derive(StructOpt, Debug)]
pub struct ConfigPath {
    pub config: PathBuf,
}

pub fn load_configurations() -> Result<Opts> {
    let config_file = ConfigPath::from_args_safe();
    match config_file {
        Ok(ConfigPath { config }) => {
            let config_data = Config::new().with_merged(File::from(config))?;

            let data: Opts = config_data.try_into()?;
            Ok(data)
        }
        Err(_) => Ok(Opts::from_args()),
    }
}
