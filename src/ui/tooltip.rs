use ratatui::{buffer::Buffer, layout::Rect, style::Style};
use unicode_width::UnicodeWidthChar;

use super::theme;

/// Direction the tooltip box opens relative to its anchor.
pub enum Direction {
    /// Tooltip opens to the right of the anchor (for room list).
    Right,
    /// Tooltip opens to the left of the anchor (for members pane).
    Left,
}

/// Write a single cell if within buffer bounds.
#[inline]
pub fn set_cell_if(buf: &mut Buffer, bounds: &Rect, x: u16, y: u16, ch: char, style: Style) {
    if x >= bounds.x && x < bounds.x + bounds.width && y >= bounds.y && y < bounds.y + bounds.height
    {
        buf[(x, y)].set_char(ch);
        buf[(x, y)].set_style(style);
    }
}

/// Write one character at `x` (a *cell* column, not a char index), advancing
/// `x` by the character's display width. Wide characters (e.g. emoji) occupy
/// two cells; the second cell is marked `skip` so ratatui's buffer diffing
/// doesn't overwrite it each frame and cause flicker artifacts.
fn write_char_advance(
    buf: &mut Buffer,
    bounds: &Rect,
    x: &mut u16,
    y: u16,
    ch: char,
    style: Style,
) {
    let width = UnicodeWidthChar::width(ch).unwrap_or(0);
    if width == 0 {
        // Zero-width (combining) char: attach to the previous cell.
        if *x > bounds.x {
            buf[(*x - 1, y)].set_char(ch);
        }
        return;
    }
    let cx = *x;
    if cx < bounds.x + bounds.width && y >= bounds.y && y < bounds.y + bounds.height {
        let cell = &mut buf[(cx, y)];
        cell.set_char(ch);
        cell.set_style(style);
        cell.skip = false;
    }
    *x += 1;
    if width > 1 && cx + 1 < bounds.x + bounds.width {
        let cont = &mut buf[(cx + 1, y)];
        cont.set_char(' ');
        cont.set_style(style);
        cont.skip = true;
    }
    *x += 1;
}

/// Write a string clipped to the given rectangle. Returns `true` if the text was truncated.
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
    // Total display width, not char count (wide chars occupy 2 cells).
    let text_width: u16 = text
        .chars()
        .map(|c| UnicodeWidthChar::width(c).unwrap_or(0))
        .sum::<usize>() as u16;
    let truncated = x + text_width > clip_end;

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
        if cx >= bounds.x
            && cx < bounds.x + bounds.width
            && y >= bounds.y
            && y < bounds.y + bounds.height
        {
            write_char_advance(buf, &bounds, &mut cx, y, ch, style);
        } else {
            // Out of bounds: still advance by display width to keep alignment.
            cx = cx.saturating_add(w.max(1) as u16);
        }
    }

    // Overwrite last visible character with ellipsis when truncated
    if truncated && ellipsis && clip_end > clip.x {
        let last = clip_end - 1;
        if last >= bounds.x
            && last < bounds.x + bounds.width
            && y >= bounds.y
            && y < bounds.y + bounds.height
        {
            buf[(last, y)].set_char('\u{2026}');
            buf[(last, y)].set_style(style);
        }
    }

    truncated
}

/// Render a bordered tooltip box containing `label` anchored at (`anchor_x`, `anchor_y`).
///
/// `direction` controls whether the tooltip opens to the right or left of the anchor.
/// `term_area` is the full terminal area used for clamping.
pub fn render_tooltip_box(
    buf: &mut Buffer,
    term_area: Rect,
    label: &str,
    anchor_x: u16,
    anchor_y: u16,
    direction: Direction,
) {
    let bounds = *buf.area();
    let label_width: u16 = label
        .chars()
        .map(|c| UnicodeWidthChar::width(c).unwrap_or(0))
        .sum::<usize>() as u16;
    let content_width = label_width + 2; // 1-char padding each side
    let box_height: u16 = 3; // top border + content + bottom border

    let (tooltip_x, box_width) = match direction {
        Direction::Right => {
            let max_w = term_area.width.saturating_sub(anchor_x);
            if max_w < 5 {
                return;
            }
            (anchor_x, (content_width + 2).min(max_w))
        }
        Direction::Left => {
            let total = (content_width + 2).min(anchor_x.saturating_sub(term_area.x));
            if total < 5 {
                return;
            }
            (anchor_x.saturating_sub(total), total)
        }
    };

    // Clamp to terminal bounds vertically
    let tooltip_y = if anchor_y + box_height > term_area.y + term_area.height {
        (term_area.y + term_area.height).saturating_sub(box_height)
    } else {
        anchor_y
    };

    if tooltip_y + box_height > term_area.y + term_area.height {
        return;
    }

    let border_style = Style::default().fg(theme::CYAN).bg(theme::BG);
    let text_style = Style::default().fg(theme::TEXT).bg(theme::BG);

    // Clear background
    for dy in 0..box_height {
        for dx in 0..box_width {
            set_cell_if(
                buf,
                &bounds,
                tooltip_x + dx,
                tooltip_y + dy,
                ' ',
                Style::default().bg(theme::BG),
            );
        }
    }

    // Top: ╭─...─╮
    set_cell_if(buf, &bounds, tooltip_x, tooltip_y, '╭', border_style);
    for dx in 1..box_width - 1 {
        set_cell_if(buf, &bounds, tooltip_x + dx, tooltip_y, '─', border_style);
    }
    set_cell_if(
        buf,
        &bounds,
        tooltip_x + box_width - 1,
        tooltip_y,
        '╮',
        border_style,
    );

    // Middle: │ text │
    let mid_y = tooltip_y + 1;
    set_cell_if(buf, &bounds, tooltip_x, mid_y, '│', border_style);
    set_cell_if(
        buf,
        &bounds,
        tooltip_x + box_width - 1,
        mid_y,
        '│',
        border_style,
    );

    // Fill middle row background
    for dx in 1..box_width - 1 {
        set_cell_if(buf, &bounds, tooltip_x + dx, mid_y, ' ', text_style);
    }

    // Write the label text (clipped to box interior)
    let text_clip = Rect::new(tooltip_x + 1, mid_y, box_width.saturating_sub(2), 1);
    write_str_clipped(
        buf,
        tooltip_x + 2,
        mid_y,
        label,
        text_style,
        &text_clip,
        false,
    );

    // Bottom: ╰─...─╯
    let bot_y = tooltip_y + 2;
    set_cell_if(buf, &bounds, tooltip_x, bot_y, '╰', border_style);
    for dx in 1..box_width - 1 {
        set_cell_if(buf, &bounds, tooltip_x + dx, bot_y, '─', border_style);
    }
    set_cell_if(
        buf,
        &bounds,
        tooltip_x + box_width - 1,
        bot_y,
        '╯',
        border_style,
    );
}
