//! Width-aware helpers for measuring text and writing terminal cells.
//!
//! CJK characters and most emoji occupy two terminal cells, and their UTF-8
//! encoding takes 2–4 bytes. Any code that feeds a string length into cursor
//! positioning, centering, or column layout must measure *display width*,
//! not `str::len()` (bytes) or `chars().count()` — otherwise input via IME
//! (Chinese/Japanese/Korean) renders with drifting cursors and misaligned
//! chrome.
//!
//! All buffer writes here step by display width and mark the continuation
//! cell of a wide character with `skip = true` so ratatui's buffer diffing
//! leaves it alone (writing it each frame flickers and clobbers the right
//! half of the glyph).

use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::Style;
use unicode_width::UnicodeWidthChar;

/// Display width of `text` in terminal cells: wide chars (CJK, emoji) count
/// as 2, combining marks as 0.
pub fn display_width(text: &str) -> u16 {
    text.chars()
        .map(|c| UnicodeWidthChar::width(c).unwrap_or(0))
        .sum::<usize>() as u16
}

#[inline]
pub fn in_bounds(x: u16, y: u16, r: &Rect) -> bool {
    x >= r.x && x < r.x + r.width && y >= r.y && y < r.y + r.height
}

/// Write a single cell if within buffer bounds.
#[inline]
pub fn set_cell(buf: &mut Buffer, bounds: &Rect, x: u16, y: u16, ch: char, style: Style) {
    if in_bounds(x, y, bounds) {
        let cell = &mut buf[(x, y)];
        cell.set_char(ch);
        cell.set_style(style);
        cell.skip = false;
    }
}

/// Write one character at column `x` (a *cell* column, not a char index),
/// returning the number of cells advanced. Wide characters (CJK, emoji)
/// occupy two cells, with the continuation cell marked `skip`. Zero-width
/// characters (combining marks) attach to the previous cell and advance 0.
pub(crate) fn write_char(
    buf: &mut Buffer,
    bounds: &Rect,
    x: u16,
    y: u16,
    ch: char,
    style: Style,
) -> u16 {
    let width = UnicodeWidthChar::width(ch).unwrap_or(0);
    if width == 0 {
        // Zero-width (combining) char: append it to the previous cell's
        // symbol so the terminal renders the full grapheme cluster.
        if x > bounds.x && in_bounds(x - 1, y, bounds) {
            let cell = &mut buf[(x - 1, y)];
            let mut sym = cell.symbol().to_string();
            sym.push(ch);
            cell.set_symbol(&sym);
        }
        return 0;
    }
    set_cell(buf, bounds, x, y, ch, style);
    if width > 1 {
        let cont_x = x + 1;
        if cont_x < bounds.x + bounds.width {
            let cont = &mut buf[(cont_x, y)];
            cont.set_char(' ');
            cont.set_style(style);
            cont.skip = true;
        }
    }
    width as u16
}

/// Write a string starting at column `x`, stepping by display width.
pub fn write_str(buf: &mut Buffer, bounds: &Rect, x: u16, y: u16, text: &str, style: Style) {
    let mut cx = x;
    for ch in text.chars() {
        cx += write_char(buf, bounds, cx, y, ch, style);
    }
}

/// Write `chars` starting at column `x`, stopping at `limit_x` (exclusive).
/// A wide character that would straddle `limit_x` is not started. Returns
/// the column just past the last written character.
pub fn write_chars_until(
    buf: &mut Buffer,
    bounds: &Rect,
    x: u16,
    y: u16,
    chars: impl IntoIterator<Item = char>,
    style: Style,
    limit_x: u16,
) -> u16 {
    let mut cx = x;
    for ch in chars {
        if cx >= limit_x {
            break;
        }
        let w = UnicodeWidthChar::width(ch).unwrap_or(0);
        if w > 1 && cx + 1 >= limit_x {
            break; // don't start a wide char at the edge
        }
        cx += write_char(buf, bounds, cx, y, ch, style);
    }
    cx
}

/// Write a string clipped to `clip`. Returns `true` if the text was
/// truncated. When `ellipsis` is set and the text is truncated, the last
/// visible cell is replaced with `…`.
pub fn write_str_clipped(
    buf: &mut Buffer,
    x: u16,
    y: u16,
    text: &str,
    style: Style,
    clip: &Rect,
    ellipsis: bool,
) -> bool {
    let bounds = *buf.area();
    let clip_end = clip.x + clip.width;
    let truncated = x + display_width(text) > clip_end;

    let mut cx = x;
    for ch in text.chars() {
        if cx >= clip_end {
            break;
        }
        let w = UnicodeWidthChar::width(ch).unwrap_or(0);
        // Don't start a wide char at the clip edge — skip it entirely.
        if w > 1 && cx + 1 > clip_end {
            break;
        }
        if in_bounds(cx, y, &bounds) {
            cx += write_char(buf, &bounds, cx, y, ch, style);
        } else {
            // Out of bounds: still advance by display width to keep alignment.
            cx = cx.saturating_add(w.max(1) as u16);
        }
    }

    if truncated && ellipsis && clip_end > clip.x {
        set_cell(buf, &bounds, clip_end - 1, y, '\u{2026}', style);
    }

    truncated
}

