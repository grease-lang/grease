use dioxus::prelude::*;
use std::collections::HashMap;

/// Material 3 color scheme
#[derive(Clone, Debug, PartialEq)]
pub struct ColorScheme {
    pub primary: String,
    pub on_primary: String,
    pub primary_container: String,
    pub on_primary_container: String,
    pub secondary: String,
    pub on_secondary: String,
    pub secondary_container: String,
    pub on_secondary_container: String,
    pub tertiary: String,
    pub on_tertiary: String,
    pub tertiary_container: String,
    pub on_tertiary_container: String,
    pub error: String,
    pub on_error: String,
    pub error_container: String,
    pub on_error_container: String,
    pub background: String,
    pub on_background: String,
    pub surface: String,
    pub on_surface: String,
    pub surface_variant: String,
    pub on_surface_variant: String,
    pub outline: String,
    pub outline_variant: String,
    pub shadow: String,
    pub scrim: String,
    pub inverse_surface: String,
    pub inverse_on_surface: String,
    pub inverse_primary: String,
}

impl Default for ColorScheme {
    fn default() -> Self {
        Self::catppuccin_mocha()
    }
}

impl ColorScheme {
    /// Catppuccin Mocha theme (dark)
    pub fn catppuccin_mocha() -> Self {
        Self {
            primary: "#cba6f7".to_string(),
            on_primary: "#11111b".to_string(),
            primary_container: "#56526e".to_string(),
            on_primary_container: "#e6e0f9".to_string(),
            secondary: "#f2cdcd".to_string(),
            on_secondary: "#11111b".to_string(),
            secondary_container: "#56526e".to_string(),
            on_secondary_container: "#ffe4e1".to_string(),
            tertiary: "#eba0ac".to_string(),
            on_tertiary: "#11111b".to_string(),
            tertiary_container: "#6e5058".to_string(),
            on_tertiary_container: "#ffd9dc".to_string(),
            error: "#f38ba8".to_string(),
            on_error: "#11111b".to_string(),
            error_container: "#6e5058".to_string(),
            on_error_container: "#ffd9dc".to_string(),
            background: "#1e1e2e".to_string(),
            on_background: "#cdd6f4".to_string(),
            surface: "#1e1e2e".to_string(),
            on_surface: "#cdd6f4".to_string(),
            surface_variant: "#313244".to_string(),
            on_surface_variant: "#a6adc8".to_string(),
            outline: "#6c7086".to_string(),
            outline_variant: "#45475a".to_string(),
            shadow: "#000000".to_string(),
            scrim: "#000000".to_string(),
            inverse_surface: "#e6e0f9".to_string(),
            inverse_on_surface: "#31303d".to_string(),
            inverse_primary: "#6750a4".to_string(),
        }
    }

    /// Catppuccin Latte theme (light)
    pub fn catppuccin_latte() -> Self {
        Self {
            primary: "#8839ef".to_string(),
            on_primary: "#ffffff".to_string(),
            primary_container: "#eaddff".to_string(),
            on_primary_container: "#21005d".to_string(),
            secondary: "#625b71".to_string(),
            on_secondary: "#ffffff".to_string(),
            secondary_container: "#e8def8".to_string(),
            on_secondary_container: "#1d192b".to_string(),
            tertiary: "#7d5260".to_string(),
            on_tertiary: "#ffffff".to_string(),
            tertiary_container: "#ffd8e4".to_string(),
            on_tertiary_container: "#31111d".to_string(),
            error: "#ba1a1a".to_string(),
            on_error: "#ffffff".to_string(),
            error_container: "#ffdad6".to_string(),
            on_error_container: "#410002".to_string(),
            background: "#fffbfe".to_string(),
            on_background: "#1c1b1f".to_string(),
            surface: "#fffbfe".to_string(),
            on_surface: "#1c1b1f".to_string(),
            surface_variant: "#e7e0ec".to_string(),
            on_surface_variant: "#49454f".to_string(),
            outline: "#79747e".to_string(),
            outline_variant: "#cab6d0".to_string(),
            shadow: "#000000".to_string(),
            scrim: "#000000".to_string(),
            inverse_surface: "#313033".to_string(),
            inverse_on_surface: "#f4eff4".to_string(),
            inverse_primary: "#d0bcff".to_string(),
        }
    }

