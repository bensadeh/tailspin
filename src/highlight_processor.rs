use crate::highlighters::Highlighters;

pub struct HighlightProcessor {
    highlighters: Highlighters,
}

impl HighlightProcessor {
    pub fn new(highlighters: Highlighters) -> HighlightProcessor {
        HighlightProcessor { highlighters }
    }

    pub fn apply(&self, text: &str) -> String {
        let mut result = String::from(text);

        for highlight in &self.highlighters.before {
            result = highlight(&result);
        }

        for highlight in &self.highlighters.after {
            result = highlight(&result);
        }

        result
    }
}
