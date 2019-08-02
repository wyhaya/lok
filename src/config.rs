use crate::HashMap;
use crate::Regex;

macro_rules! regex {
    ($reg:expr) => {{
        Some(Regex::new($reg).unwrap())
    }};
    ($start:expr, $end:expr) => {{
        Some((Regex::new($start).unwrap(), Regex::new($end).unwrap()))
    }};
}

#[derive(Debug)]
pub struct Config {
    pub language: &'static str,
    pub single: Option<Regex>,
    pub multi: Option<(Regex, Regex)>,
}

pub fn new() -> HashMap<&'static str, Config> {
    let mut config = HashMap::new();

    macro_rules! language {
        ($ext: expr, $language: expr, $single: expr, $multi: expr) => {
            config.insert(
                $ext,
                Config {
                    language: $language,
                    single: $single,
                    multi: $multi,
                },
            );
        };
    }

    language!(
        "rs",
        "Rust",
        regex!(r#"^\s*//"#),
        regex!(r#"/\*"#, r#"\*/"#)
    );
    language!(
        "js",
        "JavaScript",
        regex!(r#"^\s*//"#),
        regex!(r#"/\*"#, r#"\*/"#)
    );
    language!(
        "ts",
        "TypeScript",
        regex!(r#"^\s*//"#),
        regex!(r#"/\*"#, r#"\*/"#)
    );
    language!("css", "CSS", None, None);
    language!("scss", "CSS", None, None);
    language!("sass", "CSS", None, None);
    language!("less", "CSS", None, None);
    language!("html", "HTML", None, None);
    language!("jsx", "JavaScript JSX", None, None);
    language!("tsx", "TypeScript JSX", None, None);
    language!("json", "JSON", None, None);
    language!("md", "MarkDown", None, None);
    language!("php", "PHP", None, None);
    language!("rs", "Rust", None, None);
    language!("go", "Go", None, None);
    language!("py", "Python", None, None);
    language!("sh", "Shell", None, None);
    language!("yml", "YML", None, None);
    language!("swift", "Swift", None, None);
    language!("c", "C", None, None);
    language!("coffee", "CoffeeScript", None, None);
    language!("dart", "Dart", None, None);
    language!("java", "Java", None, None);
    language!("lua", "Lua", None, None);
    language!("m", "ObjectiveC", None, None);
    language!("aspx", "AspNet", None, None);
    language!("sc", "Scala", None, None);
    language!("sql", "Sql", None, None);
    language!("styl", "Stylus", None, None);
    language!("vim", "VimScript", None, None);
    language!("xml", "XML", None, None);
    language!("toml", "TOML", None, None);
    language!("vue", "Vue", None, None);

    config
}

pub fn blank() -> Regex {
    Regex::new(r#"^\s*$"#).unwrap()
}
