use crate::core::highlighter::Highlight;
use crate::core::highlighters::StaticHighlight;
use memchr::memmem::Finder;
use std::borrow::Cow;
use std::cmp::min;

const THREE_KB: usize = 3 * 1024;

#[derive(Default)]
pub struct FoldState<'a> {
    result: Option<String>,
    processed: usize,
    input: &'a str,
}

impl FoldState<'_> {
    fn update(&mut self, text: &str) {
        if let Some(buf) = &mut self.result {
            buf.push_str(text);
        } else {
            self.processed += text.len();
        }
    }

    fn update_owned(&mut self, new_text: &str) {
        if self.result.is_none() {
            let mut buf = allocate_string(self.input);
            buf.push_str(&self.input[..self.processed]);
            self.result = Some(buf);
        }
        self.result.as_mut().unwrap().push_str(new_text);
    }
}

pub fn apply_only_to_unhighlighted<'a>(input: &'a str, highlighter: &StaticHighlight) -> Cow<'a, str> {
    let mut state = FoldState {
        input,
        ..Default::default()
    };

    for chunk in ChunkIter::new(input) {
        match chunk {
            Chunk::AlreadyHighlighted(text) => state.update(text),
            Chunk::NotHighlighted(text) => match highlighter.apply(text) {
                Cow::Borrowed(new_text) => state.update(new_text),
                Cow::Owned(new_text) => state.update_owned(&new_text),
            },
        }
    }

    state.result.map_or(Cow::Borrowed(input), Cow::Owned)
}

fn allocate_string(input: &str) -> String {
    let input_length_times_3 = input.len().saturating_mul(3);
    let allocation_size = min(input_length_times_3, THREE_KB);

    String::with_capacity(allocation_size)
}

enum Chunk<'a> {
    NotHighlighted(&'a str),
    AlreadyHighlighted(&'a str),
}

struct ChunkIter<'a> {
    input: &'a str,
    pos: usize,
    inside_escape: bool,
    escape_finder: Finder<'static>,
    reset_finder: Finder<'static>,
}

impl<'a> ChunkIter<'a> {
    fn new(input: &'a str) -> Self {
        Self {
            input,
            pos: 0,
            inside_escape: false,
            escape_finder: Finder::new("\x1b["),
            reset_finder: Finder::new("\x1b[0m"),
        }
    }
}

impl<'a> Iterator for ChunkIter<'a> {
    type Item = Chunk<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos >= self.input.len() {
            return None;
        }

        let remainder = &self.input[self.pos..];

        if self.inside_escape {
            if let Some(idx) = self.reset_finder.find(remainder.as_bytes()) {
                let end = self.pos + idx + "\x1b[0m".len();
                let chunk = Chunk::AlreadyHighlighted(&self.input[self.pos..end]);
                self.pos = end;
                self.inside_escape = false;
                Some(chunk)
            } else {
                let chunk = Chunk::AlreadyHighlighted(remainder);
                self.pos = self.input.len();
                Some(chunk)
            }
        } else if let Some(idx) = self.escape_finder.find(remainder.as_bytes()) {
            if idx == 0 {
                self.inside_escape = true;
                return self.next();
            }
            let chunk = Chunk::NotHighlighted(&self.input[self.pos..self.pos + idx]);
            self.pos += idx;
            Some(chunk)
        } else {
            let chunk = Chunk::NotHighlighted(remainder);
            self.pos = self.input.len();
            Some(chunk)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chunk_iter() {
        let input = "Text \x1b[31mhighlighted\x1b[0m text.";
        let chunks: Vec<_> = ChunkIter::new(input).collect();

        assert_eq!(chunks.len(), 3);
        match &chunks[0] {
            Chunk::NotHighlighted(text) => assert_eq!(*text, "Text "),
            _ => panic!("Unexpected chunk"),
        }
        match &chunks[1] {
            Chunk::AlreadyHighlighted(text) => assert_eq!(*text, "\x1b[31mhighlighted\x1b[0m"),
            _ => panic!("Unexpected chunk"),
        }
        match &chunks[2] {
            Chunk::NotHighlighted(text) => assert_eq!(*text, " text."),
            _ => panic!("Unexpected chunk"),
        }
    }
}
