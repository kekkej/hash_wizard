use std::io::{Write, Result as IoResult};
use crate::db::IssuedKey;

pub fn render_journal_table<W: Write>(entries: &[IssuedKey], mut writer: W) -> IoResult<()> {
    let max_user_width = 12;
    let id_width = 4;
    let status_width = 6;
    let created_width = 20;
    let hash_len = 64;

    writeln!(
        writer,
        "{:>width_id$} | {:<width_user$} | {:<status_width$} | {:<width_created$} | {:<hash_len$} | {}",
        "ID",
        "User",
        "Status",
        "Created",
        "Key",
        "Hash",
        width_id = id_width,
        width_user = max_user_width,
        status_width = status_width,
        width_created = created_width,
        hash_len = hash_len,
    )?;

    writeln!(
        writer,
        "{:-<width_id$}-+-{:-<width_user$}-+-{:-<status_width$}-+-{:-<width_created$}-+-{:-<hash_len$}-+",
        "",
        "",
        "",
        "",
        "",
        width_id = id_width,
        width_user = max_user_width,
        status_width = status_width,
        width_created = created_width,
        hash_len = hash_len
    )?;

    for entry in entries {
        let user_lines = split_into_lines(&entry.user, max_user_width);

        let status = if entry.active { "active" } else { "      " };
        let key = &entry.key;
        let hash = &entry.key_hash;
        let created = &entry.created_at;

        let total_lines = user_lines.len().max(1);

        for line_idx in 0..total_lines {
            let user_part = user_lines
                .get(line_idx)
                .map(|x| {x.as_str()})
                .unwrap_or("");

            writeln!(
                writer,
                "{:>width_id$} | {:<width_user$} | {} | {:<width_created$} | {} | {}",
                if line_idx == 0 { entry.id.to_string() } else { "".to_string() },
                user_part,
                if line_idx == 0 { status } else { "" },
                if line_idx == 0 { created } else { "" },
                if line_idx == 0 { key } else { "" },
                if line_idx == 0 { hash } else { "" },
                width_id = id_width,
                width_user = max_user_width,
                width_created = created_width
            )?;
        }
    }

    Ok(())
}

fn split_into_lines(s: &str, max_width: usize) -> Vec<String> {
    let mut lines = Vec::new();
    let mut remaining = s;

    while !remaining.is_empty() {
        let (line, rest) = if remaining.len() > max_width {
            remaining.split_at(max_width)
        } else {
            (remaining, "")
        };
        lines.push(line.to_string());
        remaining = rest;
    }

    lines
}