    /// Try to detect system theme and return appropriate color scheme
    pub fn detect_system_theme() -> Self {
        #[cfg(target_os = "linux")]
        {
            // Try to detect KDE Plasma theme
            if let Ok(output) = std::process::Command::new("kreadconfig5")
                .args(&["--group", "Colors:Window", "--key", "BackgroundNormal"])
                .output()
            {
                if output.status.success() {
                    let bg_color = String::from_utf8_lossy(&output.stdout);
                    if !bg_color.trim().is_empty() {
                        // Parse KDE color and determine if it's dark/light
                        return Self::from_kde_color(&bg_color);
                    }
                }
            }
        }

        #[cfg(target_family = "wasm")]
        {
            // Web environment - use prefers-color-scheme
            return Self::catppuccin_mocha(); // Default to dark for web
        }

        #[cfg(not(any(target_os = "linux", target_family = "wasm")))]
        {
            // Other platforms - default to dark theme
            return Self::catppuccin_mocha();
        }

        // Fallback to dark theme
        Self::catppuccin_mocha()
    }

    #[cfg(target_os = "linux")]
    fn from_kde_color(kde_color: &str) -> Self {
        // Simple heuristic: if the background is dark, use dark theme
        let color = kde_color.trim().trim_matches('#');
        if color.len() == 6 {
            if let Ok(r) = u8::from_str_radix(&color[0..2], 16) {
                if let Ok(g) = u8::from_str_radix(&color[2..4], 16) {
                    if let Ok(b) = u8::from_str_radix(&color[4..6], 16) {
                        // Calculate luminance
                        let luminance = (0.299 * r as f32 + 0.587 * g as f32 + 0.114 * b as f32) / 255.0;
                        if luminance < 0.5 {
                            return Self::catppuccin_mocha();
                        } else {
                            return Self::catppuccin_latte();
                        }
                    }
                }
            }
        }
        Self::catppuccin_mocha()
    }
}

/// Material 3 typography scale
#[derive(Clone, Debug, PartialEq)]
pub struct Typography {
    pub display_large: TextStyle,
    pub display_medium: TextStyle,
    pub display_small: TextStyle,
    pub headline_large: TextStyle,
    pub headline_medium: TextStyle,
    pub headline_small: TextStyle,
    pub title_large: TextStyle,
    pub title_medium: TextStyle,
    pub title_small: TextStyle,
    pub body_large: TextStyle,
    pub body_medium: TextStyle,
    pub body_small: TextStyle,
    pub label_large: TextStyle,
    pub label_medium: TextStyle,
    pub label_small: TextStyle,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TextStyle {
    pub font_size: String,
    pub font_weight: String,
    pub line_height: String,
    pub letter_spacing: String,
}

impl Default for Typography {
    fn default() -> Self {
        Self {
            display_large: TextStyle {
                font_size: "57px".to_string(),
                font_weight: "400".to_string(),
                line_height: "64px".to_string(),
                letter_spacing: "-0.25px".to_string(),
            },
            display_medium: TextStyle {
                font_size: "45px".to_string(),
                font_weight: "400".to_string(),
                line_height: "52px".to_string(),
                letter_spacing: "0px".to_string(),
            },
            display_small: TextStyle {
                font_size: "36px".to_string(),
                font_weight: "400".to_string(),
                line_height: "44px".to_string(),
                letter_spacing: "0px".to_string(),
            },
            headline_large: TextStyle {
                font_size: "32px".to_string(),
                font_weight: "400".to_string(),
                line_height: "40px".to_string(),
                letter_spacing: "0px".to_string(),
            },
            headline_medium: TextStyle {
                font_size: "28px".to_string(),
                font_weight: "400".to_string(),
                line_height: "36px".to_string(),
                letter_spacing: "0px".to_string(),
            },
            headline_small: TextStyle {
                font_size: "24px".to_string(),
                font_weight: "400".to_string(),
                line_height: "32px".to_string(),
                letter_spacing: "0px".to_string(),
            },
            title_large: TextStyle {
                font_size: "22px".to_string(),
                font_weight: "500".to_string(),
                line_height: "28px".to_string(),
                letter_spacing: "0px".to_string(),
            },
            title_medium: TextStyle {
                font_size: "16px".to_string(),
                font_weight: "500".to_string(),
                line_height: "24px".to_string(),
                letter_spacing: "0.15px".to_string(),
            },
            title_small: TextStyle {
                font_size: "14px".to_string(),
                font_weight: "500".to_string(),
                line_height: "20px".to_string(),
                letter_spacing: "0.1px".to_string(),
            },
            body_large: TextStyle {
                font_size: "16px".to_string(),
                font_weight: "400".to_string(),
                line_height: "24px".to_string(),
                letter_spacing: "0.5px".to_string(),
            },
            body_medium: TextStyle {
                font_size: "14px".to_string(),
                font_weight: "400".to_string(),
                line_height: "20px".to_string(),
                letter_spacing: "0.25px".to_string(),
            },
            body_small: TextStyle {
                font_size: "12px".to_string(),
                font_weight: "400".to_string(),
                line_height: "16px".to_string(),
                letter_spacing: "0.4px".to_string(),
            },
            label_large: TextStyle {
                font_size: "14px".to_string(),
                font_weight: "500".to_string(),
                line_height: "20px".to_string(),
                letter_spacing: "0.1px".to_string(),
            },
            label_medium: TextStyle {
                font_size: "12px".to_string(),
                font_weight: "500".to_string(),
                line_height: "16px".to_string(),
                letter_spacing: "0.5px".to_string(),
            },
            label_small: TextStyle {
                font_size: "11px".to_string(),
                font_weight: "500".to_string(),
                line_height: "16px".to_string(),
                letter_spacing: "0.5px".to_string(),
            },
        }
    }
}

/// Material 3 shape system
#[derive(Clone, Debug, PartialEq)]
pub struct Shape {
    pub corner_radius: String,
}

impl Shape {
    pub fn none() -> Self {
        Self {
            corner_radius: "0px".to_string(),
        }
    }

