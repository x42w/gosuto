use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use unicode_width::UnicodeWidthChar;

use crate::ui::{gradient, theme};

pub fn centered_rect(w: u16, h: u16, area: Rect) -> Rect {
    Rect::new(
        area.x + area.width.saturating_sub(w) / 2,
        area.y + area.height.saturating_sub(h) / 2,
        w.min(area.width),
        h.min(area.height),
    )
}

#[inline]
pub fn in_bounds(x: u16, y: u16, r: &Rect) -> bool {
    x >= r.x && x < r.x + r.width && y >= r.y && y < r.y + r.height
}

#[inline]
pub fn set_cell(buf: &mut Buffer, bounds: &Rect, x: u16, y: u16, ch: char, style: Style) {
    if in_bounds(x, y, bounds) {
        let cell = &mut buf[(x, y)];
        cell.set_char(ch);
        cell.set_style(style);
        cell.skip = false;
    }
}

pub fn write_str(buf: &mut Buffer, bounds: &Rect, x: u16, y: u16, text: &str, style: Style) {
    let mut cx = x;
    for ch in text.chars() {
        let w = UnicodeWidthChar::width(ch).unwrap_or(0);
        set_cell(buf, bounds, cx, y, ch, style);
        if w > 1 {
            set_cell(buf, bounds, cx + 1, y, ' ', style);
            cx += 2;
        } else if w == 0 {
            // zero-width: keep position
        } else {
            cx += 1;
        }
    }
}

pub fn fill_bg(buf: &mut Buffer, bounds: &Rect, popup: Rect) {
    for y in popup.y..popup.y + popup.height {
        for x in popup.x..popup.x + popup.width {
            if in_bounds(x, y, bounds) {
                let cell = &mut buf[(x, y)];
                cell.set_char(' ');
                cell.set_style(Style::default().bg(theme::BG));
                cell.skip = false;
            }
        }
    }
}

pub fn truncate_str(s: &str, max: usize) -> String {
    if s.len() > max {
        // Slice at a char boundary: max may fall inside a multi-byte
        // UTF-8 sequence (e.g. CJK, emoji), which would panic.
        let cut = s.floor_char_boundary(max.saturating_sub(1));
        format!("{}…", &s[..cut])
    } else {
        s.to_string()
    }
}

/// Render popup border with a perimeter gradient (cyan → magenta).
/// Uses double-line box characters: ╔ ╗ ╚ ╝ ═ ║
pub fn render_gradient_border(buf: &mut Buffer, bounds: &Rect, area: Rect, phase: f32) {
    let x1 = area.x;
    let x2 = area.x + area.width - 1;
    let y1 = area.y;
    let y2 = area.y + area.height - 1;

    gradient::walk_perimeter(area, |x, y, i, total| {
        let ch = match (x == x1, x == x2, y == y1, y == y2) {
            (true, _, true, _) => '╔',
            (_, true, true, _) => '╗',
            (true, _, _, true) => '╚',
            (_, true, _, true) => '╝',
            (_, _, true, _) | (_, _, _, true) => '═',
            _ => '║',
        };
        let color = gradient::perimeter_color(
            i,
            total,
            theme::GRADIENT_BORDER_START,
            theme::GRADIENT_BORDER_END,
            phase,
        );
        set_cell(
            buf,
            bounds,
            x,
            y,
            ch,
            Style::default().fg(color).bg(theme::BG),
        );
    });
}

pub fn render_title(buf: &mut Buffer, bounds: &Rect, area: Rect, color: Color, title: &str) {
    let border_s = Style::default().fg(color).bg(theme::BG);
    let title_s = border_s.add_modifier(Modifier::BOLD);

    let bracket_l = area.x + 3;
    let title_start = bracket_l + 2;

    set_cell(buf, bounds, bracket_l, area.y, '╡', border_s);
    set_cell(buf, bounds, bracket_l + 1, area.y, ' ', border_s);

    let mut cx = title_start;
    for ch in title.chars() {
        if cx >= area.x + area.width - 1 {
            break;
        }
        let w = UnicodeWidthChar::width(ch).unwrap_or(0);
        if w > 1 && cx + 1 >= area.x + area.width - 1 {
            break; // don't start a wide char at the edge
        }
        set_cell(buf, bounds, cx, area.y, ch, title_s);
        if w > 1 {
            set_cell(buf, bounds, cx + 1, area.y, ' ', title_s);
            cx += 2;
        } else if w == 0 {
            cx = cx; // zero-width: keep position
        } else {
            cx += 1;
        }
    }

    let title_width: u16 = title
        .chars()
        .map(|c| UnicodeWidthChar::width(c).unwrap_or(0))
        .sum::<usize>() as u16;
    let bracket_r_space = title_start + title_width;
    let bracket_r = bracket_r_space + 1;
    set_cell(buf, bounds, bracket_r_space, area.y, ' ', border_s);
    if bracket_r < area.x + area.width - 1 {
        set_cell(buf, bounds, bracket_r, area.y, '╞', border_s);
    }
}

pub fn render_hint(buf: &mut Buffer, bounds: &Rect, popup: Rect, hint: &str) {
    let hint_row = popup.y + popup.height.saturating_sub(2);
    let inner_w = popup.width.saturating_sub(6) as usize;
    let hint_width: usize = hint
        .chars()
        .map(|c| UnicodeWidthChar::width(c).unwrap_or(0))
        .sum();
    let left = popup.x + 3;
    let hx = left + (inner_w.saturating_sub(hint_width)) as u16 / 2;
    write_str(
        buf,
        bounds,
        hx,
        hint_row,
        hint,
        Style::default().fg(theme::DIM).bg(theme::BG),
    );
}

pub fn render_popup_chrome(buf: &mut Buffer, bounds: &Rect, popup: Rect, title: &str, phase: f32) {
    fill_bg(buf, bounds, popup);
    render_gradient_border(buf, bounds, popup, phase);
    render_title(buf, bounds, popup, theme::CYAN, title);
}

pub fn history_visibility_description(value: &str) -> &'static str {
    match value {
        "shared" => "All members see full history",
        "invited" => "See history from when invited",
        "joined" => "See history from when joined",
        "world_readable" => "Anyone can read full history",
        _ => "",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn truncate_str_short_unchanged() {
        assert_eq!(truncate_str("---炸", 50), "---炸");
    }

    #[test]
    fn truncate_str_safe_with_multibyte() {
        // 30 x "炸" = 90 bytes; cut lands mid-char, must not panic.
        let s = "炸".repeat(30);
        let t = truncate_str(&s, 50);
        assert!(t.ends_with('…'));
        // floor(49) = 48 bytes = 16 chars, then the ellipsis.
        assert_eq!(t, format!("{}…", "炸".repeat(16)));
        assert!(std::str::from_utf8(t.as_bytes()).is_ok());
    }

    #[test]
    fn truncate_str_truncates_ascii() {
        assert_eq!(truncate_str("hello world", 5), "hell…");
        assert_eq!(truncate_str("hello", 5), "hello");
    }

    #[test]
    fn truncate_str_zero_max() {
        assert_eq!(truncate_str("hello", 0), "…");
    }
}
