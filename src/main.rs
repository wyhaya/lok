use regex::Regex;
use std::collections::HashMap;
use std::fs;
use std::fs::DirEntry;
use std::path::PathBuf;
#[macro_use]
extern crate lazy_static;
mod output;
use output::{Output, OutputFormat};

use ace::App;

macro_rules! error {
    ($($arg:tt)*) => {
       {
            eprint!("\x1b[91m{}: \x1b[0m", "error");
            eprintln!($($arg)*);
            std::process::exit(1)
       }
    };
}
macro_rules! file_warn {
    ($err: expr, $path: expr) => {
        eprint!("\x1b[93m{}: \x1b[0m", "error");
        eprintln!("{:?} {:?}", $err.kind(), $path);
    };
}

macro_rules! regex {
    ($reg:expr) => {{
        Some(Regex::new($reg).unwrap())
    }};
    ($start:expr, $end:expr) => {{
        Some((Regex::new($start).unwrap(), Regex::new($end).unwrap()))
    }};
}

#[derive(Debug)]
struct Config {
    language: &'static str,
    single: Option<Regex>,
    multi: Option<(Regex, Regex)>,
}

lazy_static! {
    static ref BLANK_REGEX: Regex = Regex::new(r#"^\s*$"#).unwrap();
    static ref CONFIGS: HashMap<&'static str, Config> = {
        let mut hash = HashMap::new();
        let config = vec![
            (
                "rs",
                "Rust",
                regex!(r#"^\s*//"#),
                regex!(r#"/\*"#, r#"\*/"#),
            ),
            (
                "js",
                "JavaScript",
                regex!(r#"^\s*//"#),
                regex!(r#"/\*"#, r#"\*/"#),
            ),
            (
                "ts",
                "TypeScript",
                regex!(r#"^\s*//"#),
                regex!(r#"/\*"#, r#"\*/"#),
            ),
            ("css", "CSS", None, None),
            ("scss", "CSS", None, None),
            ("sass", "CSS", None, None),
            ("less", "CSS", None, None),
            ("html", "HTML", None, None),
            ("jsx", "JavaScript JSX", None, None),
            ("tsx", "TypeScript JSX", None, None),
            ("json", "JSON", None, None),
            ("md", "MarkDown", None, None),
            ("php", "PHP", None, None),
            ("rs", "Rust", None, None),
            ("go", "Go", None, None),
            ("py", "Python", None, None),
            ("sh", "Shell", None, None),
            ("yml", "YML", None, None),
            ("swift", "Swift", None, None),
            ("c", "C", None, None),
            ("coffee", "CoffeeScript", None, None),
            ("dart", "Dart", None, None),
            ("docker", "Docker", None, None),
            ("java", "Java", None, None),
            ("lua", "Lua", None, None),
            ("m", "ObjectiveC", None, None),
            ("aspx", "AspNet", None, None),
            ("makefile", "Makefile", None, None),
            ("sc", "Scala", None, None),
            ("sql", "Sql", None, None),
            ("styl", "Stylus", None, None),
            ("vim", "VimScript", None, None),
            ("xml", "XML", None, None),
            ("toml", "TOML", None, None),
            ("lock", "Lock", None, None),
        ];
        for c in config {
            hash.insert(
                c.0,
                Config {
                    language: c.1,
                    single: c.2,
                    multi: c.3,
                },
            );
        }
        hash
    };
    static ref EXTENSIONS: Vec<&'static str> = CONFIGS.iter().map(|item| *item.0).collect();
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

fn main() {
    let app = App::new(env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"))
        .cmd("help", "Print help information")
        .cmd("version", "Print version information")
        .opt("-e", "Which extension file is used (Example: js rs)")
        .opt("-i", "Ignored file (Rust regex)")
        .opt("-o", "Output format (Optional: ascii, html, markdown)");

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
    let output = app.value("-o");
    let format = if let Some(v) = output {
        if v.len() == 0 {
            OutputFormat::ASCII
        } else {
            match v[0].as_str() {
                "ascii" => OutputFormat::ASCII,
                "html" => OutputFormat::HTML,
                "markdown" => OutputFormat::MarkDown,
                _ => error!("-o value: `ascii` `html` `markdown`"),
            }
        }
    } else {
        OutputFormat::ASCII
    };

    let mut result = vec![];

    let ext = app.value("-e").unwrap_or(vec![]);
    let ignore = app
        .value("-i")
        .unwrap_or(vec![])
        .iter()
        .map(|i| match Regex::new(i) {
            Ok(reg) => reg,
            Err(err) => error!("{:?}", err),
        })
        .collect::<Vec<Regex>>();

    let current_dir = PathBuf::from(".");
    let data = tree(current_dir, &ext, &ignore);

    for r in data {
        let find = result
            .iter()
            .position(|item: &Result| item.language == r.language);
        if let Some(i) = find {
            result[i].comment += r.comment;
            result[i].blank += r.blank;
            result[i].code += r.code;
            result[i].size += r.size;
            result[i].file += 1;
        } else {
            result.push(Result {
                language: r.language,
                comment: r.comment,
                blank: r.blank,
                code: r.code,
                size: r.size,
                file: 1,
            })
        }
    }

    match format {
        OutputFormat::ASCII => Output(result).ascii(),
        OutputFormat::HTML => Output(result).html(),
        OutputFormat::MarkDown => Output(result).markdown(),
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

fn parse(file: &DirEntry) -> Parse {
    let content = fs::read_to_string(file.path()).unwrap();

    let mut blank = 0;
    let mut comment = 0;
    let mut code = 0;
    let mut is_comment = false;

    let p = file.path();

    let ext = p.extension().unwrap().to_str().unwrap();

    let config = CONFIGS.get(ext).unwrap();

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

    let size = file.metadata().unwrap().len();

    Parse {
        language: config.language,
        blank,
        comment,
        code,
        size,
    }
}

fn tree(dir: PathBuf, ext: &Vec<&String>, ignore: &Vec<Regex>) -> Vec<Parse> {
    let read_dir = match fs::read_dir(&dir) {
        Ok(dir) => dir,
        Err(err) => {
            file_warn!(err, &dir);
            return vec![];
        }
    };

    let mut files = vec![];

    for file in read_dir {
        let file = match file {
            Ok(file) => file,
            Err(err) => {
                file_warn!(err, &dir);
                continue;
            }
        };

        let is_dir = match file.metadata() {
            Ok(meta) => meta.is_dir(),
            Err(err) => {
                file_warn!(err, &dir);
                continue;
            }
        };

        if ignore.len() != 0 {
            let file_name = file.file_name();
            match file_name.to_str() {
                Some(name) => {
                    if ignore.iter().any(|item| item.is_match(name)) {
                        continue;
                    }
                }
                None => continue,
            };
        }

        if is_dir {
            files.extend(tree(file.path(), &ext, &ignore));
        } else {
            let p = file.path();
            let e = match p.extension() {
                Some(d) => match d.to_str() {
                    Some(d) => d,
                    None => continue,
                },
                None => continue,
            };

            if ext.len() != 0 {
                if !ext.iter().any(|item| item == &e) {
                    continue;
                }
            }

            let any = EXTENSIONS.iter().any(|item| item == &e);
            if any {
                files.push(parse(&file));
            }
        }
    }
    files
}
