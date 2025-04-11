pub mod highlighter;
pub mod style;

pub mod config;
mod highlighters;
mod utils;

#[cfg(test)]
mod tests {
    pub(crate) mod escape_code_converter;
}
