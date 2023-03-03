// TODO don`t show hidden files by default
// TODO add flags:
// to output [type, size, last modified]
// to customize colours
// sort output differently
// to show hidden
// to show the path and not only the name
// to show idx per entry
// show stats

use clap::{Arg, ArgAction, Command};
use colored::*;
use flexi_logger::{detailed_format, Duplicate, FileSpec, Logger};
use log::error;

use std::{
    env,
    fs::{self, FileType},
    io,
    os::windows::prelude::MetadataExt,
    path::{Path, PathBuf},
    process,
    time::SystemTime,
};

#[derive(Debug)]
struct FileData {
    idx: u32,
    name: String,
    path: String,
    filetype: String,
    hidden: bool,
    read_only: bool,
    modified: u64,
}

impl FileData {
    fn new(
        idx: u32,
        name: String,
        path: String,
        filetype: FileType,
        hidden: bool,
        read_only: bool,
        modified: u64,
    ) -> FileData {
        let mut ftype = String::new();
        match filetype.is_file() {
            true => ftype.push_str("file"),
            false => match filetype.is_dir() {
                true => ftype.push_str("dir"),
                false => ftype.push_str("symlink"),
            },
        }

        FileData {
            idx: idx,
            name: name,
            path: path,
            filetype: ftype,
            hidden: hidden,
            read_only: read_only,
            modified: modified,
        }
    }
}

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
    let hidden_flag = matches.get_flag("hidden");
    let colour_flag = matches.get_flag("colour");
    let fullpath_flag = matches.get_flag("fullpath");
    if let Some(arg) = matches.get_one::<String>("path") {
        let mut path = Path::new(&arg).to_path_buf();

        if arg.is_empty() {
            let current_dir = env::current_dir().unwrap_or_else(|err| {
                error!("Unable to get current directory: {err}");
                process::exit(1);
            });
            path.push(current_dir);
        }

        if let Err(err) = read_dir(path, long_flag, hidden_flag, fullpath_flag, colour_flag) {
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

                if let Err(err) = read_dir(path, long_flag, hidden_flag, fullpath_flag, colour_flag)
                {
                    error!("Error while trying to change the filenames: {}", err);
                    process::exit(1);
                }
            }
        }
    }
}

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
            Arg::new("colour")
                .short('c')
                .long("colour")
                .help("Show coloured output")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("fullpath")
                .short('f')
                .long("fullpath")
                .help("Show the complete pathes instead of just the names")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("hidden")
                .short('H')
                .long("hidden")
                .help("Show hidden files")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("long")
                .short('l')
                .long("long")
                .help("Show more output")
                .long_help("Additionaly display [type, size, last modified]")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("path")
                .short('p')
                .long("path")
                .help("Add a path to a directory")
                .action(ArgAction::Set)
                .num_args(1)
                .value_name("PATH"),
        )
        .subcommand(
            Command::new("log")
                .short_flag('L')
                .about("Show content of the log file"),
        )
}

