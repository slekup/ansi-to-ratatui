// use ansi_to_tui::{ansi_to_text, ansi_to_text_override_style};
use ansi_to_ratatui::IntoText;
use pretty_assertions::assert_eq;
use ratatui::style::Stylize;
use ratatui::{
    style::{Color, Style},
    text::{Line, Span, Text},
};

#[test]
fn test_empty_op() {
    let string = b"\x1b[32mGREEN\x1b[mFOO\nFOO";
    let output = Text::from(vec![
        Line::from(vec![
            Span::styled("GREEN", Style::default().fg(Color::Green)),
            Span::styled("FOO", Style::reset()),
        ]),
        Line::from(Span::styled("FOO", Style::reset())),
    ]);
    test_both(string, output);
}

#[test]
fn test_string() {
    let string: Vec<u8> = "FOO".to_string().bytes().collect();
    test_both(string, Text::raw("FOO"));
}

#[test]
fn test_unicode() {
    // these are 8 byte unicode characters
    // first 4 bytes are for the unicode and the last 4 bytes are for the color / variant
    let bytes = "AAA🅱️🅱️🅱️".as_bytes().to_vec();
    let output = Text::raw("AAA🅱️🅱️🅱️");
    test_both(bytes, output);
}

#[test]
fn test_ascii_rgb() {
    let bytes: Vec<u8> = b"\x1b[38;2;100;100;100mAAABBB".to_vec();
    let output = Text::from(Span::styled(
        "AAABBB",
        Style::default().fg(Color::Rgb(100, 100, 100)),
    ));
    test_both(bytes, output);
}

// #[test]
// fn test_ascii_multi() {
//     let bytes = "\x1b[31m\x1b[4m\x1b[1mHELLO".as_bytes().to_vec();
//     println!("{:#?}", ansi_to_text(bytes));
// }

#[test]
fn test_ascii_newlines() {
    let bytes = "LINE_1\n\n\n\n\n\n\nLINE_8".as_bytes().to_vec();
    let output = Text::from(vec![
        Line::from("LINE_1"),
        Line::from(""),
        Line::from(""),
        Line::from(""),
        Line::from(""),
        Line::from(""),
        Line::from(""),
        Line::from("LINE_8"),
    ]);

    test_both(bytes, output);
}

#[test]
fn test_reset() {
    let string = "\x1b[33mA\x1b[0mB";
    let output = Text::from(Line::from(vec![
        Span::styled("A", Style::default().fg(Color::Yellow)),
        Span::styled("B", Style::reset()),
    ]));
    test_both(string, output);
}

#[test]
fn test_screen_modes() {
    let bytes: Vec<u8> = b"\x1b[?25hAAABBB".to_vec();
    let output = Text::styled(
        "AAABBB", // or "AAABBB"
        Style::default(),
    );
    test_both(bytes, output);
}

#[test]
fn test_cursor_shape_and_color() {
    // malformed -> malformed -> empty
    let bytes: Vec<u8> = b"\x1b[4 q\x1b]12;#fab1ed\x07".to_vec();
    let output = Text::raw("");
    test_both(bytes, output);
}

#[test]
fn test_malformed_simple() {
    let bytes: Vec<u8> = b"\x1b[".to_vec();
    let output = Text::raw("");
    test_both(bytes, output);
}

#[test]
fn test_malformed_complex() {
    let bytes: Vec<u8> = b"\x1b\x1b[0\x1b[m\x1b".to_vec();
    let output = Text::raw("");
    test_both(bytes, output);
}

#[test]
fn empty_span() {
    // Yellow -> Red -> Green -> "Hello" -> Reset -> "World"
    let bytes: Vec<u8> = b"\x1b[33m\x1b[31m\x1b[32mHello\x1b[0mWorld".to_vec();
    let output = Text::from(Line::from(vec![
        // Not sure whether to keep this empty span or remove it somehow
        Span::styled("", Style::default().fg(Color::Yellow)),
        // Span::styled("", Style::default().fg(Color::Red)),
        Span::styled("Hello", Style::default().fg(Color::Green)),
        Span::styled("World", Style::reset()),
    ]));
    test_both(bytes, output);
}

#[test]
fn test_color_and_style_reset() {
    let bytes: Vec<u8> = String::from(
        "\u{1b}[32m* \u{1b}[0mRunning before-startup command \u{1b}[1mcommand\u{1b}[0m=make my-simple-package.cabal\n\
        \u{1b}[32m* \u{1b}[0m$ make my-simple-package.cabal\n\
        Build profile: -w ghc-9.0.2 -O1\n").into_bytes();
    let output = Text::from(vec![
        Line::from(vec![
            Span::styled("* ", Style::default().fg(Color::Green)),
            Span::styled("Running before-startup command ", Style::reset()),
            Span::styled("command", Style::reset().bold()),
            Span::styled("=make my-simple-package.cabal", Style::reset()),
        ]),
        Line::from(vec![
            Span::styled("* ", Style::reset().fg(Color::Green)),
            Span::styled("$ make my-simple-package.cabal", Style::reset()),
        ]),
        Line::from(vec![Span::styled(
            "Build profile: -w ghc-9.0.2 -O1",
            Style::reset(),
        )]),
    ]);
    test_both(bytes, output);
}

#[cfg(test)]
pub fn test_both(bytes: impl AsRef<[u8]>, other: Text) {
    let bytes = bytes.as_ref();
    let zero_copy = bytes.to_text().unwrap();
    let owned = bytes.into_text().unwrap();
    assert_eq!(zero_copy, owned);
    assert_eq!(owned, other);
    assert_eq!(zero_copy, other);
}
