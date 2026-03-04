use ratatui::style::Color;

/// AntiRaid color theme for the TUI
/// Based on the CSS color schemes from Badger
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Theme {
    pub name: &'static str,
    pub background: Color,
    pub foreground: Color,
    pub card: Color,
    pub card_foreground: Color,
    pub primary: Color,
    pub primary_foreground: Color,
    pub secondary: Color,
    pub secondary_foreground: Color,
    pub extra: Color,
    pub extra_foreground: Color,
    pub muted: Color,
    pub muted_foreground: Color,
    pub accent: Color,
    pub accent_foreground: Color,
    pub destructive: Color,
    pub destructive_foreground: Color,
    pub border: Color,
    pub input: Color,
    pub ring: Color,
}

impl Theme {
    /// Dark Red theme - The sick red aesthetic (default)
    pub fn dark_red() -> Self {
        Self {
            name: "Dark Red",
            background: hsl_to_rgb(350, 60, 6),
            foreground: hsl_to_rgb(350, 15, 98),
            card: hsl_to_rgb(350, 50, 8),
            card_foreground: hsl_to_rgb(350, 15, 98),
            primary: hsl_to_rgb(355, 95, 55),
            primary_foreground: hsl_to_rgb(350, 15, 98),
            secondary: hsl_to_rgb(350, 40, 15),
            secondary_foreground: hsl_to_rgb(350, 15, 98),
            extra: hsl_to_rgb(330, 90, 65),
            extra_foreground: hsl_to_rgb(350, 15, 98),
            muted: hsl_to_rgb(350, 40, 15),
            muted_foreground: hsl_to_rgb(350, 25, 70),
            accent: hsl_to_rgb(355, 70, 22),
            accent_foreground: hsl_to_rgb(350, 15, 98),
            destructive: hsl_to_rgb(0, 95, 65),
            destructive_foreground: hsl_to_rgb(0, 0, 98),
            border: hsl_to_rgb(350, 50, 12),
            input: hsl_to_rgb(350, 40, 18),
            ring: hsl_to_rgb(355, 95, 55),
        }
    }

    /// Electric Purple theme - Purple vibes
    pub fn electric_purple() -> Self {
        Self {
            name: "Electric Purple",
            background: hsl_to_rgb(280, 70, 5),
            foreground: hsl_to_rgb(280, 20, 98),
            card: hsl_to_rgb(280, 60, 7),
            card_foreground: hsl_to_rgb(280, 20, 98),
            primary: hsl_to_rgb(275, 100, 60),
            primary_foreground: hsl_to_rgb(280, 20, 98),
            secondary: hsl_to_rgb(280, 50, 12),
            secondary_foreground: hsl_to_rgb(280, 20, 98),
            extra: hsl_to_rgb(290, 90, 70),
            extra_foreground: hsl_to_rgb(280, 20, 98),
            muted: hsl_to_rgb(280, 40, 16),
            muted_foreground: hsl_to_rgb(280, 30, 75),
            accent: hsl_to_rgb(275, 75, 20),
            accent_foreground: hsl_to_rgb(280, 20, 98),
            destructive: hsl_to_rgb(0, 95, 65),
            destructive_foreground: hsl_to_rgb(0, 0, 98),
            border: hsl_to_rgb(280, 50, 10),
            input: hsl_to_rgb(280, 40, 14),
            ring: hsl_to_rgb(275, 100, 60),
        }
    }

    /// Dark Blue theme - Blue aesthetic
    pub fn dark_blue() -> Self {
        Self {
            name: "Dark Blue",
            background: hsl_to_rgb(225, 60, 4),
            foreground: hsl_to_rgb(215, 30, 98),
            card: hsl_to_rgb(225, 65, 6),
            card_foreground: hsl_to_rgb(215, 30, 98),
            primary: hsl_to_rgb(220, 95, 50),
            primary_foreground: hsl_to_rgb(215, 30, 98),
            secondary: hsl_to_rgb(225, 60, 10),
            secondary_foreground: hsl_to_rgb(215, 30, 98),
            extra: hsl_to_rgb(200, 90, 60),
            extra_foreground: hsl_to_rgb(215, 30, 98),
            muted: hsl_to_rgb(225, 50, 14),
            muted_foreground: hsl_to_rgb(225, 30, 70),
            accent: hsl_to_rgb(220, 75, 18),
            accent_foreground: hsl_to_rgb(215, 30, 98),
            destructive: hsl_to_rgb(0, 95, 65),
            destructive_foreground: hsl_to_rgb(0, 0, 98),
            border: hsl_to_rgb(225, 60, 10),
            input: hsl_to_rgb(225, 50, 14),
            ring: hsl_to_rgb(220, 95, 50),
        }
    }