fn read_dir(
    path: PathBuf,
    long_flag: bool,
    hidden_flag: bool,
    fullpath_flag: bool,
    colour_flag: bool,
) -> io::Result<()> {
    // TODO
    if long_flag {
        unimplemented!();
    } else if hidden_flag {
        if fullpath_flag {
            if colour_flag {
                let dir_entries = store_dir_entries(&path).unwrap();
                for entry in dir_entries {
                    print_output_short(entry.path, true, entry.filetype.as_str());
                }
            } else {
                let dir_entries = store_dir_entries(&path).unwrap();
                for entry in dir_entries {
                    print_output_short(entry.path, false, entry.filetype.as_str());
                }
            }
        } else {
            if colour_flag {
                let dir_entries = store_dir_entries(&path).unwrap();
                for entry in dir_entries {
                    print_output_short(entry.name, true, entry.filetype.as_str());
                }
            } else {
                let dir_entries = store_dir_entries(&path).unwrap();
                for entry in dir_entries {
                    print_output_short(entry.name, false, entry.filetype.as_str());
                }
            }
        }
    } else {
        if fullpath_flag {
            if colour_flag {
                let dir_entries = store_dir_entries(&path).unwrap();
                for entry in dir_entries {
                    if entry.hidden {
                        continue;
                    }
                    print_output_short(entry.path, true, entry.filetype.as_str());
                }
            } else {
                let dir_entries = store_dir_entries(&path).unwrap();
                for entry in dir_entries {
                    if entry.hidden {
                        continue;
                    }
                    print_output_short(entry.path, false, entry.filetype.as_str());
                }
            }
        } else {
            if colour_flag {
                let dir_entries = store_dir_entries(&path).unwrap();
                for entry in dir_entries {
                    if entry.hidden {
                        continue;
                    }
                    print_output_short(entry.name, true, entry.filetype.as_str());
                }
            } else {
                let dir_entries = store_dir_entries(&path).unwrap();
                for entry in dir_entries {
                    if entry.hidden {
                        continue;
                    }
                    print_output_short(entry.name, false, entry.filetype.as_str());
                }
            }
        }

        // for entry in fs::read_dir(path)? {
        //     let entry = entry?;

        //     let name = entry
        //         .path()
        //         .file_name()
        //         .unwrap_or_else(|| {
        //             error!("Unable to get the filename");
        //             process::exit(1);
        //         })
        //         .to_string_lossy()
        //         .to_string();

        //     if entry.path().is_file() {
        //         println!("{}", name);
        //     } else if entry.path().is_dir() {
        //         println!("{}", name.bold());
        //     } else {
        //         println!("{}", name.italic().dimmed());
        //     }
        // }
    }

    Ok(())
}

// TODO replace unwraps
fn store_dir_entries(entry_path: &PathBuf) -> io::Result<(Vec<FileData>)> {
    let mut storage: Vec<FileData> = Vec::new();
    let mut idx = 0;
    for entry in fs::read_dir(entry_path)? {
        let entry = entry?;

        let path = entry.path().as_path().to_string_lossy().to_string();
        let name = entry
            .path()
            .file_name()
            .unwrap_or_else(|| {
                error!("Unable to get the filename");
                process::exit(1);
            })
            .to_string_lossy()
            .to_string();
        let hidden = is_hidden(&entry.path()).unwrap();

        let metadata = fs::metadata(entry.path())?;
        let filetype = metadata.file_type();
        let read_only = metadata.permissions().readonly();
        let modified_systime = metadata.modified().unwrap();
        let diff = SystemTime::now()
            .duration_since(modified_systime)
            .unwrap()
            .as_secs();
        // TODO round
        let modified = diff / 3600;

        let filedata = FileData::new(idx, name, path, filetype, hidden, read_only, modified);
        storage.push(filedata);

        // println!(
        //     "{}: {}, {}, hidden: {}, read_only: {}, modified: {} hours ago",
        //     idx, name, path, hidden, read_only, ago
        // );
        idx += 1;
    }

    Ok(storage)
}

fn print_output_short(name_or_path: String, colour: bool, filetype: &str) {
    if colour {
        match filetype {
            "file" => {
                println!("{}", name_or_path.bright_green())
            }
            "dir" => {
                println!("{}", name_or_path.bold().blue())
            }
            _ => {
                println!("{}", name_or_path.italic().dimmed())
            }
        }
    } else {
        match filetype {
            "file" => {
                println!("{}", name_or_path)
            }
            "dir" => {
                println!("{}", name_or_path.bold())
            }
            _ => {
                println!("{}", name_or_path.italic().dimmed())
            }
        }
    }
}

fn is_hidden(file_path: &PathBuf) -> std::io::Result<bool> {
    let metadata = fs::metadata(file_path)?;
    let attributes = metadata.file_attributes();

    if (attributes & 0x2) > 0 {
        Ok(true)
    } else {
        Ok(false)
    }
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
