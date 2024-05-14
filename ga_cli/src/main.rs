use std::fs;

use anyhow::Result;
use genetic_framework::Framework;
use inquire::{list_option::ListOption, InquireError, Select};
use log::LevelFilter;
use log4rs::{
    append::file::FileAppender,
    config::{Appender, Root},
    encode::pattern::PatternEncoder,
    Config,
};

fn format_path(path: ListOption<&String>) -> String {
    fs::canonicalize(path.value)
        .expect("Failed to canonicalize path")
        .file_name()
        .expect("Failed to get file name")
        .to_str()
        .expect("Failed to convert to string")
        .to_string()
}

fn config_tracing(problem_name: &str) {
    let file_path = format!(
        "data/outputs/{}-{}.log",
        problem_name,
        chrono::Local::now().format("%Y-%m-%d-%H-%M-%S")
    );

    // Logging to log file.
    let log_file = FileAppender::builder()
        // Pattern: https://docs.rs/log4rs/*/log4rs/encode/pattern/index.html
        .encoder(Box::new(PatternEncoder::new("{l} - {m}\n")))
        .build(file_path)
        .expect("Unable to build file appender");

    // Log Trace level output to file where trace is the default level
    // and the programmatically specified level to stderr.
    let config = Config::builder()
        .appender(Appender::builder().build("log_file", Box::new(log_file)))
        .build(
            Root::builder()
                .appender("log_file")
                .build(LevelFilter::Info),
        )
        .expect("Unable to build log config");

    // Use this to change log levels at runtime.
    // This means you can change the default log level to trace
    // if you are trying to debug an issue and need more logs on then turn it off
    // once you are done.
    log4rs::init_config(config).expect("Unable to start log4rs config");
}

fn main() {
    #[cfg(not(feature = "sequential"))]
    println!("Parallel feature enabled");

    let options: Vec<&str> = vec!["SAT-3", "RADIO", "ALGEBRAIC-FUNCTION", "NQUEENS"];
    let problem_name_answer: Result<&str, InquireError> =
        Select::new("Which problem to run?", options).prompt();
    let problem_name = problem_name_answer.expect("Problem not found");
    config_tracing(problem_name);

    let instances_options: Vec<String> =
        fs::read_dir(format!("data/instances/{}", problem_name.to_lowercase()))
            .expect("Unable to find instances")
            .map(|entry| {
                fs::canonicalize(entry.expect("Unable to retrieve entry").path())
                    .expect("Unable to canonicalize path")
                    .into_os_string()
                    .into_string()
                    .expect("Unable to convert to string")
            })
            .collect();
    let instance_answer: Result<String, InquireError> =
        Select::new("Which instance to run?", instances_options)
            .with_formatter(&format_path)
            .prompt();
    let instance = instance_answer.expect("Instance not found");

    let config_options: Vec<String> = fs::read_dir("data/config")
        .expect("Unable to find config files")
        .map(std::result::Result::unwrap)
        .filter(|entry| {
            entry
                .path()
                .extension()
                .expect("Unable to retrieve file extension")
                == "json"
                && entry
                    .file_name()
                    .into_string()
                    .expect("Unable to convert to string")
                    .starts_with(problem_name.to_lowercase().as_str())
        })
        .map(|entry| {
            fs::canonicalize(entry.path())
                .expect("Unable to canonicalize path")
                .into_os_string()
                .into_string()
                .expect("Unable to convert to string")
        })
        .collect();
    let config_answer: Result<String, InquireError> =
        Select::new("Which config to run?", config_options)
            .with_formatter(&format_path)
            .prompt();
    let config_path = config_answer.expect("Config not found");

    let (problem, config) =
        problem_factory::problem_factory(problem_name, &instance, &config_path);
    let ga_framework = Framework::new(problem, config);
    println!("{:?}", ga_framework.run());
}
