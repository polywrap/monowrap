use getopts::Options;

use crate::{LoggerLogLevel, get_logger_log_level_value};

fn get_usage(opts: Options) -> String {
    format!(
        "{}",
        opts.usage("Usage: pwr monowrap.eth COMMAND [options]")
    )
}

fn get_error(opts: Options, error: String) -> String {
    format!("Error: {}\n{}", error, get_usage(opts))
}

#[derive(Clone, Debug)]
pub struct Args {
    pub manifest: String,
    pub commands: Vec<String>,
    pub scope: Vec<String>,
    pub log_level: LoggerLogLevel,
}

pub enum ArgParseResult {
    Args(Args),
    Help(String),
    Error(String),
}

pub fn arg_parse(args: Vec<String>) -> ArgParseResult {
    let mut opts = Options::new();
    opts.optflag("h", "help", "print this help menu");
    opts.optopt("m", "manifest", "path to manifest file", "MANIFEST");
    opts.optmulti("s", "scope", "scope of dependencies for executing command", "SCOPE");
    opts.optopt("l", "log-level", "log level", "LOG_LEVEL");

    let matches = match opts.parse(&args[..]) {
        Ok(m) => m,
        Err(f) => return ArgParseResult::Error(get_error(opts, f.to_string())),
    };

    if matches.opt_present("h") {
        return ArgParseResult::Help(get_usage(opts));
    }

    let commands = matches.free.clone();
    if commands.len() == 0 {
        return ArgParseResult::Error(get_error(opts, "No command specified".to_string()));
    }

    let manifest = match matches.opt_str("m") {
        Some(m) => m,
        None => "./monowrap.json".to_string(),
    };

    let mut scope = matches.opt_strs("s");
    if scope.len() == 0 {
        scope.push("*".to_string())
    }

    let log_level = match matches.opt_str("l") {
        Some(l) => l,
        None => "INFO".to_string(),
    };

    return ArgParseResult::Args(Args {
        commands,
        manifest,
        scope,
        log_level: get_logger_log_level_value(&log_level).unwrap(),
    });
}
