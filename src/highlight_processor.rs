use crate::highlighters::Highlighters;
use crate::line_info::LineInfo;
use crate::types::Highlight;
use rayon::prelude::*;
use std::sync::Arc;

pub struct HighlightProcessor {
    highlighters: Highlighters,
}

impl HighlightProcessor {
    pub fn new(highlighters: Highlighters) -> HighlightProcessor {
        HighlightProcessor { highlighters }
    }

    pub fn apply(&self, lines: Vec<String>) -> String {
        lines
            .par_iter()
            .map(|line| {
                let line_info = LineInfo::process(line);

                let stages = [
                    &self.highlighters.before,
                    &self.highlighters.main,
                    &self.highlighters.after,
                ];

                stages.iter().fold(String::from(line), |result, highlighters| {
                    self.apply_highlighters(&result, &line_info, highlighters)
                })
            })
            .collect()
    }

    fn apply_highlighters(
        &self,
        text: &str,
        line_info: &LineInfo,
        highlighters: &[Arc<dyn Highlight + Send + Sync>],
    ) -> String {
        highlighters
            .iter()
            .filter(|highlighter| !highlighter.should_short_circuit(line_info))
            .fold(String::from(text), |result, highlighter| highlighter.apply(&result))
    }
}
