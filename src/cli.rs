use crate::output::Format;
use crate::util::to_glob_pattern;
use crate::CONFIG;
use clap::{crate_name, crate_version, value_t_or_exit, App, AppSettings, Arg, SubCommand};
use glob::Pattern;
use std::path::PathBuf;

pub struct Args {
    pub work_dir: PathBuf,
    pub print_error: bool,
    pub exclude: Option<Vec<Pattern>>,
    pub include: Option<Vec<Pattern>>,
    pub format: Format,
    pub sort: Sort,
    pub extension: Option<Vec<String>>,
}

#[derive(Debug)]
pub enum Sort {
    Language,
    Code,
    Comment,
    Blank,
    File,
    Size,
}

impl std::str::FromStr for Sort {
    type Err = ();
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "language" => Ok(Sort::Language),
            "code" => Ok(Sort::Code),
            "comment" => Ok(Sort::Comment),
            "blank" => Ok(Sort::Blank),
            "file" => Ok(Sort::File),
            "size" => Ok(Sort::Size),
            _ => Err(()),
        }
    }
}

pub fn parse() -> Args {
    let app = App::new(crate_name!())
        .version(crate_version!())
        .global_setting(AppSettings::ColoredHelp)
        .setting(AppSettings::VersionlessSubcommands)
        .subcommand(SubCommand::with_name("ls").about("Print a list of supported languages"))
        .arg(Arg::with_name("directory").help("Calculate the specified directory"))
        .arg(
            Arg::with_name("error")
                .long("error")
                .help("Print error message"),
        )
        .arg(
            Arg::with_name("exclude")
                .short("e")
                .long("exclude")
                .value_name("GLOB")
                .multiple(true)
                .help("Exclude files using 'glob' matching"),
        )
        .arg(
            Arg::with_name("include")
                .short("i")
                .long("include")
                .value_name("GLOB")
                .multiple(true)
                .help("Include files using 'glob' matching"),
        )
        .arg(
            Arg::with_name("output")
                .short("o")
                .long("output")
                .value_name("OUTPUT")
                .possible_values(&["table", "html", "markdown"])
                .default_value("table")
                .max_values(1)
                .hide_default_value(true)
                .help("Specify output format"),
        )
        .arg(
            Arg::with_name("sort")
                .short("s")
                .long("sort")
                .value_name("SORT")
                .possible_values(&["language", "code", "comment", "blank", "file", "size"])
                .default_value("language")
                .max_values(1)
                .hide_default_value(true)
                .help("Specify the column sort by"),
        )
        .arg(
            Arg::with_name("extension")
                .long("extension")
                .multiple(true)
                .value_name("EXTENSION")
                .display_order(1000)
                .help("Parse file with specified extension"),
        )
        .get_matches();

    if app.is_present("ls") {
        print_language_list();
        std::process::exit(0)
    }

    let dir = app.value_of("directory").unwrap_or(".");
    let work_dir = PathBuf::from(dir);

    // Whether the output is wrong
    let print_error = app.is_present("error");

    let exclude = app
        .values_of("exclude")
        .map(|values| to_glob_pattern(&work_dir, values.collect()));

    let include = app
        .values_of("include")
        .map(|values| to_glob_pattern(&work_dir, values.collect()));

    let format = value_t_or_exit!(app, "output", Format);

    let sort = value_t_or_exit!(app, "sort", Sort);

    let extension = app
        .values_of("extension")
        .map(|values| values.map(|s| s.to_string()).collect::<Vec<String>>());

    Args {
        work_dir,
        print_error,
        exclude,
        include,
        format,
        sort,
        extension,
    }
}

fn print_language_list() {
    let n = CONFIG
        .languages()
        .iter()
        .map(|language| language.name.len())
        .fold(0, |a, b| a.max(b));

    for language in CONFIG.languages() {
        let ext = language
            .extension
            .iter()
            .map(|e| format!(".{}", e))
            .collect::<Vec<String>>()
            .join(" ");
        println!("{:name$}    {}", language.name, ext, name = n);
    }
}
