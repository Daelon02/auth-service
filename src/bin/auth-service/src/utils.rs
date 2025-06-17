use crate::errors::Result;
use crate::opts::app::AppState;
use actix_web::web::{Data, ServiceConfig};
use colored::Colorize;
use log::{Level, LevelFilter};
use std::collections::HashMap;
use std::path::PathBuf;
use std::str::FromStr;
use std::thread::ThreadId;

pub fn init_logging() -> Result<()> {
    // Logging src errors and all app logs
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

pub fn configure_data(app_state: AppState) -> Box<dyn FnOnce(&mut ServiceConfig)> {
    Box::new(move |cfg: &mut ServiceConfig| {
        cfg.app_data(Data::new(app_state.database))
            .app_data(Data::new(app_state.auth0));
    })
}

#[derive(Debug)]
pub struct ConfigPath {
    pub config: PathBuf,
}
