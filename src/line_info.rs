pub struct LineInfo {
    pub slashes: usize,
    pub dots: usize,
    pub dashes: usize,
}

impl LineInfo {
    pub fn process(line: &str) -> LineInfo {
        let (slashes, dots, dashes) =
            line.chars()
                .fold((0, 0, 0), |(slashes, dots, dashes), c| match c {
                    '/' => (slashes + 1, dots, dashes),
                    '.' => (slashes, dots + 1, dashes),
                    '-' => (slashes, dots, dashes + 1),
                    _ => (slashes, dots, dashes),
                });

        LineInfo {
            slashes,
            dots,
            dashes,
        }
    }
}
