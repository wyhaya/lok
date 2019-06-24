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

pub struct App {
    pub args: Vec<String>,
}

impl App {
    pub fn new() -> App {
        let args = env::args().collect();
        App { args }
    }

    pub fn is_help(&self) -> bool {
        self.get_arg_flag("--help")
    }

    pub fn is_version(&self) -> bool {
        self.get_arg_flag("--version")
    }

    pub fn ignore(&self) -> Option<Vec<Regex>> {
        if let Some(v) = self.get_arg_option("--ignore") {
            Some(
                v.iter()
                    .map(|item| match Regex::new(&item) {
                        Ok(reg) => reg,
                        Err(err) => exit!("{:?}", err),
                    })
                    .collect(),
            )
        } else {
            None
        }
    }

    pub fn ext(&self) -> Option<Vec<String>> {
        self.get_arg_option("--ext")
    }

    pub fn print_help(&self) {
        print!(
            r#"{0} version {1}
USAGE:
    {0} [OPTIONS] [FLAGS] ...
FLAGS:
    --help         Print help information
    --version      Print version number
OPTIONS:
    --ext     <EXT>  ...    ...
    --ignore  <NAME> ...    ...
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

    fn get_arg_flag(&self, flag: &str) -> bool {
        self.args.iter().any(|item| item == &flag)
    }

    fn get_arg_option(&self, arg: &str) -> Option<Vec<String>> {
        const ALLOW_ARG: &[&str] = &["--ignore", "--ext", "--help", "--version"];
        let find = self.args.iter().position(|item| item == &arg);
        if let Some(i) = find {
            let mut vec = vec![];
            for arg in &self.args[i + 1..] {
                if ALLOW_ARG.iter().any(|item| item == arg) {
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
