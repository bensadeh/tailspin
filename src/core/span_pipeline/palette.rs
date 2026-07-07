use std::ops::Index;

use nu_ansi_term::Style as NuStyle;

use crate::style::Style;

/// A compact handle to an interned [`Style`]: an index into the [`Palette`]'s
/// precomputed ANSI prefixes. Spans carry this instead of the style itself.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct StyleId(u16);

#[cfg(test)]
impl StyleId {
    pub(crate) const fn new(id: u16) -> Self {
        Self(id)
    }
}

/// The distinct ANSI prefixes a pipeline can emit, frozen at build time.
/// Finders intern their configured styles at construction, so the render hot
/// path resolves a span's style with a plain array index.
///
/// Interning canonicalizes: styles rendering to equal prefixes share one id.
/// This is load-bearing, not just thrifty — merge coalesces adjacent bytes by
/// id equality, so equal ids for equal prefixes is what keeps same-styled
/// fragments from different finders merging into a single span.
#[derive(Debug, Clone)]
pub(crate) struct Palette {
    prefixes: Vec<String>,
}

impl Palette {
    pub const fn new() -> Self {
        Self { prefixes: Vec::new() }
    }

    pub fn intern(&mut self, style: Style) -> StyleId {
        let prefix = NuStyle::from(style).prefix().to_string();
        let index = self.prefixes.iter().position(|p| *p == prefix).unwrap_or_else(|| {
            self.prefixes.push(prefix);
            self.prefixes.len() - 1
        });

        StyleId(u16::try_from(index).expect("distinct style count exceeds u16 range"))
    }
}

impl Index<StyleId> for Palette {
    type Output = str;

    fn index(&self, id: StyleId) -> &str {
        &self.prefixes[id.0 as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::style::Color;

    #[test]
    fn interning_the_same_style_returns_the_same_id() {
        let mut palette = Palette::new();
        let a = palette.intern(Style::new().fg(Color::Red));
        let b = palette.intern(Style::new().fg(Color::Blue));
        let c = palette.intern(Style::new().fg(Color::Red));

        assert_eq!(a, c);
        assert_ne!(a, b);
    }

    #[test]
    fn indexing_resolves_the_precomputed_prefix() {
        let mut palette = Palette::new();
        let cyan = palette.intern(Style::new().fg(Color::Cyan));

        assert_eq!(&palette[cyan], "\x1b[36m");
    }
}
