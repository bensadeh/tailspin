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
        let mut result = String::from(text);
        let line_info = LineInfo::process(text);

        result = self.apply_highlighters(&result, &line_info, &self.highlighters.before);
        result = self.apply_highlighters(&result, &line_info, &self.highlighters.main);
        result = self.apply_highlighters(&result, &line_info, &self.highlighters.after);

        result
    }

    fn apply_highlighters(
        &self,
        text: &str,
        line_info: &LineInfo,
        highlighters: &Vec<Box<dyn Highlight + Send>>,
    ) -> String {
        let mut result = String::from(text);

        for highlight in highlighters {
            result = highlight.apply(&result, line_info);
        }

        result
    }
}
