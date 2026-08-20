use ratatui::style::{Color, Modifier, Style};

// Monochrome gray palette — intentionally unremarkable.
// All UI is gray-on-gray; emphasis is expressed via brightness only.
pub const BG: Color = Color::Rgb(18, 18, 18);
pub const SIDEBAR_BG: Color = Color::Rgb(24, 24, 24);
pub const CHAT_BG: Color = Color::Rgb(22, 22, 22);
pub const CYAN: Color = Color::Rgb(160, 160, 160);
pub const MAGENTA: Color = Color::Rgb(140, 140, 140);
pub const GREEN: Color = Color::Rgb(150, 150, 150);
pub const RED: Color = Color::Rgb(175, 175, 175);
pub const TEXT: Color = Color::Rgb(205, 205, 205);
pub const DIM: Color = Color::Rgb(125, 125, 125);
pub const BORDER: Color = Color::Rgb(85, 85, 85);
pub const BLACK: Color = Color::Rgb(20, 20, 20);

// Mode indicator colors
pub const NORMAL_MODE_BG: Color = Color::Rgb(110, 110, 110);
pub const INSERT_MODE_BG: Color = Color::Rgb(150, 150, 150);
pub const COMMAND_MODE_BG: Color = Color::Rgb(95, 95, 95);

// Sender name palette (rotating gray shades, no hue)
pub const SENDER_COLORS: &[Color] = &[
    Color::Rgb(160, 160, 160),
    Color::Rgb(140, 140, 140),
    Color::Rgb(180, 180, 180),
    Color::Rgb(120, 120, 120),
    Color::Rgb(195, 195, 195),
    Color::Rgb(135, 135, 135),
    Color::Rgb(170, 170, 170),
    Color::Rgb(110, 110, 110),
];

pub fn sender_color(sender: &str) -> Color {
    let hash: usize = sender.bytes().map(|b| b as usize).sum();
    SENDER_COLORS[hash % SENDER_COLORS.len()]
}

// Semantic colors
pub const HIGHLIGHT_BG: Color = Color::Rgb(40, 40, 40);
pub const MUTED: Color = Color::Rgb(70, 70, 70);
pub const BAR_EMPTY: Color = Color::Rgb(80, 80, 80);
pub const METER_EMPTY: Color = Color::Rgb(45, 45, 45);
pub const MESSAGE_SELECT_BG: Color = Color::Rgb(48, 48, 48);
pub const REPLY_INDICATOR: Color = Color::Rgb(150, 150, 150);
pub const REACTION_BG: Color = Color::Rgb(42, 42, 42);
pub const REACTION_OWN_BG: Color = Color::Rgb(75, 75, 75);
pub const EDIT_INDICATOR: Color = Color::Rgb(165, 165, 165);

// Gradient endpoints — same gray at both ends so gradients render as flat,
// static gray (no animated color sweep).
pub const GRADIENT_BORDER_START: Color = Color::Rgb(110, 110, 110);
pub const GRADIENT_BORDER_END: Color = Color::Rgb(110, 110, 110);
pub const GRADIENT_TITLE_END: Color = Color::Rgb(160, 160, 160);
pub const GRADIENT_HIGHLIGHT_START: Color = Color::Rgb(50, 50, 50);
pub const GRADIENT_HIGHLIGHT_END: Color = Color::Rgb(50, 50, 50);
pub const GRADIENT_DATE_BRIGHT: Color = Color::Rgb(120, 120, 120);
pub const GRADIENT_DATE_DIM: Color = Color::Rgb(70, 70, 70);
pub const STATUS_BAR_BG: Color = Color::Rgb(26, 26, 26);
pub const UNREAD_BADGE_BG: Color = Color::Rgb(80, 80, 80);
pub const TIMESTAMP_BRIGHT: Color = Color::Rgb(120, 120, 120);
pub const INPUT_BORDER_CYAN_DIM: Color = Color::Rgb(95, 95, 95);
pub const INPUT_BORDER_MAGENTA_DIM: Color = Color::Rgb(95, 95, 95);

// Rich text / formatted message colors
pub const CODE_INLINE_FG: Color = Color::Rgb(175, 175, 175);
pub const CODE_INLINE_BG: Color = Color::Rgb(40, 40, 40);
pub const CODE_BLOCK_BG: Color = Color::Rgb(26, 26, 26);
pub const LINK_FG: Color = Color::Rgb(160, 160, 160);
pub const BLOCKQUOTE_FG: Color = Color::Rgb(120, 120, 120);

