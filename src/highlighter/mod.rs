pub mod core;
pub mod error;
pub mod style;

pub mod config;
mod highlighters;
mod normalizer;
mod split_and_apply;

#[cfg(test)]
mod tests {
    pub(crate) mod escape_code_converter;
}