    /// Dark Green theme - Green aesthetic
    pub fn dark_green() -> Self {
        Self {
            name: "Dark Green",
            background: hsl_to_rgb(160, 70, 3),
            foreground: hsl_to_rgb(150, 15, 98),
            card: hsl_to_rgb(160, 65, 5),
            card_foreground: hsl_to_rgb(150, 15, 98),
            primary: hsl_to_rgb(155, 95, 40),
            primary_foreground: hsl_to_rgb(150, 15, 98),
            secondary: hsl_to_rgb(160, 50, 10),
            secondary_foreground: hsl_to_rgb(150, 15, 98),
            extra: hsl_to_rgb(170, 90, 50),
            extra_foreground: hsl_to_rgb(150, 15, 98),
            muted: hsl_to_rgb(160, 45, 12),
            muted_foreground: hsl_to_rgb(160, 25, 70),
            accent: hsl_to_rgb(155, 70, 15),
            accent_foreground: hsl_to_rgb(150, 15, 98),
            destructive: hsl_to_rgb(0, 95, 65),
            destructive_foreground: hsl_to_rgb(0, 0, 98),
            border: hsl_to_rgb(160, 50, 8),
            input: hsl_to_rgb(160, 45, 12),
            ring: hsl_to_rgb(155, 95, 40),
        }
    }

    /// Zen Dark theme - Clean slate gray aesthetic
    pub fn zen_dark() -> Self {
        Self {
            name: "Zen Dark",
            background: hsl_to_rgb(220, 15, 8),
            foreground: hsl_to_rgb(220, 10, 95),
            card: hsl_to_rgb(220, 15, 12),
            card_foreground: hsl_to_rgb(220, 10, 95),
            primary: hsl_to_rgb(200, 20, 60), // Muted Blue
            primary_foreground: hsl_to_rgb(220, 10, 98),
            secondary: hsl_to_rgb(220, 15, 20),
            secondary_foreground: hsl_to_rgb(220, 10, 95),
            extra: hsl_to_rgb(180, 20, 70), // Muted Teal
            extra_foreground: hsl_to_rgb(220, 10, 95),
            muted: hsl_to_rgb(220, 10, 25),
            muted_foreground: hsl_to_rgb(220, 10, 60),
            accent: hsl_to_rgb(330, 20, 40), // Muted Rose
            accent_foreground: hsl_to_rgb(220, 10, 98),
            destructive: hsl_to_rgb(0, 95, 65),
            destructive_foreground: hsl_to_rgb(0, 0, 98),
            border: hsl_to_rgb(220, 15, 15),
            input: hsl_to_rgb(220, 15, 18),
            ring: hsl_to_rgb(200, 20, 60),
        }
    }

    /// Dark theme - Original dark purple
    pub fn dark() -> Self {
        Self {
            name: "Dark",
            background: hsl_to_rgb(263, 92, 5),
            foreground: hsl_to_rgb(0, 0, 98),
            card: hsl_to_rgb(300, 6, 7),
            card_foreground: hsl_to_rgb(0, 0, 98),
            primary: hsl_to_rgb(281, 100, 37),
            primary_foreground: hsl_to_rgb(0, 0, 98),
            secondary: hsl_to_rgb(270, 5, 15),
            secondary_foreground: hsl_to_rgb(0, 0, 98),
            extra: hsl_to_rgb(244, 84, 61),
            extra_foreground: hsl_to_rgb(0, 0, 5),
            muted: hsl_to_rgb(280, 5, 22),
            muted_foreground: hsl_to_rgb(280, 5, 49),
            accent: hsl_to_rgb(282, 45, 14),
            accent_foreground: hsl_to_rgb(0, 0, 98),
            destructive: hsl_to_rgb(0, 84, 60),
            destructive_foreground: hsl_to_rgb(0, 0, 98),
            border: hsl_to_rgb(280, 5, 12),
            input: hsl_to_rgb(285, 4, 18),
            ring: hsl_to_rgb(281, 100, 40),
        }
    }

