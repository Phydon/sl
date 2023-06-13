use clap::{Arg, ArgAction, Command};
use colored::*;
use flexi_logger::{detailed_format, Duplicate, FileSpec, Logger};
use log::error;

use std::{
    env,
    fs::{self, FileType, Permissions},
    io,
    os::windows::prelude::MetadataExt,
    path::{Path, PathBuf},
    process,
    time::SystemTime,
};

const KB: u64 = 1024;
const MB: u64 = 1024_u64.pow(2);
const GB: u64 = 1024_u64.pow(3);
const TB: u64 = 1024_u64.pow(4);

// red
const EXECUTABLE: &[&'static str] = &["exe", "msi", "bat"];
// yellow
const SPECIAL: &[&'static str] = &[
    "md", "cgf", "conf", "config", "ini", "json", "tml", "toml", "yaml", "yml", "csv", "markdown",
    "org", "rst", "xml",
];
// green
const PROGRAMMING: &[&'static str] = &[
    "py", "pl", "rs", "c", "cpp", "awk", "vb", "cabal", "clj", "cs", "csx", "css", "h", "hpp",
    "dart", "ex", "exs", "elc", "elm", "erl", "fs", "go", "hs", "ipynb", "java", "bsh", "js", "jl",
    "kt", "tex", "lisp", "lua", "matlab", "pas", "p", "php", "ps1", "r", "rb", "scala", "sh",
    "bash", "zsh", "fish", "sql", "swift", "ts", "tsx", "vim", "cmake", "make",
];
// pink
const MEDIA: &[&'static str] = &[
    "bmp", "gif", "jpeg", "jpg", "png", "svg", "avi", "mp4", "wmv", "wma", "mp3", "wav", "mid",
    "ttf",
];
// red
const OFFICE: &[&'static str] = &[
    "doc", "docx", "epub", "odt", "pdf", "ps", "xls", "xlsx", "ods", "xlr", "ppt", "pptx", "odp",
    "pps", "ics",
];
// cyan
const ARCHIVES: &[&'static str] = &[
    "apk", "deb", "rpm", "xbps", "bag", "bin", "dmg", "img", "iso", "toast", "vcd", "7z", "arj",
    "gz", "zip", "pkg", "tar", "jar", "rar", "tgz", "z", "zst", "xz", "tgz",
];
// darkgray
const OTHER: &[&'static str] = &["~", ".git", ".gitignore", "tmp", "lock"];

// COLOURS
// -------
// OLD
// green (filesize) = 59, 179, 140
// red (readonly) = 250, 0, 104
// blue (dirs) = 109, 144, 217
// exec = 102, 255, 179
// normal (file) = 191, 179, 255

// ICEBERG-DARK
//   background_color: '161821'     22,24,33
//   white: 'c6c8d1'                198,200,209
//   red: 'e27878'                  226,120,120
//   green: 'b4be82'                180,190,130
//   yellow: 'e2a478'               226,164,120
//   blue: '84a0c6'                 132,160,198
//   pink: 'ada0d3'                 173,160,211
//   cyan: '89b8c2'                 137,184,194
//   black: '1e2132'                30,33,50
//   gray: '828597'                 130,133,151
//   darkgray: '6b7089'             107,112,137
//   darkergray: '36384a'           54,56,74

struct Perms {
    read: String,
    write: String,
}

struct FileData {
    name: String,
    path: String,
    filetype: String,
    filesize: String,
    hidden: bool,
    modified: String,
    permissions: Perms,
    file_extension: String,
}

impl FileData {
    fn new(
        name: String,
        path: String,
        filetype: FileType,
        filesize: u64,
        hidden: bool,
        modified: u64,
        permissions: Permissions,
        file_extension: String,
    ) -> FileData {
        let mut ftype = String::new();
        match filetype.is_file() {
            true => {
                ftype.push_str("file");
            }
            false => {
                match filetype.is_dir() {
                    true => ftype.push_str("dir"),
                    false => ftype.push_str("symlink"),
                };
            }
        }

        let mut fsize = String::new();
        if filesize <= 0 {
            fsize.push_str("-");
        } else {
            match filesize {
                s if s > TB => {
                    let size = ((filesize as f64 / TB as f64) * 10.0).round() / 10.0;
                    fsize.push_str(&size.to_string());
                    fsize.push_str("T");
                }
                s if s > GB && s < TB => {
                    let size = ((filesize as f64 / GB as f64) * 10.0).round() / 10.0;
                    fsize.push_str(&size.to_string());
                    fsize.push_str("G");
                }
                s if s > MB && s < GB => {
                    let size = ((filesize as f64 / MB as f64) * 10.0).round() / 10.0;
                    fsize.push_str(&size.to_string());
                    fsize.push_str("M");
                }
                s if s > KB && s < MB => {
                    let size = ((filesize as f64 / KB as f64) * 10.0).round() / 10.0;
                    fsize.push_str(&size.to_string());
                    fsize.push_str("K");
                }
                s if s < KB => {
                    fsize.push_str(&filesize.to_string());
                }
                _ => fsize.push_str("-"),
            }
        }

        let mut modified_human_readable = String::new();
        match modified {
            0..=59 => {
                modified_human_readable.push_str(modified.to_string().as_str());
                modified_human_readable.push_str(" secs ago");
            }
            60..=3599 => {
                let minutes = ((modified as f64 / 60.0) as f64).round();
                modified_human_readable.push_str(minutes.to_string().as_str());
                modified_human_readable.push_str(" mins ago");
            }
            3600..=86399 => {
                let hours = ((modified as f64 / 3600.0) as f64).round();
                modified_human_readable.push_str(hours.to_string().as_str());
                modified_human_readable.push_str("  hrs ago");
            }
            86400.. => {
                let days = ((modified as f64 / 86400.0) as f64).round();
                modified_human_readable.push_str(days.to_string().as_str());
                modified_human_readable.push_str(" days ago");
            }
        }

        let mut perms = Perms {
            read: String::new(),
            write: String::new(),
        };
        match permissions.readonly() {
            true => {
                perms.read = String::from("r");
                perms.write = String::from("-");
            }
            false => {
                perms.read = String::from("r");
                perms.write = String::from("w");
            }
        }

        FileData {
            name: name,
            path: path,
            filetype: ftype,
            filesize: fsize,
            hidden: hidden,
            modified: modified_human_readable,
            permissions: perms,
            file_extension: file_extension,
        }
    }
}

fn main() {
    // handle Ctrl+C
    ctrlc::set_handler(move || {
        println!(
            "{} {} {} {}",
            "Received Ctrl-C!".bold().red(),
            "🤬",
            "Exit program!".bold().red(),
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
    let files_flag = matches.get_flag("files");
    let dirs_flag = matches.get_flag("dirs");

    if let Some(arg) = matches.get_one::<String>("path") {
        let mut path = Path::new(&arg).to_path_buf();

        if arg.is_empty() {
            let current_dir = env::current_dir().unwrap_or_else(|err| {
                error!("Unable to get current directory: {err}");
                process::exit(1);
            });
            path.push(current_dir);
        }

        if let Err(err) = list_dirs(
            path,
            long_flag,
            hidden_flag,
            fullpath_flag,
            colour_flag,
            files_flag,
            dirs_flag,
        ) {
            error!("Unable to get the entries of the directory: {}", err);
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

                if let Err(err) = list_dirs(
                    path,
                    long_flag,
                    hidden_flag,
                    fullpath_flag,
                    colour_flag,
                    files_flag,
                    dirs_flag,
                ) {
                    error!("Unable to get the entries of the directory: {}", err);
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
        // TODO update version
        .version("1.1.0")
        .author("Leann Phydon <leann.phydon@gmail.com>")
        .arg(
            Arg::new("colour")
                .short('c')
                .long("colour")
                .visible_alias("color")
                .help("Show coloured output")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("dirs")
                .short('d')
                .long("dirs")
                .visible_alias("dir")
                .help("Show only dirs")
                .action(ArgAction::SetTrue)
                .conflicts_with("files"),
        )
        .arg(
            Arg::new("files")
                .short('f')
                .long("files")
                .visible_alias("file")
                .help("Show only files")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("fullpath")
                .short('F')
                .long("fullpath")
                .help("Show the complete path instead of just the name")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("hidden")
                .short('H')
                .long("hidden")
                .visible_alias("all")
                .help("Show hidden files")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("long")
                .short('l')
                .long("long")
                .help("Show more output")
                .long_help("Additionaly display [type, size, last modified, read_only]")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("path")
                .help("Add a path to a directory")
                .action(ArgAction::Set)
                .num_args(1)
                .value_name("PATH"),
        )
        .subcommand(
            Command::new("log")
                .short_flag('L')
                .long_flag("log")
                .about("Show content of the log file"),
        )
}

fn list_dirs(
    path: PathBuf,
    long_flag: bool,
    hidden_flag: bool,
    fullpath_flag: bool,
    colour_flag: bool,
    files_flag: bool,
    dirs_flag: bool,
) -> io::Result<()> {
    let dir_entries = store_dir_entries(&path)?;

    match long_flag {
        true => {
            for entry in dir_entries {
                if entry.hidden && !hidden_flag {
                    continue;
                }

                if files_flag && !entry.filetype.as_str().contains("file") {
                    continue;
                }

                if dirs_flag && !entry.filetype.as_str().contains("dir") {
                    continue;
                }

                let name_or_path = if fullpath_flag {
                    entry.path
                } else {
                    entry.name
                };

                print_output_long(
                    name_or_path,
                    entry.filetype.as_str(),
                    entry.filesize.as_str(),
                    colour_flag,
                    entry.modified,
                    entry.permissions,
                    entry.file_extension,
                );
            }
        }
        false => {
            for entry in dir_entries {
                if entry.hidden && !hidden_flag {
                    continue;
                }

                if files_flag && !entry.filetype.as_str().contains("file") {
                    continue;
                }

                if dirs_flag && !entry.filetype.as_str().contains("dir") {
                    continue;
                }

                let name_or_path = if fullpath_flag {
                    entry.path
                } else {
                    entry.name
                };

                print_output_short(
                    name_or_path,
                    entry.filetype.as_str(),
                    entry.file_extension,
                    colour_flag,
                );
            }
        }
    }

    Ok(())
}

fn store_dir_entries(entry_path: &PathBuf) -> io::Result<Vec<FileData>> {
    let mut storage: Vec<FileData> = Vec::new();
    for entry in fs::read_dir(entry_path)? {
        let entry = entry?;

        let path = entry.path().as_path().to_string_lossy().to_string();
        let name = entry
            .path()
            .file_name()
            .unwrap_or_else(|| {
                error!("Unable to get the filename of {path}");
                process::exit(1);
            })
            .to_string_lossy()
            .to_string();
        let hidden = is_hidden(&entry.path()).unwrap_or_else(|err| {
            error!("Unable to tell if file {path} is hidden: {err}");
            process::exit(1);
        });

        let mut file_extension = String::new();
        if let Some(extension) = entry.path().extension() {
            file_extension.push_str(&extension.to_string_lossy().to_string());
        }

        let metadata = fs::metadata(entry.path())?;
        let filetype = metadata.file_type();
        let filesize = metadata.file_size();
        let permissions = metadata.permissions();
        let modified_systime = metadata.modified()?;
        let diff = SystemTime::now()
            .duration_since(modified_systime)
            .unwrap_or_else(|err| {
                error!("Unable to get duration since the system is running: {err}");
                process::exit(1);
            })
            .as_secs();
        let modified = diff;

        let filedata = FileData::new(
            name,
            path,
            filetype,
            filesize,
            hidden,
            modified,
            permissions,
            file_extension,
        );
        storage.push(filedata);
    }

    Ok(storage)
}

fn print_output_short(name_or_path: String, filetype: &str, file_extension: String, colour: bool) {
    if colour {
        match filetype {
            "file" => {
                let mut name = String::new();
                if EXECUTABLE.iter().any(|it| &file_extension == it) {
                    let cstr = format!("{}", name_or_path.bold().truecolor(226, 120, 120));
                    name.push_str(&cstr);
                } else if SPECIAL.iter().any(|it| &file_extension == it) {
                    let cstr = format!(
                        "{}",
                        name_or_path
                            .on_truecolor(226, 164, 120)
                            .truecolor(22, 24, 33)
                    );
                    name.push_str(&cstr);
                } else if PROGRAMMING.iter().any(|it| &file_extension == it) {
                    let cstr = format!("{}", name_or_path.truecolor(180, 190, 130));
                    name.push_str(&cstr);
                } else if OFFICE.iter().any(|it| &file_extension == it) {
                    let cstr = format!("{}", name_or_path.truecolor(226, 120, 120));
                    name.push_str(&cstr);
                } else if OTHER.iter().any(|it| &file_extension == it) {
                    let cstr = format!("{}", name_or_path.truecolor(107, 112, 137));
                    name.push_str(&cstr);
                } else if MEDIA.iter().any(|it| &file_extension == it) {
                    let cstr = format!("{}", name_or_path.truecolor(173, 160, 211));
                    name.push_str(&cstr);
                } else if ARCHIVES.iter().any(|it| &file_extension == it) {
                    let cstr = format!("{}", name_or_path.truecolor(137, 184, 194));
                    name.push_str(&cstr);
                } else {
                    let cstr = format!("{}", name_or_path.truecolor(198, 200, 209));
                    name.push_str(&cstr);
                }
                println!("{}", name)
            }
            "dir" => {
                println!("{}", name_or_path.truecolor(109, 144, 217).bold())
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

fn print_output_long(
    name_or_path: String,
    filetype: &str,
    filesize: &str,
    colour: bool,
    modified: String,
    permissions: Perms,
    file_extension: String,
) {
    let mut ftype = String::new();
    let mut name = String::new();
    match filetype {
        "file" => {
            ftype.push_str(".");
            if colour {
                if EXECUTABLE.iter().any(|it| &file_extension == it) {
                    let cstr = format!("{}", name_or_path.bold().truecolor(226, 120, 120));
                    name.push_str(&cstr);
                } else if SPECIAL.iter().any(|it| &file_extension == it) {
                    let cstr = format!("{}", name_or_path.truecolor(226, 164, 120))
                        .on_truecolor(22, 24, 33);
                    name.push_str(&cstr);
                } else if PROGRAMMING.iter().any(|it| &file_extension == it) {
                    let cstr = format!("{}", name_or_path.truecolor(180, 190, 130));
                    name.push_str(&cstr);
                } else if OFFICE.iter().any(|it| &file_extension == it) {
                    let cstr = format!("{}", name_or_path.truecolor(226, 120, 120));
                    name.push_str(&cstr);
                } else if OTHER.iter().any(|it| &file_extension == it) {
                    let cstr = format!("{}", name_or_path.truecolor(107, 112, 137));
                    name.push_str(&cstr);
                } else if MEDIA.iter().any(|it| &file_extension == it) {
                    let cstr = format!("{}", name_or_path.truecolor(173, 160, 211));
                    name.push_str(&cstr);
                } else if ARCHIVES.iter().any(|it| &file_extension == it) {
                    let cstr = format!("{}", name_or_path.truecolor(137, 184, 194));
                    name.push_str(&cstr);
                } else {
                    let cstr = format!("{}", name_or_path.truecolor(198, 200, 209));
                    name.push_str(&cstr);
                }
            } else {
                name.push_str(&name_or_path);
            }
        }
        "dir" => {
            ftype.push_str("d");
            if colour {
                let cstr = format!("{}", name_or_path.truecolor(109, 144, 217).bold());
                name.push_str(&cstr);
            } else {
                let cstr = format!("{}", name_or_path.bold());
                name.push_str(&cstr);
            }
        }
        _ => {
            ftype.push_str("s");
            let cstr = format!("{}", name_or_path.italic().dimmed());
            name.push_str(&cstr);
        }
    }

    let mut perm_read = String::new();
    let mut perm_write = String::new();
    if colour && permissions.write.contains("-") {
        perm_read.push_str(&format!("{}", permissions.read.truecolor(250, 0, 104)));
        perm_write.push_str(&format!("{}", permissions.write.truecolor(250, 0, 104)));
    } else {
        perm_read.push_str(&permissions.read);
        perm_write.push_str(&permissions.write);
    }

    let fsize = if colour {
        filesize.truecolor(59, 179, 140)
    } else {
        filesize.white()
    };

    println!(
        "{}{}{}\t{:>7}  {:>14}  {}",
        ftype, perm_read, perm_write, fsize, modified, name,
    );
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
