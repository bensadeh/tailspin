use crate::highlighters::Highlighters;
use crate::line_info::LineInfo;

pub struct HighlightProcessor {
    highlighters: Highlighters,
}

impl HighlightProcessor {
    pub fn new(highlighters: Highlighters) -> HighlightProcessor {
        HighlightProcessor { highlighters }
    }

    pub fn apply(&self, text: &str) -> String {
        let mut result = String::from(text);
        let line_info = LineInfo::process(text);

        for highlight in &self.highlighters.before {
            result = highlight(&result, &line_info);
        }

        for highlight in &self.highlighters.main {
            result = highlight(&result, &line_info);
        }

        for highlight in &self.highlighters.after {
            result = highlight(&result, &line_info);
        }

        result
    }
}
