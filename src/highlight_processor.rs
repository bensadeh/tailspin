use inlet_manifold::Highlighter;
use rayon::prelude::*;

pub struct HighlightProcessor {
    highlighter: Highlighter,
}

impl HighlightProcessor {
    pub const fn new(highlighters: Highlighter) -> HighlightProcessor {
        HighlightProcessor {
            highlighter: highlighters,
        }
    }

    pub fn apply(&self, lines: Vec<String>) -> String {
        lines
            .into_par_iter()
            .map(|line| self.highlighter.apply(line))
            .collect::<Vec<_>>()
            .join("\n")
    }
}
