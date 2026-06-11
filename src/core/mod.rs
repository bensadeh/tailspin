pub mod highlighter;
pub mod style;

pub mod config;
pub(crate) mod span_pipeline;

#[cfg(test)]
mod tests {
    pub(crate) mod escape_code_converter;
}
