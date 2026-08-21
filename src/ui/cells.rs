//! Width-aware helpers for measuring text and writing terminal cells.
//!
//! CJK characters and most emoji occupy two terminal cells, and their UTF-8
//! encoding takes 2–4 bytes. Any code that feeds a string length into cursor
//! positioning, centering, or column layout must measure *display width*,
//! not `str::len()` (bytes) or `chars().count()` — otherwise input via IME
//! (Chinese/Japanese/Korean) renders with drifting cursors and misaligned
//! chrome.

use unicode_width::UnicodeWidthChar;

/// Display width of `text` in terminal cells: wide chars (CJK, emoji) count
/// as 2, combining marks as 0.
pub fn display_width(text: &str) -> u16 {
    text.chars()
        .map(|c| UnicodeWidthChar::width(c).unwrap_or(0))
        .sum::<usize>() as u16
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_width_empty() {
        assert_eq!(display_width(""), 0);
    }

    #[test]
    fn display_width_ascii() {
        assert_eq!(display_width("hello"), 5);
    }

    #[test]
    fn display_width_cjk() {
        // Each hanzi occupies two cells.
        assert_eq!(display_width("中文"), 4);
        assert_eq!(display_width("日本語"), 6);
    }

    #[test]
    fn display_width_emoji() {
        assert_eq!(display_width("👍"), 2);
    }

    #[test]
    fn display_width_mixed() {
        assert_eq!(display_width("a中b"), 4);
    }

    #[test]
    fn display_width_zero_width() {
        // Combining marks attach to the previous cell.
        assert_eq!(display_width("e\u{301}"), 1);
    }

    #[test]
    fn display_width_matches_bytes_for_ascii() {
        let s = "plain ascii 123";
        assert_eq!(display_width(s), s.len() as u16);
    }
}
