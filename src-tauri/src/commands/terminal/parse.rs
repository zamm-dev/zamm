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
    pub state: EscapeSequence,
    pub cleaned_lines: Vec<String>,
    pub current_line: String,
    pub escape_args: Vec<String>,
    pub current_arg: String,
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
                        self.cleaned_lines.push(self.current_line.clone());
                        self.current_line.clear();
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
            state: EscapeSequence::None,
            cleaned_lines: Vec::<String>::new(),
            current_line: String::new(),
            escape_args: Vec::<String>::new(),
            current_arg: String::new(),
        }
    }
}

pub fn clean_output(output: &str) -> String {
    let mut parser = OutputParser::new();

    output.chars().for_each(|c| {
        if c == '\u{001B}' {
            parser.state = EscapeSequence::Start;
        } else if parser.state == EscapeSequence::Start {
            if c == '[' || c == '(' {
                parser.state = EscapeSequence::InEscape;
            } else if c == ']' {
                parser.state = EscapeSequence::InOperatingSystemEscape;
            } else {
                parser.state = EscapeSequence::None;
                parser.current_line.push(c);
            }
        } else if parser.state == EscapeSequence::InEscape
            || parser.state == EscapeSequence::InOperatingSystemEscape
        {
            let mut escape_command: Option<char> = None;
            if c == '?' {
                // it's just a private sequence marker, do nothing
            } else if parser.state == EscapeSequence::InEscape
                && ESCAPE_COMMANDS.contains(&c)
            {
                escape_command = Some(c);
                parser.state = EscapeSequence::None;
            } else if c == ';' {
                parser.escape_args.push(parser.current_arg.clone());
                parser.current_arg.clear();
            } else if c == '\u{0007}' {
                escape_command = parser.current_arg.pop();
                parser.escape_args.push(parser.current_arg.clone());
                parser.state = EscapeSequence::None;
            } else {
                parser.current_arg.push(c);
            }

            if parser.state == EscapeSequence::None {
                parser.escape_args.push(parser.current_arg.clone());
                parser.current_arg.clear();

                if let Some(escape_command) = escape_command {
                    parser.handle_escape_command(escape_command);
                }

                parser.escape_args.clear();
            }
        } else if c == '\r' {
            parser.state = EscapeSequence::LineStart;
        } else if parser.state == EscapeSequence::LineStart {
            if c == '\n' {
                parser.state = EscapeSequence::None;
                parser.cleaned_lines.push(parser.current_line.clone());
                parser.current_line.clear();
            } else {
                parser.state = EscapeSequence::None;
                parser.current_line.clear();
                parser.current_line.push(c);
            }
        } else if c == '\n' {
            parser.state = EscapeSequence::None;
            parser.cleaned_lines.push(parser.current_line.clone());
            parser.current_line.clear();
        } else {
            parser.current_line.push(c);
        }
    });

    parser.cleaned_lines.push(parser.current_line);
    parser.cleaned_lines.join("\n").trim_start().to_string()
}
