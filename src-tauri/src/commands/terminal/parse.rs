#[derive(PartialEq)]
pub enum EscapeSequence {
    None,
    Start,
    InEscape,
    InOperatingSystemEscape,
    LineStart,
}

static ESCAPE_COMMANDS: &[char] = &['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'b', 'm'];

pub struct OutputParser {
    pub cleaned_lines: Vec<String>,
    pub cleaned_line: String,
    pub escape: EscapeSequence,
    pub current_escape_arg: String,
    pub escape_args: Vec<String>,
}

impl OutputParser {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn handle_escape_command(&mut self, escape_command: char) {
        if escape_command == 'H' {
            if let Some(first_arg) = self.escape_args.first() {
                if let Ok(row) = first_arg.parse::<usize>() {
                    if row > self.cleaned_lines.len() {
                        self.cleaned_lines.push(self.cleaned_line.clone());
                        self.cleaned_line.clear();
                        // -1 because the new cleaned_line will be added at
                        // the end as the next line
                        self.cleaned_lines.resize(row - 1, "".to_string());
                    }
                }
            }
        }
    }
}

impl Default for OutputParser {
    fn default() -> Self {
        OutputParser {
            cleaned_lines: Vec::<String>::new(),
            cleaned_line: String::new(),
            escape: EscapeSequence::None,
            current_escape_arg: String::new(),
            escape_args: Vec::<String>::new(),
        }
    }
}

pub fn clean_output(output: &str) -> String {
    let mut parser = OutputParser::new();

    output.chars().for_each(|c| {
        if c == '\u{001B}' {
            parser.escape = EscapeSequence::Start;
        } else if parser.escape == EscapeSequence::Start {
            if c == '[' || c == '(' {
                parser.escape = EscapeSequence::InEscape;
            } else if c == ']' {
                parser.escape = EscapeSequence::InOperatingSystemEscape;
            } else {
                parser.escape = EscapeSequence::None;
                parser.cleaned_line.push(c);
            }
        } else if parser.escape == EscapeSequence::InEscape
            || parser.escape == EscapeSequence::InOperatingSystemEscape
        {
            let mut escape_command: Option<char> = None;
            if c == '?' {
                // it's just a private sequence marker, do nothing
            } else if parser.escape == EscapeSequence::InEscape
                && ESCAPE_COMMANDS.contains(&c)
            {
                escape_command = Some(c);
                parser.escape = EscapeSequence::None;
            } else if c == ';' {
                parser.escape_args.push(parser.current_escape_arg.clone());
                parser.current_escape_arg.clear();
            } else if c == '\u{0007}' {
                escape_command = parser.current_escape_arg.pop();
                parser.escape_args.push(parser.current_escape_arg.clone());
                parser.escape = EscapeSequence::None;
            } else {
                parser.current_escape_arg.push(c);
            }

            if parser.escape == EscapeSequence::None {
                parser.escape_args.push(parser.current_escape_arg.clone());
                parser.current_escape_arg.clear();

                if let Some(escape_command) = escape_command {
                    parser.handle_escape_command(escape_command);
                }

                parser.escape_args.clear();
            }
        } else if c == '\r' {
            parser.escape = EscapeSequence::LineStart;
        } else if parser.escape == EscapeSequence::LineStart {
            if c == '\n' {
                parser.escape = EscapeSequence::None;
                parser.cleaned_lines.push(parser.cleaned_line.clone());
                parser.cleaned_line.clear();
            } else {
                parser.escape = EscapeSequence::None;
                parser.cleaned_line.clear();
                parser.cleaned_line.push(c);
            }
        } else if c == '\n' {
            parser.escape = EscapeSequence::None;
            parser.cleaned_lines.push(parser.cleaned_line.clone());
            parser.cleaned_line.clear();
        } else {
            parser.cleaned_line.push(c);
        }
    });

    parser.cleaned_lines.push(parser.cleaned_line);
    parser.cleaned_lines.join("\n").trim_start().to_string()
}
