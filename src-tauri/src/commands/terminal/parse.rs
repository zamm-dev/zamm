use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref THREE_OR_MORE_NEWLINES: Regex = Regex::new(r"\n{3,}").unwrap();
}

#[derive(PartialEq)]
pub enum EscapeSequence {
    None,
    Start,
    InEscape,
    InOperatingSystemEscape,
    LineStart,
}

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

    pub fn handle_escape_sequence_start(&mut self, c: char) {
        if c == '[' || c == '(' {
            self.state = EscapeSequence::InEscape;
        } else if c == ']' {
            self.state = EscapeSequence::InOperatingSystemEscape;
        } else {
            self.state = EscapeSequence::None;
            self.current_line.push(c);
        }
    }

    pub fn handle_escape_sequence_input(&mut self, c: char) {
        let mut escape_command: Option<char> = None;
        if c == '?' {
            // it's just a private sequence marker, do nothing
        } else if self.state == EscapeSequence::InEscape && c.is_ascii_alphabetic() {
            escape_command = Some(c);
            self.state = EscapeSequence::None;
        } else if c == ';' {
            self.escape_args.push(self.current_arg.clone());
            self.current_arg.clear();
        } else if c == '\u{0007}' {
            escape_command = self.current_arg.pop();
            self.escape_args.push(self.current_arg.clone());
            self.state = EscapeSequence::None;
        } else {
            self.current_arg.push(c);
        }

        // if we changed the state here
        if self.state == EscapeSequence::None {
            self.escape_args.push(self.current_arg.clone());
            self.current_arg.clear();

            if let Some(escape_command) = escape_command {
                self.handle_escape_command(escape_command);
            }

            self.escape_args.clear();
        }
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

    pub fn handle_newline(&mut self) {
        self.state = EscapeSequence::None;
        self.cleaned_lines.push(self.current_line.clone());
        self.current_line.clear();
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
            parser.handle_escape_sequence_start(c);
        } else if parser.state == EscapeSequence::InEscape
            || parser.state == EscapeSequence::InOperatingSystemEscape
        {
            parser.handle_escape_sequence_input(c);
        } else if c == '\r' {
            parser.state = EscapeSequence::LineStart;
        } else if parser.state == EscapeSequence::LineStart {
            if c == '\n' {
                parser.handle_newline();
            } else {
                parser.state = EscapeSequence::None;
                parser.current_line.clear();
                parser.current_line.push(c);
            }
        } else if c == '\n' {
            parser.handle_newline();
        } else {
            parser.current_line.push(c);
        }
    });

    parser.cleaned_lines.push(parser.current_line);
    THREE_OR_MORE_NEWLINES
        .replace_all(&parser.cleaned_lines.join("\n"), "\n\n")
        .trim_start()
        .to_string()
}
