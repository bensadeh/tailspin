use crate::highlight_utils::apply_without_overwriting_existing_highlighting;
use crate::highlighters::Highlighters;
use crate::line_info::LineInfo;
use crate::types::Highlight;
use rayon::prelude::*;
use std::sync::Arc;

pub struct HighlightProcessor {
    highlighters: Highlighters,
}

impl HighlightProcessor {
    pub const fn new(highlighters: Highlighters) -> HighlightProcessor {
        HighlightProcessor { highlighters }
    }

    pub fn apply(&self, lines: &[String]) -> String {
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
                    HighlightProcessor::apply_highlighters(&result, &line_info, highlighters)
                })
            })
            .collect::<Vec<_>>()
            .join("\n")
    }

    fn apply_highlighters(
        text: &str,
        line_info: &LineInfo,
        highlighters: &[Arc<dyn Highlight + Send + Sync>],
    ) -> String {
        highlighters
            .iter()
            .filter(|highlighter| !highlighter.should_short_circuit(line_info))
            .fold(String::from(text), |acc, highlighter| {
                if highlighter.only_apply_to_segments_not_already_highlighted() {
                    let result =
                        apply_without_overwriting_existing_highlighting(&acc, |chunk| highlighter.apply(chunk));

                    match result {
                        std::borrow::Cow::Borrowed(_) => acc,
                        std::borrow::Cow::Owned(s) => s,
                    }
                } else {
                    let result = highlighter.apply(&acc);

                    match result {
                        std::borrow::Cow::Borrowed(_) => acc,
                        std::borrow::Cow::Owned(s) => s,
                    }
                }
            })
    }
}
