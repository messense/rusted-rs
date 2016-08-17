//! Terminal renderer that takes an iterator of events as input.

use pulldown_cmark::{Event, Tag};
use pulldown_cmark::Event::{Start, End, Text, Html, InlineHtml, SoftBreak, HardBreak};
use syntect::parsing::SyntaxSet;
use syntect::highlighting::{ThemeSet, Style};
use syntect::util::as_24_bit_terminal_escaped;
use syntect::easy::HighlightLines;

struct Ctx<'b, I> {
    iter: I,
    buf: &'b mut String,
    is_code: bool,
}

impl<'a, 'b, I: Iterator<Item=Event<'a>>> Ctx<'b, I> {
    fn fresh_line(&mut self) {
        if !(self.buf.is_empty() || self.buf.ends_with('\n')) {
            self.buf.push('\n');
        }
    }

    pub fn run(&mut self) {
        let ps = SyntaxSet::load_defaults_nonewlines();
        let ts = ThemeSet::load_defaults();
        let syntax = ps.find_syntax_by_extension("rs").unwrap();
        let mut highlighter = HighlightLines::new(syntax, &ts.themes["base16-ocean.dark"]);
        while let Some(event) = self.iter.next() {
            match event {
                Start(tag) => self.start_tag(tag),
                End(tag) => self.end_tag(tag),
                Text(text) |
                Html(text) |
                InlineHtml(text) => {
                    if !self.is_code {
                        self.buf.push_str(&text)
                    } else {
                        for line in text.lines() {
                            let ranges: Vec<(Style, &str)> = highlighter.highlight(&line);
                            let escaped = as_24_bit_terminal_escaped(&ranges[..], true);
                            self.buf.push_str(&escaped);
                            self.fresh_line();
                        }
                    }
                },
                SoftBreak => self.buf.push('\n'),
                HardBreak => self.buf.push_str("\n"),
                _ => {}
            }
        }
    }

    fn start_tag(&mut self, tag: Tag<'a>) {
        match tag {
            Tag::Paragraph =>  {
                self.fresh_line();
            }
            Tag::Rule => {
                self.fresh_line();
                self.buf.push_str("\n")
            }
            Tag::CodeBlock(_) => {
                self.is_code = true;
                self.fresh_line();
            }
            Tag::Code => {
                self.buf.push('`');
            }
            Tag::List(None) => {
                self.fresh_line();
            }
            Tag::List(Some(_)) => {
                self.fresh_line();
            }
            Tag::Item => {
                self.fresh_line();
                self.buf.push_str(" * ");
            }
            Tag::Link(dest, _) => {
                self.buf.push_str(&dest);
            }
            _ => {}
        }
    }

    fn end_tag(&mut self, tag: Tag) {
        match tag {
            Tag::Paragraph => {
                self.buf.push_str("\n\n");
                self.is_code = false;
            },
            Tag::Rule => {
                self.is_code = false;
            },
            Tag::BlockQuote => {
                self.buf.push_str("\n");
                self.is_code = false;
            },
            Tag::CodeBlock(_) => {
                self.buf.push_str("\n");
                self.is_code = false;
            },
            Tag::Code => {
                self.buf.push('`');
                self.is_code = false;
            },
            Tag::List(_) => {
                self.buf.push_str("\n");
                self.is_code = false;
            },
            Tag::Item => {
                self.buf.push_str("\n");
                self.is_code = false;
            },
            _ => {}
        }
    }
}

pub fn render<'a, I: Iterator<Item=Event<'a>>>(buf: &mut String, iter: I) {
    let mut ctx = Ctx {
        iter: iter,
        buf: buf,
        is_code: false,
    };
    ctx.run();
}
