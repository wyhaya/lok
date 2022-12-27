#[derive(Debug)]
pub struct Config(&'static [Language]);

#[derive(Debug)]
pub struct Language {
    pub name: &'static str,
    pub extensions: &'static [&'static str],
    pub single: &'static [&'static str],
    pub multi: &'static [(&'static str, &'static str)],
}

macro_rules! language {
    ($name: expr, $ext: expr, $single: expr, $multi: expr) => {
        Language {
            name: $name,
            extensions: $ext,
            single: $single,
            multi: $multi,
        }
    };
}

impl Config {
    pub fn languages(&self) -> &'static [Language] {
        self.0
    }

    // Get language configuration by extension
    pub fn get(&self, extension: &str) -> Option<&Language> {
        self.0
            .iter()
            .find(|&item| item.extensions.contains(&extension))
    }
}

pub const CONFIG: Config = Config(&[
    language!(
        "ASP.NET",
        &["asax", "ascx", "asmx", "aspx", "master", "sitemap", "webinfo"],
        &[],
        &[("<!--", "-->"), ("<%--", "-->")]
    ),
    language!("C", &["c"], &["//"], &[("/*", "*/")]),
    language!(
        "CSS",
        &["css", "scss", "sass", "less"],
        &["//"],
        &[("/*", "*/")]
    ),
    language!("C++", &["cpp"], &["//"], &[("/*", "*/")]),
    language!("CoffeeScript", &["coffee"], &["#"], &[("###", "###")]),
    language!("C#", &["cs"], &["//", "///"], &[("/*", "*/")]),
    language!("D", &["d"], &["//", "///"], &[("/*", "*/")]),
    language!("Dart", &["dart"], &["//", "///"], &[("/*", "*/")]),
    language!("Go", &["go"], &["//"], &[("/*", "*/")]),
    language!("HTML", &["htm", "html"], &[], &[("<!--", "-->")]),
    language!("Haskell", &["hs"], &["--"], &[("{-", "-}")]),
    language!("JavaScript", &["js", "mjs"], &["//"], &[("/*", "*/")]),
    language!("JavaScript JSX", &["jsx"], &["//"], &[("/*", "*/")]),
    language!("JSON", &["json"], &[], &[]),
    language!("Julia", &["jl"], &["#"], &[("#=", "=#")]),
    language!("Java", &["java"], &["//"], &[("/*", "*/")]),
    language!(
        "Common Lisp",
        &["lisp", "lsp", "l", "cl", "fasl"],
        &[";", ";;", ";;;", ";;;;"],
        &["#|", "|#"]
    ),
    language!("LLVM", &["ll"], &[","], &[]),
    language!("Lua", &["lua"], &["--"], &[("--[[", "]]")]),
    language!("Markdown", &["md", "markdown"], &[], &[]),
    language!("Nim", &["nim"], &["#"], &[("＃[", "]#")]),
    language!("ObjectiveC", &["m"], &["//", "///"], &[("/*", "*/")]),
    language!("Objective-C++", &["mm"], &["//"], &[("/*", "*/")]),
    language!("PHP", &["php"], &["//", "#"], &[("/*", "*/")]),
    language!(
        "Python",
        &["py"],
        &["#"],
        &[("'''", "'''"), (r#"""""#, r#"""""#)]
    ),
    language!("Perl", &["pl", "pm"], &["#"], &[("=", "=")]),
    language!("R", &["r"], &["#"], &[]),
    language!("Rust", &["rs"], &["//", "///"], &[("/*", "*/")]),
    language!("Ruby", &["rb"], &["#"], &[("=", "=")]),
    language!("Swift", &["swift"], &["//"], &[("/*", "*/")]),
    language!("Scala", &["sc"], &["//"], &[("/*", "*/")]),
    language!(
        "Scheme",
        &["scm", "ss"],
        &[";", ";;", ";;;", ";;;;", "#;"],
        &[("#|", "|#")]
    ),
    language!("Shell", &["sh", "bash", "zsh", "fish"], &["#"], &[]),
    language!("SQL", &["sql"], &["--"], &[("/*", "*/")]),
    language!("TypeScript", &["ts"], &["//"], &[("/*", "*/")]),
    language!("TypeScript JSX", &["tsx"], &["//"], &[("/*", "*/")]),
    language!("TOML", &["toml"], &["#"], &[]),
    // This file may contain multiple languages. html.. js ts .. css scss sass..
    // Not processed here
    language!("Vue", &["vue"], &["//"], &[("<!--", "-->"), ("/*", "*/")]),
    language!("VimScript", &["vim"], &[], &[]),
    language!("XML", &["xml"], &[], &[("<!--", "-->")]),
    language!("YAML", &["yml", "yaml"], &["#"], &[]),
]);
