pub struct LineInfo {
    pub slashes: usize,
    pub dots: usize,
    pub dashes: usize,
    pub double_quotes: usize,
}

impl LineInfo {
    pub fn process(line: &str) -> LineInfo {
        let mut slashes = 0;
        let mut dots = 0;
        let mut dashes = 0;
        let mut double_quotes = 0;

        for c in line.chars() {
            match c {
                '/' => slashes += 1,
                '.' => dots += 1,
                '-' => dashes += 1,
                '"' => double_quotes += 1,
                _ => {}
            }
        }

        LineInfo {
            slashes,
            dots,
            dashes,
            double_quotes,
        }
    }
}
