use crate::parse::Changelog;
use regex::Regex;

#[derive(PartialEq, Eq)]
pub enum Format {
    Bbcode,
    Html,
}

pub fn emit(changelog: Changelog, format: Format) {
    match format {
        Format::Html => emit_html(changelog),
        Format::Bbcode => emit_bbcode(changelog),
    }
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
