use crate::util::{format_number, format_size};
use crate::Detail;
use std::str::FromStr;

#[derive(Debug)]
pub enum Format {
    Table,
    Html,
    Markdown,
}

impl FromStr for Format {
    type Err = ();
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "table" => Ok(Format::Table),
            "html" => Ok(Format::Html),
            "markdown" => Ok(Format::Markdown),
            _ => Err(()),
        }
    }
}

#[derive(Default)]
pub struct Output {
    pub data: Vec<Detail>,
    pub total_code: i32,
    pub total_comment: i32,
    pub total_blank: i32,
    pub total_file: i32,
    pub total_size: u64,
}

impl Output {
    pub fn new(data: Vec<Detail>) -> Self {
        let (total_code, total_comment, total_blank, total_file, total_size) = data
            .iter()
            .map(|detail| {
                (
                    detail.code,
                    detail.comment,
                    detail.blank,
                    detail.file,
                    detail.size,
                )
            })
            .fold((0, 0, 0, 0, 0), |p, n| {
                (p.0 + n.0, p.1 + n.1, p.2 + n.2, p.3 + n.3, p.4 + n.4)
            });

        Self {
            data,
            total_code,
            total_comment,
            total_blank,
            total_file,
            total_size,
        }
    }

    pub fn print(self, format: Format) {
        let mut data = vec![];
        match format {
            Format::Table => self.table(&mut data),
            Format::Html => self.html(&mut data),
            Format::Markdown => self.markdown(&mut data),
        };

        println!("{}", data.join("\n"));
    }

    fn table(&self, data: &mut Vec<String>) {
        data.push(format!("╭{:─<78}╮", ""));
        data.push(format!(
            "│ {:<14}{:>12}{:>12}{:>12}{:>12}{:>14} │",
            "Language", "Code", "Comment", "Blank", "File", "Size"
        ));
        data.push(format!("├{:─<78}┤", ""));

        for item in &self.data {
            data.push(format!(
                "│ {:<14}{:>12}{:>12}{:>12}{:>12}{:>14} │",
                item.language,
                item.code,
                item.comment,
                item.blank,
                item.file,
                format_size(item.size)
            ));
        }

        data.push(format!("├{:─<78}┤", ""));

        data.push(format!(
            "│ {:<14}{:>12}{:>12}{:>12}{:>12}{:>14} │",
            "Total",
            format_number(self.total_code),
            format_number(self.total_comment),
            format_number(self.total_blank),
            format_number(self.total_file),
            format_size(self.total_size)
        ));
        data.push(format!("╰{:─<78}╯", ""));
    }

    fn html(&self, data: &mut Vec<String>) {
        data.push("<table>".to_string());
        data.push(
            "   <thead>
        <tr>
            <th>Language</th>
            <th>Code</th>
            <th>Comment</th>
            <th>Blank</th>
            <th>File</th>
            <th>Size</th>
        </tr>
    </thead>"
                .to_string(),
        );
        data.push("    <tbody>".to_string());

        for item in &self.data {
            data.push(format!(
                "        <tr>
            <td>{}</td>
            <td>{}</td>
            <td>{}</td>
            <td>{}</td>
            <td>{}</td>
            <td>{}</td>
        </tr>",
                item.language,
                item.code,
                item.comment,
                item.blank,
                item.file,
                format_size(item.size)
            ));
        }
        data.push("    </tbody>".to_string());

        data.push(format!(
            "    <tfoot>
        <tr>
            <td>Total</td>
            <td>{}</td>
            <td>{}</td>
            <td>{}</td>
            <td>{}</td>
            <td>{}</td>
        </tr>
    </tfoot>",
            format_number(self.total_code),
            format_number(self.total_comment),
            format_number(self.total_blank),
            format_number(self.total_file),
            format_size(self.total_size)
        ));
        data.push("</table>".to_string());
    }

    fn markdown(&self, data: &mut Vec<String>) {
        data.push(format!(
            "| {:<14} | {:<12} | {:<12} | {:<12} | {:<12} | {:<14} |",
            "Language", "Code", "Comment", "Blank", "File", "Size"
        ));
        data.push(format!(
            "| :{:-<13} | {:-<11}: | {:-<11}: | {:-<11}: | {:-<11}: | {:-<13}: |",
            "", "", "", "", "", ""
        ));
        for item in &self.data {
            data.push(format!(
                "| {:<14} | {:<12} | {:<12} | {:<12} | {:<12} | {:<14} |",
                item.language,
                item.code,
                item.comment,
                item.blank,
                item.file,
                format_size(item.size)
            ));
        }

        data.push(format!(
            "| {:<14} | {:<12} | {:<12} | {:<12} | {:<12} | {:<14} |",
            "Total",
            format_number(self.total_code),
            format_number(self.total_comment),
            format_number(self.total_blank),
            format_number(self.total_file),
            format_size(self.total_size)
        ));
    }
}