#[cfg(test)]
mod tests {
    use super::*;

    fn buffer() -> (Buffer, Rect) {
        let buf = Buffer::empty(Rect::new(0, 0, 20, 3));
        let bounds = *buf.area();
        (buf, bounds)
    }

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

    #[test]
    fn write_str_narrow_chars() {
        let (mut buf, bounds) = buffer();
        write_str(&mut buf, &bounds, 0, 0, "ab", Style::default());
        assert_eq!(buf[(0, 0)].symbol(), "a");
        assert_eq!(buf[(1, 0)].symbol(), "b");
    }

    #[test]
    fn write_str_wide_char_marks_skip_continuation() {
        let (mut buf, bounds) = buffer();
        write_str(&mut buf, &bounds, 0, 0, "日", Style::default());
        assert_eq!(buf[(0, 0)].symbol(), "日");
        assert!(!buf[(0, 0)].skip);
        assert_eq!(buf[(1, 0)].symbol(), " ");
        assert!(buf[(1, 0)].skip);
    }

    #[test]
    fn write_str_next_char_after_wide_lands_two_cells_over() {
        let (mut buf, bounds) = buffer();
        write_str(&mut buf, &bounds, 0, 0, "日x", Style::default());
        assert_eq!(buf[(0, 0)].symbol(), "日");
        assert_eq!(buf[(2, 0)].symbol(), "x");
    }

    #[test]
    fn write_str_combining_char_appends_to_previous() {
        let (mut buf, bounds) = buffer();
        write_str(&mut buf, &bounds, 1, 0, "e\u{301}", Style::default());
        assert_eq!(buf[(1, 0)].symbol(), "e\u{301}");
        assert_eq!(buf[(2, 0)].symbol(), " ");
    }

    #[test]
    fn write_str_out_of_bounds_is_noop() {
        let (mut buf, bounds) = buffer();
        write_str(&mut buf, &bounds, 30, 0, "ab", Style::default());
        assert_eq!(buf[(19, 0)].symbol(), " ");
    }

    #[test]
    fn write_chars_until_stops_at_limit() {
        let (mut buf, bounds) = buffer();
        let end = write_chars_until(
            &mut buf,
            &bounds,
            0,
            0,
            "abcde".chars(),
            Style::default(),
            3,
        );
        assert_eq!(end, 3);
        assert_eq!(buf[(2, 0)].symbol(), "c");
        assert_eq!(buf[(3, 0)].symbol(), " ");
    }

    #[test]
    fn write_chars_until_skips_wide_char_at_edge() {
        let (mut buf, bounds) = buffer();
        // "ab日": 日 would occupy cells 2-3, but limit is 3 → not started.
        let end = write_chars_until(&mut buf, &bounds, 0, 0, "ab日".chars(), Style::default(), 3);
        assert_eq!(end, 2);
        assert_eq!(buf[(2, 0)].symbol(), " ");
    }

    #[test]
    fn write_str_clipped_reports_truncation() {
        let (mut buf, _) = buffer();
        let clip = Rect::new(0, 0, 5, 1);
        assert!(write_str_clipped(
            &mut buf,
            0,
            0,
            "abcdef",
            Style::default(),
            &clip,
            false
        ));
        assert!(!write_str_clipped(
            &mut buf,
            0,
            0,
            "abc",
            Style::default(),
            &clip,
            false
        ));
    }

    #[test]
    fn write_str_clipped_writes_ellipsis() {
        let (mut buf, _) = buffer();
        let clip = Rect::new(0, 0, 5, 1);
        write_str_clipped(&mut buf, 0, 0, "abcdef", Style::default(), &clip, true);
        assert_eq!(buf[(4, 0)].symbol(), "…");
    }

    #[test]
    fn write_str_clipped_cjk_truncation_detection() {
        let (mut buf, _) = buffer();
        let clip = Rect::new(0, 0, 5, 1);
        // 3 hanzi = 6 cells > 5 → truncated. The third hanzi still starts
        // on the last clip column (its continuation cell is the first one
        // past the clip); with ellipsis it is replaced by "…".
        assert!(write_str_clipped(
            &mut buf,
            0,
            0,
            "日本語",
            Style::default(),
            &clip,
            false
        ));
        assert_eq!(buf[(0, 0)].symbol(), "日");
        assert_eq!(buf[(2, 0)].symbol(), "本");
        assert_eq!(buf[(4, 0)].symbol(), "語");

        let (mut buf, _) = buffer();
        assert!(write_str_clipped(
            &mut buf,
            0,
            0,
            "日本語",
            Style::default(),
            &clip,
            true
        ));
        assert_eq!(buf[(4, 0)].symbol(), "…");
    }
}
