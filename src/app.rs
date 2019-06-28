use crate::output::OutputFormat;
use regex::Regex;
use std::env;

macro_rules! exit {
    ($($arg:tt)*) => {
       {
            eprint!("\x1b[91m{}: \x1b[0m", "error");
            eprintln!($($arg)*);
            std::process::exit(1)
       }
    };
}

fn merge_option(a: Option<Vec<String>>, b: Option<Vec<String>>) -> Vec<String> {
    let mut vec = vec![];
    if let Some(v) = a {
        for item in v {
            vec.push(item);
        }
    }
    if let Some(v) = b {
        for item in v {
            vec.push(item);
        }
    }
    vec
}

pub struct App {
    pub args: Vec<String>,
}

impl App {
    pub fn new() -> App {
        let args = env::args().collect();
        App { args }
    }

    pub fn is_help(&self) -> bool {
        self.flag("-h") || self.flag("--help")
    }

    pub fn is_version(&self) -> bool {
        self.flag("-v") || self.flag("--version")
    }

    pub fn ignore(&self) -> Vec<Regex> {
        let vec = merge_option(self.get("-i"), self.get("--ignore"));
        vec.iter()
            .map(|item| match Regex::new(&item) {
                Ok(reg) => reg,
                Err(err) => exit!("{:?}", err),
            })
            .collect()
    }

    pub fn ext(&self) -> Vec<String> {
        merge_option(self.get("-e"), self.get("--ext"))
    }

    pub fn format(&self) -> Result<OutputFormat, ()> {
        let output = merge_option(self.get("-o"), self.get("--output"));
        if let Some(value) = output.get(0) {
            return match value.as_str() {
                "ascii" => Ok(OutputFormat::ASCII),
                "html" => Ok(OutputFormat::HTML),
                "markdown" => Ok(OutputFormat::MarkDown),
                _ => Err(()),
            };
        }
        Ok(OutputFormat::ASCII)
    }

    pub fn print_help(&self) {
        print!(
            r#"{0} version {1}
USAGE:
    {0} [OPTIONS] [FLAGS]
FLAGS:
    -h, --help         Print help information
    -v, --version      Print version number
OPTIONS:
    -e, --ext     <EXT>
    -i, --ignore  <NAME | REGEX> ..
    -o, --output  <ascii | html | markdown>
"#,
            env!("CARGO_PKG_NAME"),
            env!("CARGO_PKG_VERSION")
        );
    }

    pub fn print_version(&self) {
        println!(
            "{} version {}",
            env!("CARGO_PKG_NAME"),
            env!("CARGO_PKG_VERSION")
        );
    }

    fn flag(&self, flag: &str) -> bool {
        self.args.iter().any(|item| item == &flag)
    }

    fn get(&self, arg: &str) -> Option<Vec<String>> {
        let find = self.args.iter().position(|item| item == &arg);
        if let Some(i) = find {
            let mut vec = vec![];
            for arg in &self.args[i + 1..] {
                if arg.chars().next().unwrap_or('.') == '-' {
                    break;
                } else {
                    vec.push(arg.clone());
                }
            }
            return Some(vec);
        }
        None
    }
}
