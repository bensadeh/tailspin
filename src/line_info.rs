pub struct LineInfo {
    pub slashes: usize,
    pub dots: usize,
    pub dashes: usize,
}

impl LineInfo {
    pub fn process(line: &str) -> LineInfo {
        LineInfo {
            slashes: count_char(line, '/'),
            dots: count_char(line, '.'),
            dashes: count_char(line, '-'),
        }
    }
}

fn count_char(s: &str, c: char) -> usize {
    s.chars().filter(|&x| x == c).count()
}
