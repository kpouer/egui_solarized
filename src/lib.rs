/// Solarized theme for [egui](egui).
use egui::style::WidgetVisuals;
use egui::{Color32, Style, Visuals};
use std::sync::Arc;

pub const BASE03: Color32 = Color32::from_rgb(0x00, 0x2b, 0x36);
pub const BASE02: Color32 = Color32::from_rgb(0x07, 0x36, 0x42);
pub const BASE01: Color32 = Color32::from_rgb(0x58, 0x6e, 0x75);
pub const BASE00: Color32 = Color32::from_rgb(0x65, 0x7b, 0x83);
pub const BASE0: Color32 = Color32::from_rgb(0x83, 0x94, 0x96);
pub const BASE1: Color32 = Color32::from_rgb(0x93, 0xa1, 0xa1);
pub const BASE2: Color32 = Color32::from_rgb(0xee, 0xe8, 0xd5);
pub const BASE3: Color32 = Color32::from_rgb(0xfd, 0xf6, 0xe3);
pub const YELLOW: Color32 = Color32::from_rgb(0xb5, 0x89, 0x00);
pub const ORANGE: Color32 = Color32::from_rgb(0xcb, 0x4b, 0x16);
pub const RED: Color32 = Color32::from_rgb(0xdc, 0x32, 0x2f);
pub const MAGENTA: Color32 = Color32::from_rgb(0xd3, 0x36, 0x82);
pub const VIOLET: Color32 = Color32::from_rgb(0x6c, 0x71, 0xc4);
pub const BLUE: Color32 = Color32::from_rgb(0x26, 0x8b, 0xd2);
pub const CYAN: Color32 = Color32::from_rgb(0x2a, 0xa1, 0x98);
pub const GREEN: Color32 = Color32::from_rgb(0x85, 0x99, 0x00);

/// List of accent colors
pub static ACCENT_COLORS: [Color32; 8] = [YELLOW, ORANGE, RED, MAGENTA, VIOLET, BLUE, CYAN, GREEN];

/// Install the theme into the context
pub fn install(ctx: &egui::Context) {
    ctx.options_mut(|options| {
        options.dark_style = Arc::new(Style {
            visuals: Theme::solarized_dark().into(),
            ..Default::default()
        })
    });
    ctx.options_mut(|options| {
        options.light_style = Arc::new(Style {
            visuals: Theme::solarized_light().into(),
            ..Default::default()
        })
    });
}

#[derive(Debug)]
pub struct Theme {
    pub dark: bool,
    pub background: Color32,
    pub background_highlight: Color32,
    pub background_faint: Color32,
    pub text: Color32,
    pub text_emphasis: Color32,
    pub yellow: Color32,
    pub orange: Color32,
    pub red: Color32,
    pub magenta: Color32,
    pub violet: Color32,
    pub blue: Color32,
    pub cyan: Color32,
    pub green: Color32,
}

impl Default for Theme {
    fn default() -> Self {
        Self::solarized_dark()
    }
}

impl From<egui::Theme> for Theme {
    fn from(theme: egui::Theme) -> Self {
        match theme {
            egui::Theme::Dark => Theme::solarized_dark(),
            egui::Theme::Light => Theme::solarized_light(),
        }
    }
}

impl From<Theme> for Visuals {
    fn from(theme: Theme) -> Self {
        Visuals::from(&theme)
    }
}

impl From<&Theme> for Visuals {
    fn from(theme: &Theme) -> Visuals {
        let mut visuals = if theme.dark {
            Visuals::dark()
        } else {
            Visuals::light()
        };
        let shadow_color = if theme.dark {
            Color32::from_black_alpha(96)
        } else {
            Color32::from_black_alpha(25)
        };
        visuals.override_text_color = Some(theme.text);
        visuals.hyperlink_color = theme.blue;
        visuals.faint_bg_color = theme.background_faint;
        visuals.extreme_bg_color = theme.background;
        visuals.code_bg_color = theme.background;
        visuals.warn_fg_color = theme.orange;
        visuals.error_fg_color = theme.red;
        visuals.window_fill = theme.background;
        visuals.panel_fill = theme.background;
        visuals.window_stroke.color = theme.blue;
        update_widget_visual(&mut visuals.widgets.noninteractive, theme, theme.background);
        update_widget_visual(&mut visuals.widgets.inactive, theme, theme.background_faint);
        update_widget_visual(
            &mut visuals.widgets.hovered,
            theme,
            theme.background_highlight,
        );
        update_widget_visual(&mut visuals.widgets.active, theme, theme.text_emphasis);
        update_widget_visual(&mut visuals.widgets.open, theme, theme.background_faint);
        visuals.selection.bg_fill = theme
            .blue
            .linear_multiply(if theme.dark { 0.2 } else { 0.4 });
        visuals.selection.stroke.color = theme.blue;

        visuals.window_shadow.color = shadow_color;
        visuals.popup_shadow.color = shadow_color;
        visuals.dark_mode = theme.dark;
        visuals
    }
}

fn update_widget_visual(widget_visuals: &mut WidgetVisuals, theme: &Theme, bg_fill: Color32) {
    widget_visuals.bg_fill = bg_fill;
    widget_visuals.weak_bg_fill = bg_fill;
    widget_visuals.bg_stroke.color = theme.blue;
    widget_visuals.fg_stroke.color = theme.text;
}

impl Theme {
    pub fn solarized_dark() -> Theme {
        Theme {
            dark: true,
            background: BASE03,
            background_highlight: BASE02,
            background_faint: BASE02,
            text: BASE0,
            text_emphasis: BASE1,
            yellow: YELLOW,
            orange: ORANGE,
            red: RED,
            magenta: MAGENTA,
            violet: VIOLET,
            blue: BLUE,
            cyan: CYAN,
            green: GREEN,
        }
    }

    pub fn solarized_light() -> Theme {
        Theme {
            dark: true,
            background: BASE3,
            background_faint: BASE2,
            background_highlight: BASE2,
            text: BASE00,
            text_emphasis: BASE01,
            yellow: YELLOW,
            orange: ORANGE,
            red: RED,
            magenta: MAGENTA,
            violet: VIOLET,
            blue: BLUE,
            cyan: CYAN,
            green: GREEN,
        }
    }
}
