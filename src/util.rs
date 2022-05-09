use glob::Pattern;
use std::path::Path;
use std::thread::available_parallelism;

#[macro_export]
macro_rules! exit {
    ($($arg:tt)*) => {
       {
            use bright::Colorful;
            eprint!("{} ", "error:".red().bold());
            eprintln!($($arg)*);
            std::process::exit(1)
       }
    };
}

#[macro_export]
macro_rules! print_error {
    ($kind: expr, $path: expr) => {{
        use bright::Colorful;
        eprintln!("{} {:?} {:?}", "error:".yellow(), $kind, $path);
    }};
}

// Translate to the same path
// ./src src => ./src ./src
// /src  src => /src   /src
// src   src => src    src
pub fn to_glob_pattern(path: &Path, values: Vec<&str>) -> Vec<Pattern> {
    values
        .iter()
        .map(|s| {
            if path.starts_with(".") && !s.starts_with("./") {
                format!("./{}", s)
            } else if path.starts_with("/") && !s.starts_with('/') {
                format!("/{}", s)
            } else {
                (*s).to_string()
            }
        })
        .map(|s| {
            Pattern::new(s.as_str())
                .unwrap_or_else(|err| exit!("Cannot parse '{}' to glob matcher\n{:#?}", s, err))
        })
        .collect::<Vec<Pattern>>()
}

pub fn format_size(n: u64) -> String {
    const UNITS: [char; 6] = ['K', 'M', 'G', 'T', 'P', 'E'];
    if n < 1024 {
        return format!("{} B ", n);
    }
    let bytes = n as f64;
    let i = (bytes.ln() / 1024_f64.ln()) as i32;
    format!(
        "{:.2} {}B",
        bytes / 1024_f64.powi(i),
        UNITS[(i - 1) as usize]
    )
}

pub fn format_number<S: ToString>(num: S) -> String {
    let num = num.to_string();
    let mut rst = String::new();
    for (i, ch) in num.chars().enumerate() {
        rst.push(ch);
        if i != num.len() - 1 && (num.len() - 1 - i) % 3 == 0 {
            rst.push(',');
        }
    }
    rst
}

pub fn num_cpus() -> usize {
    match available_parallelism() {
        Ok(n) => n.get(),
        Err(_) => 1,
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_format_size() {
        assert_eq!(format_size(0), "0 B ");
        assert_eq!(format_size(1), "1 B ");
        assert_eq!(format_size(1023), "1023 B ");
        assert_eq!(format_size(1024), "1.00 KB");
        assert_eq!(format_size(1 * 1024 * 1024), "1.00 MB");
        assert_eq!(format_size(1 * 1024 * 1024 * 1024 * 1024), "1.00 TB");
        assert_eq!(format_size(u64::max_value()), "16.00 EB");
    }

    #[test]
    fn test_format_number() {
        assert_eq!(format_number(0), "0");
        assert_eq!(format_number(1), "1");
        assert_eq!(format_number(1000), "1,000");
        assert_eq!(format_number(999999), "999,999");
        assert_eq!(format_number(1234567), "1,234,567");
    }
}
