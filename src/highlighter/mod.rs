use inlet_manifold::*;

pub struct ManifoldTheme {
    pub number_config: Option<NumberConfig>,
    pub quote_config: Option<QuoteConfig>,
}

fn get_highlighter(theme: ManifoldTheme) -> Result<Highlighter, Error> {
    let mut builder = Highlighter::builder();

    if let Some(number_config) = theme.number_config {
        builder.with_number_highlighter(number_config);
    }

    if let Some(quote_config) = theme.quote_config {
        builder.with_quote_highlighter(quote_config);
    }

    builder.build()
}