    /// Cyberpunk theme - Neon vibes
    pub fn cyberpunk() -> Self {
        Self {
            name: "Cyberpunk",
            background: hsl_to_rgb(260, 60, 4),
            foreground: hsl_to_rgb(180, 100, 90),
            card: hsl_to_rgb(260, 70, 7),
            card_foreground: hsl_to_rgb(180, 100, 90),
            primary: hsl_to_rgb(320, 100, 60), // Neon Pink
            primary_foreground: hsl_to_rgb(260, 60, 4),
            secondary: hsl_to_rgb(180, 100, 50), // Cyan
            secondary_foreground: hsl_to_rgb(260, 60, 4),
            extra: hsl_to_rgb(60, 100, 50), // Yellow
            extra_foreground: hsl_to_rgb(260, 60, 4),
            muted: hsl_to_rgb(260, 40, 15),
            muted_foreground: hsl_to_rgb(260, 20, 60),
            accent: hsl_to_rgb(280, 80, 30),
            accent_foreground: hsl_to_rgb(180, 100, 90),
            destructive: hsl_to_rgb(0, 100, 60),
            destructive_foreground: hsl_to_rgb(0, 0, 98),
            border: hsl_to_rgb(320, 100, 60),
            input: hsl_to_rgb(260, 50, 12),
            ring: hsl_to_rgb(320, 100, 60),
        }
    }

    /// Midnight theme - Deep space vibes
    pub fn midnight() -> Self {
        Self {
            name: "Midnight",
            background: hsl_to_rgb(230, 80, 2),
            foreground: hsl_to_rgb(230, 20, 95),
            card: hsl_to_rgb(230, 70, 4),
            card_foreground: hsl_to_rgb(230, 20, 95),
            primary: hsl_to_rgb(210, 100, 70), // Light Blue
            primary_foreground: hsl_to_rgb(230, 80, 2),
            secondary: hsl_to_rgb(250, 80, 60), // Purple-ish
            secondary_foreground: hsl_to_rgb(230, 80, 2),
            extra: hsl_to_rgb(280, 100, 70), // Lavender
            extra_foreground: hsl_to_rgb(230, 80, 2),
            muted: hsl_to_rgb(230, 40, 10),
            muted_foreground: hsl_to_rgb(230, 20, 50),
            accent: hsl_to_rgb(210, 60, 25),
            accent_foreground: hsl_to_rgb(230, 20, 95),
            destructive: hsl_to_rgb(350, 100, 60),
            destructive_foreground: hsl_to_rgb(0, 0, 98),
            border: hsl_to_rgb(210, 100, 40),
            input: hsl_to_rgb(230, 60, 8),
            ring: hsl_to_rgb(210, 100, 70),
        }
    }

    /// Blue theme
    pub fn blue() -> Self {
        Self {
            name: "Blue",
            background: hsl_to_rgb(215, 55, 8),
            foreground: hsl_to_rgb(215, 30, 98),
            card: hsl_to_rgb(215, 60, 10),
            card_foreground: hsl_to_rgb(215, 30, 98),
            primary: hsl_to_rgb(210, 100, 60),
            primary_foreground: hsl_to_rgb(210, 30, 98),
            secondary: hsl_to_rgb(217, 50, 15),
            secondary_foreground: hsl_to_rgb(215, 30, 98),
            extra: hsl_to_rgb(195, 85, 65),
            extra_foreground: hsl_to_rgb(215, 30, 98),
            muted: hsl_to_rgb(215, 40, 20),
            muted_foreground: hsl_to_rgb(215, 25, 75),
            accent: hsl_to_rgb(210, 70, 22),
            accent_foreground: hsl_to_rgb(215, 30, 98),
            destructive: hsl_to_rgb(0, 95, 65),
            destructive_foreground: hsl_to_rgb(0, 0, 98),
            border: hsl_to_rgb(215, 50, 15),
            input: hsl_to_rgb(215, 40, 22),
            ring: hsl_to_rgb(210, 100, 60),
        }
    }

