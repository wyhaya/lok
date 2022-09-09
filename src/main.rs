mod cli;
mod config;
mod output;
mod parse;
mod util;
mod workder;

use cli::{Args, Sort};
use config::CONFIG;
use output::Output;
use parse::{parser, Data, Value};
use walkdir::WalkDir;
use workder::Tasks;

fn main() {
    let Args {
        work_dir,
        print_error,
        exclude,
        include,
        format,
        sort,
        extension,
    } = cli::parse();

    let files = WalkDir::new(work_dir).into_iter().filter_map(|item| {
        let entry = match item {
            Ok(entry) => entry,
            Err(error) => {
                if print_error {
                    if let (Some(err), Some(path)) = (error.io_error(), error.path()) {
                        print_error!(err.kind(), path);
                    }
                }
                return None;
            }
        };

        let path = entry.path();

        // Include files
        if let Some(include) = &include {
            let any = include.iter().any(|m| m.matches_path(path));
            if !any {
                return None;
            }
        }

        // Exclude files
        if let Some(exclude) = &exclude {
            for matcher in exclude {
                if matcher.matches_path(path) {
                    return None;
                }
            }
        }

        // File with the specified extension
        let ext = match path.extension() {
            Some(s) => match s.to_str() {
                Some(ext) => ext,
                None => return None,
            },
            None => return None,
        };

        // This extension is not included in config
        if let Some(extension) = &extension {
            if !extension.iter().any(|s| s == ext) {
                return None;
            }
        }

        // Get file path and configuration
        CONFIG
            .get(ext)
            .map(|config| (entry.path().to_path_buf(), config))
    });

    let tasks = Tasks::new();

    for (path, config) in files {
        tasks.push(move || {
            match parser(path, config) {
                Value::Ok(data) => return Some(data),
                Value::Err(kind, p) => {
                    if print_error {
                        print_error!(kind, p)
                    }
                }
                Value::Invalid => {}
            };
            None
        });
    }

    // Summary of all data
    let mut total = Vec::new();
    for rst in tasks.result() {
        let datas = rst.unwrap_or_else(|err| {
            exit!("Thread exits abnormally\n{:#?}", err);
        });
        for data in datas {
            let find = total
                .iter_mut()
                .find(|item: &&mut Detail| item.language == data.language);

            match find {
                Some(detail) => detail.add(data),
                None => total.push(Detail::from(data)),
            }
        }
    }

    total.sort_by(|a, b| match sort {
        Sort::Language => a.language.cmp(b.language),
        Sort::Code => a.code.cmp(&b.code),
        Sort::Comment => a.comment.cmp(&b.comment),
        Sort::Blank => a.blank.cmp(&b.blank),
        Sort::File => a.file.cmp(&b.file),
        Sort::Size => a.size.cmp(&b.size),
    });

    Output::new(total).print(format);
}

#[derive(Debug)]
pub struct Detail {
    language: &'static str,
    blank: i32,
    comment: i32,
    code: i32,
    size: u64,
    file: i32,
}

impl From<Data> for Detail {
    fn from(data: Data) -> Self {
        Detail {
            language: data.language,
            blank: data.blank,
            comment: data.comment,
            code: data.code,
            size: data.size,
            file: 1,
        }
    }
}

impl Detail {
    fn add(&mut self, data: Data) {
        self.comment += data.comment;
        self.blank += data.blank;
        self.code += data.code;
        self.size += data.size;
        self.file += 1;
    }
}
