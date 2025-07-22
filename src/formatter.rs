use clap::ValueEnum;
use crate::db::IssuedKey;

#[derive(Clone, Debug, ValueEnum)]
pub enum Formatter {
    Console,
    Csv,
    Markdown,
}

impl Formatter {
    pub fn render(&self, keys: &[IssuedKey], short: bool) -> String {
        match self {
            Formatter::Console => render_as_table(keys, short),
            Formatter::Csv => render_as_csv(keys, short),
            Formatter::Markdown => render_as_markdown(keys, short),
        }
    }
}

fn render_as_table(keys: &[IssuedKey], short: bool) -> String {
    let mut out = String::new();
    if short {
        out.push_str("ID | User | Key | Hash  | Created At\n");
        out.push_str("---|------|-----|------|------------\n");
        for key_entry in keys {
            out.push_str(&format!(
                "{} | {} | {} | {} | {}\n",
                key_entry.id, key_entry.user, key_entry.key, key_entry.key_hash, key_entry.created_at
            ));
        }
    } else {
        out.push_str("ID | User | Key | Hash | Active |  Reason  | Created At\n");
        out.push_str("---|------|-----|------|--------|----------|------------\n");
        for key_entry in keys {
            out.push_str(&format!(
                "{} | {} | {} | {} | {} | {} | {}\n",
                key_entry.id, key_entry.user, key_entry.key, key_entry.key_hash, key_entry.active, key_entry.reason, key_entry.created_at
            ));
        }
    }
    out
}

fn render_as_csv(keys: &[IssuedKey], short: bool) -> String {
    let mut out: String = String::new();
    if short {
        out.push_str("id,user, key, key_hash,created_at\n");
        for key_entry in keys {
            out.push_str(&format!(
                "{},{},{},{},{}\n",
                key_entry.id, key_entry.user, key_entry.key, key_entry.key_hash, key_entry.created_at
            ));
        }
    } else {
        out.push_str("id,user, key, key_hash,active, reason, created_at\n");
        for key_entry in keys {
            out.push_str(&format!(
                "{},{},{},{},{},{},{}\n",
                key_entry.id,
                key_entry.user, 
                key_entry.key, 
                key_entry.key_hash, 
                key_entry.active, 
                key_entry.reason, 
                key_entry.created_at
            ));
        }
    }
    
    out
}

fn render_as_markdown(keys: &[IssuedKey], short: bool) -> String {
    let mut out = String::new();
    if short {
        out.push_str("| ID | User | Key | Hash | Created At |\n|----|------|------|--------|------------|\n");
        for key_entry in keys {
            out.push_str(&format!(
                "| {} | {} | {} | {} | {} |\n",
                key_entry.id, key_entry.user, key_entry.key, key_entry.key_hash,  key_entry.created_at
            ));
        }
    } else {
        out.push_str("| ID | User | Key | Hash | Active| Reason | Created At |\n|----|------|------|--------|----------|------------|\n");
        for key_entry in keys {
            out.push_str(&format!(
                "| {} | {} | {} | {} | {} | {} | {} |\n",
                key_entry.id, 
                key_entry.user, 
                key_entry.key, 
                key_entry.key_hash,
                key_entry.active,
                key_entry.reason, 
                key_entry.created_at
            ));
        }
    }
    out
}