use crate::highlighters::Highlighters;
use crate::line_info::LineInfo;
use crate::types::Highlight;

pub struct HighlightProcessor {
    highlighters: Highlighters,
}

impl HighlightProcessor {
    pub fn new(highlighters: Highlighters) -> HighlightProcessor {
        HighlightProcessor { highlighters }
    }

    pub fn apply(&self, text: &str) -> String {
        let line_info = LineInfo::process(text);

        let stages = [
            &self.highlighters.before,
            &self.highlighters.main,
            &self.highlighters.after,
        ];

        stages.iter().fold(String::from(text), |result, highlighters| {
            self.apply_highlighters(&result, &line_info, highlighters)
        })
    }

    fn apply_highlighters(
        &self,
        text: &str,
        line_info: &LineInfo,
        highlighters: &[Box<dyn Highlight + Send>],
    ) -> String {
        highlighters
            .iter()
            .filter(|highlighter| !highlighter.should_short_circuit(line_info))
            .fold(String::from(text), |result, highlighter| highlighter.apply(&result))
    }
}
