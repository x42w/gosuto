use ratatui::{buffer::Buffer, layout::Rect, style::Style};

use super::cells::{display_width, set_cell, write_str_clipped};
use super::theme;

/// Direction the tooltip box opens relative to its anchor.
pub enum Direction {
    /// Tooltip opens to the right of the anchor (for room list).
    Right,
    /// Tooltip opens to the left of the anchor (for members pane).
    Left,
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
    let content_width = display_width(label) + 2; // 1-char padding each side
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
            set_cell(
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
    set_cell(buf, &bounds, tooltip_x, tooltip_y, '╭', border_style);
    for dx in 1..box_width - 1 {
        set_cell(buf, &bounds, tooltip_x + dx, tooltip_y, '─', border_style);
    }
    set_cell(
        buf,
        &bounds,
        tooltip_x + box_width - 1,
        tooltip_y,
        '╮',
        border_style,
    );

    // Middle: │ text │
    let mid_y = tooltip_y + 1;
    set_cell(buf, &bounds, tooltip_x, mid_y, '│', border_style);
    set_cell(
        buf,
        &bounds,
        tooltip_x + box_width - 1,
        mid_y,
        '│',
        border_style,
    );

    // Fill middle row background
    for dx in 1..box_width - 1 {
        set_cell(buf, &bounds, tooltip_x + dx, mid_y, ' ', text_style);
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
    set_cell(buf, &bounds, tooltip_x, bot_y, '╰', border_style);
    for dx in 1..box_width - 1 {
        set_cell(buf, &bounds, tooltip_x + dx, bot_y, '─', border_style);
    }
    set_cell(
        buf,
        &bounds,
        tooltip_x + box_width - 1,
        bot_y,
        '╯',
        border_style,
    );
}
