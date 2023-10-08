use crate::data::kv::Kv;
use crate::data::kvs::Kvs;
use crate::data::path::Path;
use crate::data::tokens::Tokens;
use crate::yaml::parse::line::Line;

pub struct Parser {
    kvs: Kvs,
    path: Path,
    last_indent: usize,
}
impl Parser {
    pub fn new() -> Self {
        Parser {
            kvs: Kvs::new(),
            path: Path::new(),
            last_indent: 0,
        }
    }

    pub fn parse(&mut self, text: &str) -> Kvs {
        self.push_mkdict(self.path.clone());

        let mut line = Line::new();
        for c in text.chars() {
            line.push(c);
            if line.is_ended() {
                line.flush();
                self.push_line(line);
                line = Line::new();
            }
        }
        line.flush();
        self.push_line(line);

        self.append_close_tags();
        self.push_enddict(Path::new());

        self.kvs.clone()
    }

    fn push_line(&mut self, line: Line) {
        if self.last_indent > line.get_indent() {
            self.path.pop();
            self.push(self.path.clone(), Tokens::EndDict);
            self.path.pop();
            self.path.push(&line.get_key());
        }
        if self.last_indent < line.get_indent() {
            self.push_mkdict(self.path.clone());
            self.path.push(&line.get_key());
        }
        if self.last_indent == line.get_indent() {
            if line.has_hyphen() {
                if self.path.is_last_index() {
                    let index = self.path.get_last_index();
                    self.path.modify_index(index + 1);
                } else {
                    self.push_mkarray(self.path.clone());
                    self.path.push_index(0);
                }
            } else {
                if self.path.is_last_index() {
                    self.path.pop();
                    self.push_endarray(self.path.clone());
                }
                self.path.pop();    
                self.path.push(&line.get_key());
            };
        };

        self.last_indent = line.get_indent().clone();

        if !line.has_value() {
            return;
        }
        self.push(self.path.clone(), self.judge_token(line.get_value()));
    }

    fn judge_token(&self, text: String) -> Tokens {
        match text.as_str() {
            "null" => Tokens::Null,
            "false" => Tokens::Bool(false),
            "true" => Tokens::Bool(true),
            "" => Tokens::String(text),
            _ => {
                if text.chars().all(|c| c.is_numeric()) {
                    Tokens::Number(text.parse::<usize>().unwrap())
                } else {
                    Tokens::String(text)
                }
            },
        }
    }

    fn push(&mut self, path: Path, value: Tokens) {
        self.kvs.push(Kv::with(path, value));
    }

    fn push_mkdict(&mut self, path: Path) {
        self.push(path, Tokens::MkDict);
    }

    fn push_enddict(&mut self, path: Path) {
        self.push(path, Tokens::EndDict);
    }

    fn push_mkarray(&mut self, path: Path) {
        self.push(path, Tokens::MkArray);
    }

    fn push_endarray(&mut self, path: Path) {
        self.push(path, Tokens::EndArray);
    }

    fn append_close_tags(&mut self) {
        if self.path.len() > 1 {
            if self.path.is_last_index() {
                self.path.pop();
                self.push_endarray(self.path.clone());
                self.path.pop();
            } else {
                self.path.pop();
                self.push_enddict(self.path.clone());
                self.path.pop();
            }
        }
    }
}
