#[macro_use]
extern crate lazy_static;

mod config;
mod output;

use ace::App;
use config::{Config, Language};
use deque::{Stealer, Stolen};
use output::{Output, Print};
use regex::Regex;
use std::fs;
use std::io::ErrorKind;
use std::path::PathBuf;
use std::thread;

macro_rules! exit {
    ($($arg:tt)*) => {
       {
            eprint!("\x1b[91m{}: \x1b[0m", "error");
            eprintln!($($arg)*);
            std::process::exit(1)
       }
    };
}

macro_rules! warn {
    ($kind: expr, $path: expr) => {
        eprint!("\x1b[93m{}: \x1b[0m", "error");
        eprintln!("{:?} {:?}", $kind, $path);
    };
}

lazy_static! {
    static ref BLANK_REGEX: Regex = Regex::new(r#"^\s*$"#).unwrap();
    static ref CONFIGS: Config = config::new();
}

fn main() {
    let app = App::new(env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"))
        .cmd("help", "Print help information")
        .cmd("version", "Print version information")
        .opt("-e", "Which extension file is used (example: js rs)")
        .opt("-i", "Ignored file (rust regex)")
        .opt("-o", "Output format (optional: ascii, html, markdown)")
        .opt("-p", "Set working directory")
        .opt(
            "-s",
            "Sort by (optional: language, code, comment, blank, file, size)",
        );

    if let Some(cmd) = app.command() {
        match cmd.as_str() {
            "help" => {
                app.help();
            }
            "version" => {
                app.version();
            }
            _ => {
                app.error_try("help");
            }
        }
        return;
    }

    let e = app.value("-e").unwrap_or(vec![]);

    let i = match app.value("-i") {
        Some(values) => {
            let val = values
                .iter()
                .map(|val| format!("({})", &val))
                .collect::<Vec<String>>()
                .join("|");

            match Regex::new(&val) {
                Ok(reg) => Some(reg),
                Err(err) => exit!("{:?}", err),
            }
        }
        None => None,
    };

    let p = match app.value("-p") {
        Some(p) => match p.len() {
            0 => PathBuf::from("."),
            _ => PathBuf::from(p[0]),
        },
        None => PathBuf::from("."),
    };

    let o = match app.value("-o") {
        Some(values) => {
            if values.len() == 0 {
                Output::ASCII
            } else {
                match values[0].to_lowercase().as_str() {
                    "ascii" => Output::ASCII,
                    "html" => Output::HTML,
                    "markdown" => Output::MarkDown,
                    _ => exit!("-o value: `ascii` `html` `markdown`"),
                }
            }
        }
        None => Output::ASCII,
    };

    let s = match app.value("-s") {
        Some(values) => {
            if values.len() == 0 {
                Sort::Language
            } else {
                match values[0].to_lowercase().as_str() {
                    "language" => Sort::Language,
                    "code" => Sort::Code,
                    "comment" => Sort::Comment,
                    "blank" => Sort::Blank,
                    "file" => Sort::File,
                    "size" => Sort::Size,
                    _ => exit!("-s value: `language`, `code` `comment` `blank` `file` `size`"),
                }
            }
        }
        None => Sort::Language,
    };

    let (work, stealer) = deque::new();
    let mut workers = vec![];

    for _ in 0..num_cpus::get() {
        let worker = Worker {
            data: stealer.clone(),
        };
        workers.push(thread::spawn(|| worker.run()));
    }

    tree(p,&e, &i, &work);

    for _ in 0..workers.len() {
        work.push(Work::Quit);
    }

    let mut result = vec![];

    for worker in workers {
        for d in worker.join().unwrap() {
            let find = result
                .iter()
                .position(|item: &Result| item.language == d.language);

            if let Some(i) = find {
                result[i].comment += d.comment;
                result[i].blank += d.blank;
                result[i].code += d.code;
                result[i].size += d.size;
                result[i].file += 1;
            } else {
                result.push(Result {
                    language: d.language,
                    comment: d.comment,
                    blank: d.blank,
                    code: d.code,
                    size: d.size,
                    file: 1,
                });
            }
        }
    }

    let data = match s {
        Sort::Code => sort(result, |a, b| a.code > b.code),
        Sort::Comment => sort(result, |a, b| a.comment > b.comment),
        Sort::Blank => sort(result, |a, b| a.blank > b.blank),
        Sort::File => sort(result, |a, b| a.file > b.file),
        Sort::Size => sort(result, |a, b| a.size > b.size),
        _ => sort(result, |a, b| position(a.language) > position(b.language)),
    };

    match o {
        Output::ASCII => Print(data).ascii(),
        Output::HTML => Print(data).html(),
        Output::MarkDown => Print(data).markdown(),
    };
}

fn sort<T>(mut vec: Vec<T>, call: fn(&T, &T) -> bool) -> Vec<T> {
    for x in 0..vec.len() {
        for y in x..vec.len() {
            if call(&vec[x], &vec[y]) {
                vec.swap(x, y);
            }
        }
    }
    vec
}

const LETTER: &'static str = "aAbBcCdDeEfFgGhHiIjJkKlLmMnNoOpPqQrRsStTuUvVwWxXyYzZ";
fn position(s: &str) -> usize {
    if let Some(c) = s.chars().next() {
        let index = LETTER.chars().position(|d| d == c);
        return match index {
            Some(i) => i,
            None => 0,
        };
    }
    0
}

fn tree(dir: PathBuf, ext: &Vec<&String>, ignore: &Option<Regex>, work: &deque::Worker<Work>) {
    let read_dir = match fs::read_dir(&dir) {
        Ok(dir) => dir,
        Err(err) => {
            warn!(err.kind(), &dir);
            return;
        }
    };

    for file in read_dir {
        let file = match file {
            Ok(file) => file,
            Err(err) => {
                warn!(err.kind(), &dir);
                continue;
            }
        };

        let meta = match file.metadata() {
            Ok(meta) => meta,
            Err(err) => {
                warn!(err.kind(), &dir);
                continue;
            }
        };

        if let Some(ignore) = ignore {
            match file.file_name().to_str() {
                Some(name) => {
                    if ignore.is_match(name) {
                        continue;
                    }
                }
                None => continue,
            };
        }
        let path = file.path();

        if meta.is_dir() {
            tree(path, &ext, &ignore, &work);
            continue;
        }

        let extension = match path.extension() {
            Some(d) => match d.to_str() {
                Some(d) => d,
                None => continue,
            },
            None => continue,
        };

        if ext.len() != 0 {
            if !ext.iter().any(|item| item == &extension) {
                continue;
            }
        }

        if let Some(config) = CONFIGS.get(extension) {
            work.push(Work::File(path, meta.len(), config));
        }
    }
}

#[derive(Debug, Clone)]
pub struct Result {
    language: &'static str,
    blank: i32,
    comment: i32,
    code: i32,
    size: u64,
    file: i32,
}

#[derive(Debug)]
enum Sort {
    Language,
    Code,
    Comment,
    Blank,
    File,
    Size,
}

enum Work<'a> {
    File(PathBuf, u64, &'a Language),
    Quit,
}

struct Worker<'a> {
    data: Stealer<Work<'a>>,
}

impl<'a> Worker<'a> {
    fn run(self) -> Vec<Parse> {
        let mut vec = vec![];
        loop {
            match self.data.steal() {
                Stolen::Empty | Stolen::Abort => continue,
                Stolen::Data(Work::Quit) => break,
                Stolen::Data(Work::File(path, size, config)) => {
                    match Parse::new(path, size, &config) {
                        Ok(d) => vec.push(d),
                        Err((kind, p)) => {
                            warn!(kind, p);
                        }
                    };
                }
            }
        }
        vec
    }
}

#[derive(Debug, Clone)]
struct Parse {
    language: &'static str,
    blank: i32,
    comment: i32,
    code: i32,
    size: u64,
}

impl Parse {
    fn new(
        path: PathBuf,
        size: u64,
        config: &Language,
    ) -> std::result::Result<Parse, (ErrorKind, PathBuf)> {
        let content = match fs::read_to_string(&path) {
            Ok(data) => data,
            Err(err) => return Err((err.kind(), path)),
        };

        let mut blank = 0;
        let mut comment = 0;
        let mut code = 0;
        let mut is_comment = false;

        for line in content.split("\n") {
            if BLANK_REGEX.is_match(&line) {
                blank += 1;
                continue;
            }

            if let Some((before, after)) = &config.multi {
                if before.is_match(line) {
                    is_comment = true;
                }
                if after.is_match(line) {
                    is_comment = false;
                    comment += 1;
                    continue;
                }
                if is_comment {
                    comment += 1;
                    continue;
                }
            }

            if let Some(single) = &config.single {
                if single.is_match(line) {
                    comment += 1;
                    continue;
                }
            }

            code += 1;
        }

        Ok(Parse {
            language: config.name,
            blank,
            comment,
            code,
            size,
        })
    }
}
