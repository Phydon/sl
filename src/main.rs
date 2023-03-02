use clap::{Arg, ArgAction, Command};
use colored::*;
use flexi_logger::{detailed_format, Duplicate, FileSpec, Logger};
use log::error;

use std::{
    env, fs, io,
    path::{Path, PathBuf},
    process,
};

fn main() {
    // handle Ctrl+C
    ctrlc::set_handler(move || {
        println!(
            "{} {} {}",
            "🤬",
            "Received Ctrl-C! => Exit program!".bold().yellow(),
            "☠",
        );
        process::exit(0)
    })
    .expect("Error setting Ctrl-C handler");

    // get config dir
    let config_dir = check_create_config_dir().unwrap_or_else(|err| {
        error!("Unable to find or create a config directory: {err}");
        process::exit(1);
    });

    // initialize the logger
    let _logger = Logger::try_with_str("info") // log warn and error
        .unwrap()
        .format_for_files(detailed_format) // use timestamp for every log
        .log_to_file(
            FileSpec::default()
                .directory(&config_dir)
                .suppress_timestamp(),
        ) // change directory for logs, no timestamps in the filename
        .append() // use only one logfile
        .duplicate_to_stderr(Duplicate::Info) // print infos, warnings and errors also to the console
        .start()
        .unwrap();

    // handle arguments
    let matches = sl().get_matches();
    let long_flag = matches.get_flag("long");
    if let Some(arg) = matches.get_one::<String>("path") {
        let mut path = Path::new(&arg).to_path_buf();

        if arg.is_empty() {
            let current_dir = env::current_dir().unwrap_or_else(|err| {
                error!("Unable to get current directory: {err}");
                process::exit(1);
            });
            path.push(current_dir);
        }

        if let Err(err) = read_dir(path, long_flag) {
            error!("Error while trying to change the filenames: {}", err);
            process::exit(1);
        }
    } else {
        match matches.subcommand() {
            Some(("log", _)) => {
                if let Ok(logs) = show_log_file(&config_dir) {
                    println!("{}", "Available logs:".bold().yellow());
                    println!("{}", logs);
                } else {
                    error!("Unable to read logs");
                    process::exit(1);
                }
            }
            _ => {
                let current_dir = env::current_dir().unwrap_or_else(|err| {
                    error!("Unable to get current directory: {err}");
                    process::exit(1);
                });

                let path = Path::new(&current_dir).to_path_buf();

                if let Err(err) = read_dir(path, long_flag) {
                    error!("Error while trying to change the filenames: {}", err);
                    process::exit(1);
                }
            }
        }
    }
}

// TODO add flags:
// sort
// customize colours
fn sl() -> Command {
    Command::new("sl")
        .bin_name("sl")
        .before_help(format!(
            "{}\n{}",
            "SIMPLE LS".bold().truecolor(250, 0, 104),
            "Leann Phydon <leann.phydon@gmail.com>".italic().dimmed()
        ))
        .about("Simply list directory entries without any fancy stuff")
        .before_long_help(format!(
            "{}\n{}",
            "SIMPLE LS".bold().truecolor(250, 0, 104),
            "Leann Phydon <leann.phydon@gmail.com>".italic().dimmed()
        ))
        .long_about(format!(
            "{}\n{} {} {} {} {} {}",
            "Simply list directory entries",
            "💥",
            "WITHOUT".strikethrough().yellow(),
            "any".bold().underline().blue(),
            "fancy".italic().purple(),
            "stuff".bright_red().reversed(),
            "✨"
        ))
        .version("1.0.0")
        .author("Leann Phydon <leann.phydon@gmail.com>")
        .arg(
            Arg::new("path")
                .short('p')
                .long("path")
                .help("Add a path to a directory")
                .action(ArgAction::Set)
                .num_args(1)
                .value_name("PATH"),
        )
        .arg(
            Arg::new("long")
                .short('l')
                .long("long")
                .help("Show more")
                .long_help("Just show more output")
                .action(ArgAction::SetTrue),
        )
        .subcommand(
            Command::new("log")
                .short_flag('L')
                .about("Show content of the log file"),
        )
}

// TODO sort output
fn read_dir(path: PathBuf, long_flag: bool) -> io::Result<()> {
    // TODO
    if long_flag {
        unimplemented!();
    } else {
        for entry in fs::read_dir(path)? {
            let entry = entry?;

            let name = entry
                .path()
                .file_name()
                .unwrap_or_else(|| {
                    error!("Unable to get the filename");
                    process::exit(1);
                })
                .to_string_lossy()
                .to_string();

            if entry.path().is_file() {
                println!("{}", name.bright_green());
            } else if entry.path().is_dir() {
                println!("{}", name.blue());
            } else {
                println!("{}", name.dimmed());
            }
        }
    }

    Ok(())
}

fn check_create_config_dir() -> io::Result<PathBuf> {
    let mut new_dir = PathBuf::new();
    match dirs::config_dir() {
        Some(config_dir) => {
            new_dir.push(config_dir);
            new_dir.push("sl");
            if !new_dir.as_path().exists() {
                fs::create_dir(&new_dir)?;
            }
        }
        None => {
            error!("Unable to find config directory");
        }
    }

    Ok(new_dir)
}

fn show_log_file(config_dir: &PathBuf) -> io::Result<String> {
    let log_path = Path::new(&config_dir).join("sl.log");
    match log_path.try_exists()? {
        true => {
            return Ok(format!(
                "{} {}\n{}",
                "Log location:".italic().dimmed(),
                &log_path.display(),
                fs::read_to_string(&log_path)?
            ));
        }
        false => {
            return Ok(format!(
                "{} {}",
                "No log file found:".red().bold().to_string(),
                log_path.display()
            ))
        }
    }
}