    /// Green theme
    pub fn green() -> Self {
        Self {
            name: "Green",
            background: hsl_to_rgb(160, 60, 7),
            foreground: hsl_to_rgb(150, 15, 98),
            card: hsl_to_rgb(160, 55, 9),
            card_foreground: hsl_to_rgb(150, 15, 98),
            primary: hsl_to_rgb(155, 85, 45),
            primary_foreground: hsl_to_rgb(150, 15, 98),
            secondary: hsl_to_rgb(160, 45, 14),
            secondary_foreground: hsl_to_rgb(150, 15, 98),
            extra: hsl_to_rgb(170, 85, 55),
            extra_foreground: hsl_to_rgb(150, 15, 98),
            muted: hsl_to_rgb(160, 40, 18),
            muted_foreground: hsl_to_rgb(160, 25, 75),
            accent: hsl_to_rgb(155, 65, 20),
            accent_foreground: hsl_to_rgb(150, 15, 98),
            destructive: hsl_to_rgb(0, 95, 65),
            destructive_foreground: hsl_to_rgb(0, 0, 98),
            border: hsl_to_rgb(160, 45, 12),
            input: hsl_to_rgb(160, 40, 16),
            ring: hsl_to_rgb(155, 85, 45),
        }
    }

    /// Get all available themes
    pub fn all() -> Vec<Self> {
        vec![
            Self::zen_dark(),
            Self::dark_red(),
            Self::electric_purple(),
            Self::cyberpunk(),
            Self::midnight(),
            Self::dark_blue(),
            Self::dark_green(),
            Self::dark(),
            Self::blue(),
            Self::green(),
        ]
    }

    /// Get the default theme (Zen Dark)
    pub fn default() -> Self {
        Self::zen_dark()
    }

    /// Get the next theme in the cycle
    #[allow(dead_code)]
    pub fn next(&self) -> Self {
        let themes = Self::all();
        let current_idx = themes.iter().position(|t| t.name == self.name).unwrap_or(0);
        let next_idx = (current_idx + 1) % themes.len();
        themes[next_idx]
    }

    /// Get the previous theme in the cycle
    #[allow(dead_code)]
    pub fn prev(&self) -> Self {
        let themes = Self::all();
        let current_idx = themes.iter().position(|t| t.name == self.name).unwrap_or(0);
        let prev_idx = if current_idx == 0 {
            themes.len() - 1
        } else {
            current_idx - 1
        };
        themes[prev_idx]
    }
}

impl Default for Theme {
    fn default() -> Self {
        Self::default()
    }
}

/// Convert HSL to RGB color for Ratatui
/// H: 0-360, S: 0-100, L: 0-100
fn hsl_to_rgb(h: u16, s: u8, l: u8) -> Color {
    let h = h as f32 / 360.0;
    let s = s as f32 / 100.0;
    let l = l as f32 / 100.0;

    let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
    let x = c * (1.0 - ((h * 6.0) % 2.0 - 1.0).abs());
    let m = l - c / 2.0;

    let (r, g, b) = if h < 1.0 / 6.0 {
        (c, x, 0.0)
    } else if h < 2.0 / 6.0 {
        (x, c, 0.0)
    } else if h < 3.0 / 6.0 {
        (0.0, c, x)
    } else if h < 4.0 / 6.0 {
        (0.0, x, c)
    } else if h < 5.0 / 6.0 {
        (x, 0.0, c)
    } else {
        (c, 0.0, x)
    };

    let r = ((r + m) * 255.0) as u8;
    let g = ((g + m) * 255.0) as u8;
    let b = ((b + m) * 255.0) as u8;

    Color::Rgb(r, g, b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_theme_cycling() {
        let theme = Theme::dark_red();
        let next = theme.next();
        assert_eq!(next.name, "Electric Purple");

        let prev = next.prev();
        assert_eq!(prev.name, "Dark Red");
    }

    #[test]
    fn test_all_themes() {
        let themes = Theme::all();
        assert!(themes.len() >= 7);
        assert!(themes.iter().any(|t| t.name == "Dark Red"));
        assert!(themes.iter().any(|t| t.name == "Electric Purple"));
    }
}