    pub fn extra_small() -> Self {
        Self {
            corner_radius: "4px".to_string(),
        }
    }

    pub fn small() -> Self {
        Self {
            corner_radius: "8px".to_string(),
        }
    }

    pub fn medium() -> Self {
        Self {
            corner_radius: "12px".to_string(),
        }
    }

    pub fn large() -> Self {
        Self {
            corner_radius: "16px".to_string(),
        }
    }

    pub fn extra_large() -> Self {
        Self {
            corner_radius: "28px".to_string(),
        }
    }
}

/// Material 3 elevation system
#[derive(Clone, Debug, PartialEq)]
pub struct Elevation {
    pub level: u8,
}

impl Elevation {
    pub fn level0() -> Self {
        Self { level: 0 }
    }

    pub fn level1() -> Self {
        Self { level: 1 }
    }

    pub fn level2() -> Self {
        Self { level: 2 }
    }

    pub fn level3() -> Self {
        Self { level: 3 }
    }

    pub fn level4() -> Self {
        Self { level: 4 }
    }

    pub fn level5() -> Self {
        Self { level: 5 }
    }

    pub fn box_shadow(&self) -> String {
        match self.level {
            0 => "none".to_string(),
            1 => "0px 1px 2px 0px rgba(0, 0, 0, 0.3), 0px 1px 3px 1px rgba(0, 0, 0, 0.15)".to_string(),
            2 => "0px 1px 2px 0px rgba(0, 0, 0, 0.3), 0px 2px 6px 2px rgba(0, 0, 0, 0.15)".to_string(),
            3 => "0px 1px 3px 0px rgba(0, 0, 0, 0.3), 0px 4px 8px 3px rgba(0, 0, 0, 0.15)".to_string(),
            4 => "0px 2px 3px 0px rgba(0, 0, 0, 0.3), 0px 6px 10px 4px rgba(0, 0, 0, 0.15)".to_string(),
            5 => "0px 4px 4px 0px rgba(0, 0, 0, 0.3), 0px 8px 12px 6px rgba(0, 0, 0, 0.15)".to_string(),
            _ => "none".to_string(),
        }
    }
}

/// Complete Material 3 theme
#[derive(Clone, Debug, PartialEq)]
pub struct Theme {
    pub colors: ColorScheme,
    pub typography: Typography,
    pub shape: Shape,
    pub elevation: Elevation,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            colors: ColorScheme::detect_system_theme(),
            typography: Typography::default(),
            shape: Shape::medium(),
            elevation: Elevation::level0(),
        }
    }
}

impl Theme {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_colors(mut self, colors: ColorScheme) -> Self {
        self.colors = colors;
        self
    }

    pub fn with_shape(mut self, shape: Shape) -> Self {
        self.shape = shape;
        self
    }

    pub fn catppuccin_mocha() -> Self {
        Self {
            colors: ColorScheme::catppuccin_mocha(),
            typography: Typography::default(),
            shape: Shape::medium(),
            elevation: Elevation::level0(),
        }
    }

    pub fn catppuccin_latte() -> Self {
        Self {
            colors: ColorScheme::catppuccin_latte(),
            typography: Typography::default(),
            shape: Shape::medium(),
            elevation: Elevation::level0(),
        }
    }
}

/// Theme provider component
#[component]
pub fn ThemeProvider(children: Element, theme: Option<Theme>) -> Element {
    let current_theme = use_signal(|| theme.unwrap_or_default());
    
    rsx! {
        div {
            style: "font-family: 'Roboto', -apple-system, BlinkMacSystemFont, 'Segoe UI', system-ui, sans-serif; \
                   background-color: {current_theme.read().colors.background}; \
                   color: {current_theme.read().colors.on_background}; \
                   min-height: 100vh; \
                   transition: all 0.2s ease-in-out;",
            
            {children}
        }
    }
}

/// Hook to access current theme
pub fn use_theme() -> Signal<Theme> {
    use_context::<Signal<Theme>>()
        .unwrap_or_else(|| use_signal(Theme::default))
}