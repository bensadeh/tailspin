use std::collections::HashMap;

pub trait ConvertEscapeCodes {
    fn convert_escape_codes(self) -> String;
}

impl ConvertEscapeCodes for String {
    fn convert_escape_codes(self) -> String {
        let mut code_map = HashMap::new();

        // Foreground colors
        code_map.insert("\x1b[31m", "[red]");
        code_map.insert("\x1b[32m", "[green]");
        code_map.insert("\x1b[33m", "[yellow]");
        code_map.insert("\x1b[34m", "[blue]");
        code_map.insert("\x1b[35m", "[magenta]");
        code_map.insert("\x1b[36m", "[cyan]");
        code_map.insert("\x1b[37m", "[white]");

        // Background colors
        code_map.insert("\x1b[41m", "[bg_red]");
        code_map.insert("\x1b[42m", "[bg_green]");
        code_map.insert("\x1b[43m", "[bg_yellow]");
        code_map.insert("\x1b[44m", "[bg_blue]");
        code_map.insert("\x1b[45m", "[bg_magenta]");
        code_map.insert("\x1b[46m", "[bg_cyan]");
        code_map.insert("\x1b[47m", "[bg_white]");

        // Other attributes
        code_map.insert("\x1b[1m", "[bold]");
        code_map.insert("\x1b[3m", "[italic]");
        code_map.insert("\x1b[4m", "[underline]");
        code_map.insert("\x1b[0m", "[reset]");

        let mut result = self;

        for (code, replacement) in code_map {
            result = result.replace(code, replacement);
        }

        result
    }
}

pub trait ConvertHighlightCodes {
    fn convert_highlight_codes(self) -> String;
}

impl ConvertHighlightCodes for String {
    fn convert_highlight_codes(self) -> String {
        let mut code_map = HashMap::new();

        // Foreground colors
        code_map.insert("[red]", "\x1b[31m");
        code_map.insert("[green]", "\x1b[32m");
        code_map.insert("[yellow]", "\x1b[33m");
        code_map.insert("[blue]", "\x1b[34m");
        code_map.insert("[magenta]", "\x1b[35m");
        code_map.insert("[cyan]", "\x1b[36m");
        code_map.insert("[white]", "\x1b[37m");

        // Background colors
        code_map.insert("[bg_red]", "\x1b[41m");
        code_map.insert("[bg_green]", "\x1b[42m");
        code_map.insert("[bg_yellow]", "\x1b[43m");
        code_map.insert("[bg_blue]", "\x1b[44m");
        code_map.insert("[bg_magenta]", "\x1b[45m");
        code_map.insert("[bg_cyan]", "\x1b[46m");
        code_map.insert("[bg_white]", "\x1b[47m");

        // Other attributes
        code_map.insert("[bold]", "\x1b[1m");
        code_map.insert("[italic]", "\x1b[3m");
        code_map.insert("[underline]", "\x1b[4m");
        code_map.insert("[reset]", "\x1b[0m");

        let mut result = self;

        for (replacement, code) in code_map {
            result = result.replace(replacement, code);
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::{ConvertEscapeCodes, ConvertHighlightCodes};

    #[test]
    fn test_escape_code_converter() {
        let input = "\x1b[31mHello \x1b[1mWorld\x1b[0m!".to_string();
        let expected = "[red]Hello [bold]World[reset]!";

        let actual = input.convert_escape_codes();

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_escape_code_converter_with_bg() {
        let input = "\x1b[41mHello \x1b[1mWorld\x1b[0m!".to_string();
        let expected = "[bg_red]Hello [bold]World[reset]!";

        let actual = input.convert_escape_codes();

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_highlight_code_converter() {
        let input = "[red]Hello [bold]World[reset]!".to_string();
        let expected = "\x1b[31mHello \x1b[1mWorld\x1b[0m!";

        let actual = input.convert_highlight_codes();

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_highlight_code_converter_with_bg() {
        let input = "[bg_red]Hello [bold]World[reset]!".to_string();
        let expected = "\x1b[41mHello \x1b[1mWorld\x1b[0m!";

        let actual = input.convert_highlight_codes();

        assert_eq!(actual, expected);
    }
}
