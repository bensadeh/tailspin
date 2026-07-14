use clap::ValueEnum;
use clap::builder::{StyledStr, Styles, styling};
use std::fmt::Write;
use styling::{AnsiColor, Style};

const CONTEXT: Style = Style::new().dimmed();
const CONTEXT_VALUE: Style = AnsiColor::Green.on_default();

pub const fn get_styles() -> Styles {
    Styles::styled()
        .header(Style::new().bold())
        .usage(Style::new().bold())
        .literal(AnsiColor::Blue.on_default().bold())
        .placeholder(AnsiColor::Yellow.on_default())
        .context(CONTEXT)
        .context_value(CONTEXT_VALUE)
}

/// Arg help with the enum's possible values on their own line, styled like
/// the `context`/`context_value` spec values clap renders inline.
pub fn help_with_possible_values<T: ValueEnum>(description: &str) -> StyledStr {
    help_with_value_list(description, "possible values", &possible_value_names::<T>())
}

pub fn help_with_possible_values_and_env<T: ValueEnum>(description: &str, env_var: &str) -> StyledStr {
    let mut help = help_with_possible_values::<T>(description);
    append_env(&mut help, env_var);
    help
}

pub fn help_with_env(description: &str, env_var: &str) -> StyledStr {
    let mut help = StyledStr::new();
    let _ = write!(help, "{description}");
    append_env(&mut help, env_var);
    help
}

pub fn help_with_value_list<S: AsRef<str>>(description: &str, label: &str, values: &[S]) -> StyledStr {
    let mut help = StyledStr::new();
    let _ = write!(help, "{description}");
    append_value_list(&mut help, label, values);
    help
}

fn possible_value_names<T: ValueEnum>() -> Vec<String> {
    T::value_variants()
        .iter()
        .filter_map(ValueEnum::to_possible_value)
        .map(|value| value.get_name().to_owned())
        .collect()
}

fn append_value_list<S: AsRef<str>>(help: &mut StyledStr, label: &str, values: &[S]) {
    // The newline lives inside the styled block: clap's wrapping only resets
    // its line-width counter at a newline within a styled segment.
    let _ = write!(help, "{CONTEXT}\n[{label}: {CONTEXT:#}");

    for (i, value) in values.iter().enumerate() {
        // The separator rides inside the value's styled segment so a wrapped
        // line can never start with the comma.
        let separator = if i + 1 < values.len() { ", " } else { "" };
        let _ = write!(help, "{CONTEXT_VALUE}{}{separator}{CONTEXT_VALUE:#}", value.as_ref());
    }

    let _ = write!(help, "{CONTEXT}]{CONTEXT:#}");
}

/// Mirrors the `[env: VAR=value]` spec clap renders inline, on its own line.
fn append_env(help: &mut StyledStr, env_var: &str) {
    let value = std::env::var(env_var).unwrap_or_default();
    let _ = write!(
        help,
        "{CONTEXT}\n[env: {CONTEXT:#}{CONTEXT_VALUE}{env_var}={value}{CONTEXT_VALUE:#}{CONTEXT}]{CONTEXT:#}"
    );
}
