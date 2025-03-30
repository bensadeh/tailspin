pub mod config;
pub mod core;
pub mod defaults;
pub mod error;
pub mod style;

mod highlighters;
mod normalizer;
mod split_and_apply;

#[cfg(test)]
mod tests {
    pub(crate) mod escape_code_converter;
}
