use regex::Regex;
use std::{fs, path::PathBuf};

#[derive(PartialEq, Eq)]
enum Format {
    Bbcode,
    Html,
}

struct Changelog {
    title: String,
    bulletins: Vec<Bulletin>,
}

struct Bulletin {
    text: String,
    indentation: usize,
}

fn main() {
    let (path, format) = validate_input();
    let contents = fs::read_to_string(path).expect("unable to read the file");

    let lines = contents.split('\n').filter(|line| {
        // trim empty newline at the end
        let trimmed = line.trim();
        !trimmed.is_empty()
    });

    let mut changelog = Changelog {
        title: String::new(),
        bulletins: Vec::new(),
    };

    for (i, line) in lines.enumerate() {
        if i == 0 && !line.starts_with('-') {
            changelog.title = line.replace("*", "");
        } else {
            let indentation = get_indentation(line);
            let trimmed = line.trim().trim_start_matches('-').trim().to_string();
            let bulletin = Bulletin {
                text: trimmed,
                indentation: indentation,
            };

            changelog.bulletins.push(bulletin);
        }
    }

    if format == Format::Html {
        emit_html(changelog);
    } else {
        emit_bbcode(changelog);
    }
}

fn get_indentation(line: &str) -> usize {
    let spaces = line.chars().take_while(|c| *c == ' ').count();
    spaces / 2
}

fn validate_input() -> (PathBuf, Format) {
    let input = std::env::args().nth(1).unwrap_or_else(|| {
        eprintln!("usage: clog <file>");
        std::process::exit(1);
    });

    let path = PathBuf::from(&input);

    if !path.exists() {
        eprintln!("error: path does not exist");
        std::process::exit(1);
    }

    if path.is_dir() {
        eprintln!("error: path must be a file, not a directory");
        std::process::exit(1);
    }

    let format = std::env::args()
        .position(|a| a == "--format") // look for --format
        .and_then(|i| std::env::args().nth(i + 1)) // if we found it, get the value at the next index
        .unwrap_or_else(|| "html".to_string()); // otherwise, default to html

    let format = match format.as_str() {
        "bbcode" => Format::Bbcode,
        "html" => Format::Html,
        _ => {
            eprintln!("error: expected bbcode or html after --format");
            std::process::exit(1);
        }
    };

    (path, format)
}

fn emit_bbcode(changelog: Changelog) {
    let mut has_indented = false;
    let mut output = String::new();
    output.push_str("[list]\n");

    for bulletin in changelog.bulletins {
        if bulletin.indentation > 0 && !has_indented {
            has_indented = true;
        }

        let line = format!("[*] {}\n", bulletin.text);
        let line = replace_inline_formatting(&line, Format::Bbcode);
        output.push_str(&line);
    }

    output.push_str("[/list]\n");

    print!("{}", output);

    if has_indented {
        eprintln!("warning: indented bulletins are not supported in bbcode");
    }
}

fn emit_html(changelog: Changelog) {
    let mut output = String::new();
    output.push_str("<ul>\n");

    let mut indentation = 0;

    for bulletin in changelog.bulletins {
        while bulletin.indentation != indentation {
            let open_tag = bulletin.indentation > indentation;
            let tag = if open_tag { "<ul>" } else { "</ul>" };

            if !open_tag {
                indentation -= 1
            }

            let line = format!("{}{}\n", indent(indentation), tag);
            output.push_str(&line);

            if open_tag {
                indentation += 1
            }
        }

        let line = format!("{}<li>{}</li>\n", indent(indentation), bulletin.text);
        let line = replace_inline_formatting(&line, Format::Html);

        output.push_str(&line);
    }

    // close remaining nested lists
    while indentation > 0 {
        indentation -= 1;
        output.push_str(&format!("{}</ul>\n", indent(indentation)));
    }

    output.push_str("</ul>\n");

    print!("{}", output);
}

fn indent(indentation: usize) -> String {
    " ".repeat((indentation + 1) * 2)
}

fn replace_inline_formatting(text: &str, format: Format) -> String {
    let patterns = [
        (
            r"\*\*\*(.*?)\*\*\*",
            "[b][i]$1[/i][/b]",
            "<strong><em>$1</em></strong>",
        ),
        (r"\*\*(.*?)\*\*", "[b]$1[/b]", "<strong>$1</strong>"),
        (r"\*(.*?)\*", "[i]$1[/i]", "<em>$1</em>"),
    ];

    let mut result = text.to_string();

    for (pattern, steam_replacement, html_replacement) in patterns {
        let re = Regex::new(pattern).unwrap();
        let replacement = match format {
            Format::Bbcode => steam_replacement,
            Format::Html => html_replacement,
        };
        result = re.replace_all(&result, replacement).to_string();
    }

    result
}
