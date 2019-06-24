use regex::Regex;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::fs::DirEntry;
use std::path::Path;
#[macro_use]
extern crate lazy_static;
#[macro_use]
mod app;
use app::App;

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
        let configs = vec![
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
        for c in configs {
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
struct Result {
    language: &'static str,
    blank: i32,
    comment: i32,
    code: i32,
    size: u64,
    file: i32,
}

fn main() {
    let app = App::new();

    if app.is_help() {
        return app.print_help();
    }

    if app.is_version() {
        return app.print_version();
    }

    let t = std::time::Instant::now();

    let mut result = vec![];

    let current_dir = match env::current_dir() {
        Ok(dir) => dir,
        Err(err) => exit!("{:?}", err),
    };

    let tree = tree(current_dir, &app.ext(), &app.ignore());

    for t in tree {
        let r = parse(&t);
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

    dbg!(t.elapsed());
    output(result);
}

macro_rules! total {
    ($name: ident, $type: path) => {
        fn $name(&self) -> $type {
            let mut n = 0;
            for item in &self.result {
                n += item.$name
            }
            n
        }
    };
}

struct Total {
    result: Vec<Result>,
}
impl Total {
    fn new(result: Vec<Result>) -> Total {
        Total { result }
    }
    total!(code, i32);
    total!(comment, i32);
    total!(blank, i32);
    total!(file, i32);
    total!(size, u64);
}

fn output(result: Vec<Result>) {
    println!("┌{:─<78}┐", "");
    println!(
        "| {:<14}{:>12}{:>12}{:>12}{:>12}{:>14} |",
        "Language", "Code", "Comment", "Blank", "File", "Size"
    );
    println!("├{:─<78}┤", "");
    for item in result.clone() {
        println!(
            "| {:<14}{:>12}{:>12}{:>12}{:>12}{:>14} |",
            item.language,
            item.code,
            item.comment,
            item.blank,
            item.file,
            bytes_to_size(item.size as f64)
        );
    }
    println!("├{:─<78}┤", "");
    let total = Total::new(result.clone());
    println!(
        "| {:<14}{:>12}{:>12}{:>12}{:>12}{:>14} |",
        "Total",
        total.code(),
        total.comment(),
        total.blank(),
        total.file(),
        bytes_to_size(total.size() as f64)
    );
    println!("└{:─<78}┘", "");
}

fn bytes_to_size(bytes: f64) -> String {
    let k = 1024_f64;
    let sizes = ["B", "KB", "MB", "GB", "TB", "PB", "EB", "ZB", "YB"];
    if bytes <= 1_f64 {
        return format!("{:.2} B", bytes);
    }
    let i = (bytes.ln() / k.ln()) as i32;
    format!("{:.2} {}", bytes / k.powi(i), sizes[i as usize])
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

fn tree<P: AsRef<Path>>(
    dir: P,
    ext: &Option<Vec<String>>,
    ignore: &Option<Vec<Regex>>,
) -> Vec<DirEntry> {
    let read_dir = match fs::read_dir(dir) {
        Ok(dir) => dir,
        Err(err) => exit!("{:?}", err),
    };

    let mut files = vec![];

    for file in read_dir {
        let file = match file {
            Ok(file) => file,
            Err(err) => exit!("{:?}", err),
        };

        let is_dir = match file.metadata() {
            Ok(meta) => meta.is_dir(),
            Err(err) => exit!("{:?}", err),
        };

        if let Some(ignore) = ignore {
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
            for f in tree(file.path(), &ext, &ignore) {
                files.push(f);
            }
        } else {
            let p = file.path();
            let e = match p.extension() {
                Some(d) => match d.to_str() {
                    Some(d) => d,
                    None => continue,
                },
                None => continue,
            };
            if let Some(ext) = ext {
                if !ext.iter().any(|item| item == &e) {
                    continue;
                }
            }
            let any = EXTENSIONS.iter().any(|item| item == &e);
            if any {
                files.push(file);
            }
        }
    }
    files
}
