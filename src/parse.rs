pub struct Changelog {
    pub title: String,
    pub bulletins: Vec<Bulletin>,
}

pub struct Bulletin {
    pub text: String,
    pub indentation: usize,
}

pub fn parse(contents: &str) -> Changelog {
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

    changelog
}

fn get_indentation(line: &str) -> usize {
    let spaces = line.chars().take_while(|c| *c == ' ').count();
    spaces / 2
}