// Composite styles
pub fn border_style() -> Style {
    Style::default().fg(BORDER)
}

pub fn border_focused_style() -> Style {
    Style::default().fg(CYAN)
}

pub fn title_style() -> Style {
    Style::default().fg(CYAN).add_modifier(Modifier::BOLD)
}

pub fn text_style() -> Style {
    Style::default().fg(TEXT)
}

pub fn dim_style() -> Style {
    Style::default().fg(DIM)
}

pub fn error_style() -> Style {
    Style::default().fg(RED)
}

pub fn dim_italic_style() -> Style {
    Style::default().fg(DIM).add_modifier(Modifier::ITALIC)
}

// Form field styles
pub fn field_marker_style(selected: bool) -> Style {
    let color = if selected { CYAN } else { DIM };
    Style::default().fg(color).bg(BG)
}

pub fn field_label_style(selected: bool) -> Style {
    let color = if selected { CYAN } else { TEXT };
    let modifier = if selected {
        Modifier::BOLD
    } else {
        Modifier::empty()
    };
    Style::default().fg(color).bg(BG).add_modifier(modifier)
}

pub fn field_value_style(selected: bool) -> Style {
    let color = if selected { TEXT } else { DIM };
    Style::default().fg(color).bg(BG)
}

pub fn field_arrow_style(selected: bool) -> Style {
    let color = if selected { CYAN } else { DIM };
    Style::default().fg(color).bg(BG)
}

pub fn edit_text_style() -> Style {
    Style::default().fg(GREEN).bg(BG)
}

pub fn edit_cursor_style() -> Style {
    Style::default().fg(GREEN).bg(BG)
}

pub fn highlight_focused_style() -> Style {
    Style::default()
        .fg(BLACK)
        .bg(CYAN)
        .add_modifier(Modifier::BOLD)
}

pub fn highlight_unfocused_style() -> Style {
    Style::default().fg(CYAN).bg(HIGHLIGHT_BG)
}

pub fn message_selected_style() -> Style {
    Style::default().bg(MESSAGE_SELECT_BG)
}

pub fn reply_indicator_style() -> Style {
    Style::default().fg(REPLY_INDICATOR)
}

pub fn reaction_badge_style() -> Style {
    Style::default().fg(TEXT).bg(REACTION_BG)
}

pub fn reaction_own_badge_style() -> Style {
    Style::default().fg(CYAN).bg(REACTION_OWN_BG)
}

pub fn edit_indicator_style() -> Style {
    Style::default()
        .fg(EDIT_INDICATOR)
        .add_modifier(Modifier::BOLD)
}

pub fn code_inline_style() -> Style {
    Style::default().fg(CODE_INLINE_FG).bg(CODE_INLINE_BG)
}

pub fn code_block_style() -> Style {
    Style::default().fg(TEXT).bg(CODE_BLOCK_BG)
}

pub fn link_style() -> Style {
    Style::default()
        .fg(LINK_FG)
        .add_modifier(Modifier::UNDERLINED)
}

pub fn blockquote_style() -> Style {
    Style::default().fg(BLOCKQUOTE_FG)
}

pub fn loading_style() -> Style {
    Style::default().fg(CYAN).bg(BG)
}

pub fn saving_style() -> Style {
    Style::default()
        .fg(GREEN)
        .bg(BG)
        .add_modifier(Modifier::BOLD)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sender_color_deterministic() {
        let c1 = sender_color("@alice:matrix.org");
        let c2 = sender_color("@alice:matrix.org");
        assert_eq!(c1, c2);
    }

    #[test]
    fn sender_color_different_inputs() {
        let c1 = sender_color("@alice:matrix.org");
        let c2 = sender_color("@bob:matrix.org");
        // Different inputs should produce results (may or may not be different
        // colors due to hash collisions, but the function should not panic)
        let _ = (c1, c2);
    }

    #[test]
    fn sender_color_empty_string() {
        let c = sender_color("");
        // 0 % 8 == 0, so should return SENDER_COLORS[0] which is CYAN
        assert_eq!(c, SENDER_COLORS[0]);
    }

    #[test]
    fn sender_color_returns_from_palette() {
        let c = sender_color("test_user");
        assert!(SENDER_COLORS.contains(&c));
    }
}
