use crate::Result;
pub enum OutputFormat {
    ASCII,
    HTML,
    MarkDown,
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

fn bytes_to_size(bytes: f64) -> String {
    let k = 1024_f64;
    let sizes = ["B", "KB", "MB", "GB", "TB", "PB", "EB", "ZB", "YB"];
    if bytes <= 1_f64 {
        return format!("{:.2} B", bytes);
    }
    let i = (bytes.ln() / k.ln()) as i32;
    format!("{:.2} {}", bytes / k.powi(i), sizes[i as usize])
}

pub struct Output {
    data: Vec<Result>,
}

impl Output {
    pub fn new(data: Vec<Result>) -> Output {
        Output { data }
    }

    pub fn ascii(&self) {
        println!("┌{:─<78}┐", "");
        println!(
            "| {:<14}{:>12}{:>12}{:>12}{:>12}{:>14} |",
            "Language", "Code", "Comment", "Blank", "File", "Size"
        );
        println!("├{:─<78}┤", "");
        for item in self.data.clone() {
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
        let total = Total::new(self.data.clone());
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

    pub fn html(&self) {
        println!("<table>");
        println!(
            "
        <thead>
            <tr>
                <th>Language</th>
                <th>Code</th>
                <th>Comment</th>
                <th>Blank</th>
                <th>File</th>
                <th>Size</th>
            </tr>
        </thead>
        "
        );

        println!("<tbody>");
        for item in self.data.clone() {
            println!(
                "
         <tr>
                <td>{}</td>
                <td>{}</td>
                <td>{}</td>
                <td>{}</td>
                <td>{}</td>
                <td>{}</td>
            </tr>
        ",
                item.language,
                item.code,
                item.comment,
                item.blank,
                item.file,
                bytes_to_size(item.size as f64)
            );
        }
        println!("</tbody>");
        let total = Total::new(self.data.clone());
        println!(
            "
        <tfoot>
            <tr>
                <td>{}</td>
                <td>{}</td>
                <td>{}</td>
                <td>{}</td>
                <td>{}</td>
                <td>{}</td>
            </tr>
        </tfoot>
        ",
            "Total",
            total.code(),
            total.comment(),
            total.blank(),
            total.file(),
            bytes_to_size(total.size() as f64)
        );

        println!("</table>");
    }

    pub fn markdown(&self) {
        println!(
            "| {:<14} | {:<12} | {:<12} | {:<12} | {:<12} | {:<14} |",
            "Language", "Code", "Comment", "Blank", "File", "Size"
        );
        println!(
            "| {:-<14} | {:-<12} | {:-<12} | {:-<12} | {:-<12} | {:-<14} |",
            "", "", "", "", "", ""
        );
        for item in self.data.clone() {
            println!(
                "| {:<14} | {:<12} | {:<12} | {:<12} | {:<12} | {:<14} |",
                item.language,
                item.code,
                item.comment,
                item.blank,
                item.file,
                bytes_to_size(item.size as f64)
            );
        }
        let total = Total::new(self.data.clone());
        println!(
            "| {:<14} | {:<12} | {:<12} | {:<12} | {:<12} | {:<14} |",
            "Total",
            total.code(),
            total.comment(),
            total.blank(),
            total.file(),
            bytes_to_size(total.size() as f64)
        );
    }
}
