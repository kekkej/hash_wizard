use clap::ValueEnum;
use crate::db::IssuedKey;

#[derive(Clone, Debug, ValueEnum)]
pub enum Formatter {
    Console,
    Csv,
    Markdown,
}

impl Formatter {
    pub fn render(&self, keys: &[IssuedKey]) -> String {
        match self {
            Formatter::Console => render_as_table(keys),
            Formatter::Csv => render_as_csv(keys),
            Formatter::Markdown => render_as_markdown(keys),
        }
    }
}

fn render_as_table(keys: &[IssuedKey]) -> String {
    let mut out = String::new();
    out.push_str("ID | User | Hash | Active | Created At\n");
    out.push_str("---|------|------|--------|------------\n");
    for key_entry in keys {
        out.push_str(&format!(
            "{} | {} | {} | {} | {} | {}\n",
            key_entry.id, key_entry.user, key_entry.key, key_entry.key_hash, key_entry.active, key_entry.created_at
        ));
    }
    out
}

fn render_as_csv(keys: &[IssuedKey]) -> String {
    let mut out = String::from("id,user, key, key_hash,active,created_at\n");
    for key_entry in keys {
        out.push_str(&format!(
            "{},{},{},{},{},{}\n",
            key_entry.id, key_entry.user, key_entry.key, key_entry.key_hash, key_entry.active, key_entry.created_at
        ));
    }
    out
}

fn render_as_markdown(keys: &[IssuedKey]) -> String {
    let mut out = String::from("| ID | User | Key | Hash | Active | Created At |\n|----|------|------|--------|------------|\n");
    for key_entry in keys {
        out.push_str(&format!(
            "| {} | {} | {} | {} | {} | {} |\n",
            key_entry.id, key_entry.user, key_entry.key, key_entry.key_hash, key_entry.active, key_entry.created_at
        ));
    }
    out
}