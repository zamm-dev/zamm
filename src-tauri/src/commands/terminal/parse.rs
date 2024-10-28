#[derive(PartialEq)]
enum EscapeSequence {
    None,
    Start,
    InEscape,
    InOperatingSystemEscape,
    LineStart,
}

static ESCAPE_COMMANDS: &[char] = &['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'b', 'm'];

pub fn clean_output(output: &str) -> String {
    let mut cleaned_lines = Vec::<String>::new();
    output.split('\n').for_each(|line| {
        let mut escape: EscapeSequence = EscapeSequence::None;
        let mut cleaned_line = String::new();
        let mut current_escape_arg = String::new();
        let mut escape_args = Vec::<String>::new();
        let mut escape_command = ' ';
        line.chars().for_each(|c| {
            if c == '\u{001B}' {
                escape = EscapeSequence::Start;
            } else if escape == EscapeSequence::Start {
                if c == '[' || c == '(' {
                    escape = EscapeSequence::InEscape;
                } else if c == ']' {
                    escape = EscapeSequence::InOperatingSystemEscape;
                } else {
                    escape = EscapeSequence::None;
                    cleaned_line.push(c);
                }
            } else if escape == EscapeSequence::InEscape
                || escape == EscapeSequence::InOperatingSystemEscape
            {
                if c == '?' {
                    // it's just a private sequence marker, do nothing
                } else if escape == EscapeSequence::InEscape
                    && ESCAPE_COMMANDS.contains(&c)
                {
                    escape_command = c;
                    escape_args.push(current_escape_arg.clone());
                    current_escape_arg.clear();
                    escape = EscapeSequence::None;
                } else if c == ';' {
                    escape_args.push(current_escape_arg.clone());
                    current_escape_arg.clear();
                } else if c == '\u{0007}' {
                    if let Some(last_char) = current_escape_arg.pop() {
                        escape_command = last_char;
                    }
                    escape_args.push(current_escape_arg.clone());
                    current_escape_arg.clear();
                    escape = EscapeSequence::None;
                } else {
                    current_escape_arg.push(c);
                }

                if escape == EscapeSequence::None {
                    escape_args.push(current_escape_arg.clone());
                    current_escape_arg.clear();

                    // now we actually handle the escape sequence
                    if escape_command == 'H' {
                        if let Some(first_arg) = escape_args.first() {
                            if let Ok(row) = first_arg.parse::<usize>() {
                                if row > cleaned_lines.len() {
                                    cleaned_lines.push(cleaned_line.clone());
                                    cleaned_line.clear();
                                    // -1 because the new cleaned_line will be added at
                                    // the end as the next line
                                    cleaned_lines.resize(row - 1, "".to_string());
                                }
                            }
                        }
                    }
                }
            } else if c == '\r' {
                escape = EscapeSequence::LineStart;
            } else if escape == EscapeSequence::LineStart {
                escape = EscapeSequence::None;
                cleaned_line.clear();
                cleaned_line.push(c);
            } else {
                cleaned_line.push(c);
            }
        });
        cleaned_lines.push(cleaned_line);
    });
    cleaned_lines.join("\n").trim_start().to_string()
}
