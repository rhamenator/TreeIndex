use std::{
    fs, io,
    path::{Path, PathBuf},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Entry {
    pub relative_path: PathBuf,
    pub size: u64,
}

pub fn scan(root: impl AsRef<Path>) -> io::Result<Vec<Entry>> {
    let root = root.as_ref().canonicalize()?;
    let mut entries = Vec::new();
    visit(&root, &root, &mut entries)?;
    entries.sort_by(|left, right| left.relative_path.cmp(&right.relative_path));
    Ok(entries)
}

fn visit(root: &Path, current: &Path, entries: &mut Vec<Entry>) -> io::Result<()> {
    let mut children: Vec<_> = fs::read_dir(current)?.collect::<Result<_, _>>()?;
    children.sort_by_key(|entry| entry.file_name());
    for child in children {
        let file_type = child.file_type()?;
        if file_type.is_symlink() {
            continue;
        }
        if file_type.is_dir() {
            visit(root, &child.path(), entries)?;
        } else if file_type.is_file() {
            entries.push(Entry {
                relative_path: child.path().strip_prefix(root).unwrap().to_path_buf(),
                size: child.metadata()?.len(),
            });
        }
    }
    Ok(())
}

pub fn render_html(title: &str, entries: &[Entry]) -> String {
    let mut output = format!(
        "<!doctype html><html><head><meta charset=\"utf-8\"><title>{}</title></head><body><h1>{}</h1><ul>",
        escape(title),
        escape(title)
    );
    for entry in entries {
        let path = entry.relative_path.to_string_lossy();
        output.push_str(&format!(
            "<li><a href=\"{}\">{}</a> ({} bytes)</li>",
            escape_attribute(&path.replace('\\', "/")),
            escape(&path),
            entry.size
        ));
    }
    output.push_str("</ul></body></html>\n");
    output
}

fn escape(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

fn escape_attribute(value: &str) -> String {
    escape(value).replace('\'', "&#39;")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn html_escapes_names_and_uses_relative_links() {
        let html = render_html(
            "A&B",
            &[Entry {
                relative_path: PathBuf::from("a<b>.txt"),
                size: 3,
            }],
        );
        assert!(html.contains("A&amp;B"));
        assert!(html.contains("a&lt;b&gt;.txt"));
        assert!(!html.contains("E:\\"));
    }
}
