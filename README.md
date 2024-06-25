# ansi-to-ratatui

Forked from [ansi-to-tui](https://github.com/uttarayan21/ansi-to-tui).

A nom parser to parse text with ANSI color codes and turn them into [`ratatui::text::Text`][Text].

## Example

```rust
use ansi_to_ratatui::IntoText;
let buffer = std::fs::read("ascii/text.ascii").unwrap();
let output = buffer.into_text();
```

[Text]: https://docs.rs/ratatui/latest/ratatui/text/struct.Text.html
[ansi-to-ratatui]: https://github.com/slekup/ansi-to-ratatui
[ansi-to-tui]: https://github.com/uttarayan21/ansi-to-tui
