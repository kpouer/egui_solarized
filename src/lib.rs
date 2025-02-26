use egui::{style, Color32, Visuals};

#[derive(Debug)]
pub struct Theme {
    pub dark: bool,
    pub background: Color32,
    pub background_highlight: Color32,
    pub background_faint: Color32,
    pub text: Color32,
    pub text_faint: Color32,
    pub info: Color32,
    pub warning: Color32,
    pub error: Color32,
    pub literal: Color32,
    pub operator: Color32,
    pub number: Color32,
    pub string: Color32,
    pub deleting: Color32,
}

impl Default for Theme {
    fn default() -> Self {
        Self::solarized_dark()
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
            egui::Color32::from_black_alpha(96)
        } else {
            egui::Color32::from_black_alpha(25)
        };
        visuals.override_text_color = Some(theme.text);
        visuals.hyperlink_color = theme.info;
        visuals.faint_bg_color = theme.background_faint;
        visuals.extreme_bg_color = theme.background;
        visuals.code_bg_color = theme.background;
        visuals.warn_fg_color = theme.warning;
        visuals.error_fg_color = theme.error;
        visuals.window_fill = theme.background;
        visuals.panel_fill = theme.background;
        visuals.window_stroke.color = theme.info;
        visuals.widgets.noninteractive= make_widget_visual(visuals.widgets.noninteractive, theme, theme.background);
        visuals.widgets.inactive= make_widget_visual(visuals.widgets.inactive, theme, theme.background_faint);
        visuals.widgets.hovered= make_widget_visual(visuals.widgets.hovered, theme, theme.background_highlight);
        visuals.widgets.active= make_widget_visual(visuals.widgets.active, theme, theme.text_faint);
        visuals.widgets.open= make_widget_visual(visuals.widgets.open, theme, theme.background_faint);
        visuals.selection.bg_fill = theme.info.linear_multiply(if theme.dark { 0.2 } else { 0.4 });
        visuals.selection.stroke.color = theme.info;

        visuals.window_shadow.color = shadow_color;
        visuals.popup_shadow.color = shadow_color;
        visuals.dark_mode = theme.dark;
        visuals
    }
}

fn make_widget_visual(
    old: style::WidgetVisuals,
    theme: &Theme,
    bg_fill: egui::Color32,
) -> style::WidgetVisuals {
    style::WidgetVisuals {
        bg_fill,
        weak_bg_fill: bg_fill,
        bg_stroke: egui::Stroke {
            color: theme.info,
            ..old.bg_stroke
        },
        fg_stroke: egui::Stroke {
            color: theme.text,
            ..old.fg_stroke
        },
        ..old
    }
}

static BASE03: Color32 = Color32::from_rgb(0x00, 0x2b, 0x36);
static BASE02: Color32 = Color32::from_rgb(0x07, 0x36, 0x42);
#[allow(dead_code)]
static BASE01: Color32 = Color32::from_rgb(0x58, 0x6e, 0x75);
#[allow(dead_code)]
static BASE00: Color32 = Color32::from_rgb(0x65, 0x7b, 0x83);
static BASE0: Color32 = Color32::from_rgb(0x83, 0x94, 0x96);
static BASE1: Color32 = Color32::from_rgb(0x93, 0xa1, 0xa1);
static BASE2: Color32 = Color32::from_rgb(0xee, 0xe8, 0xd5);
static BASE3: Color32 = Color32::from_rgb(0xfd, 0xf6, 0xe3);
static YELLOW: Color32 = Color32::from_rgb(0xb5, 0x89, 0x00);
static ORANGE: Color32 = Color32::from_rgb(0xcb, 0x4b, 0x16);
static RED: Color32 = Color32::from_rgb(0xdc, 0x32, 0x2f);
#[allow(dead_code)]
static MAGENTA: Color32 = Color32::from_rgb(0xd3, 0x36, 0x82);
#[allow(dead_code)]
static VIOLET: Color32 = Color32::from_rgb(0x6c, 0x71, 0xc4);
static BLUE: Color32 = Color32::from_rgb(0x26, 0x8b, 0xd2);
#[allow(dead_code)]
static CYAN: Color32 = Color32::from_rgb(0x2a, 0xa1, 0x98);
static GREEN: Color32 = Color32::from_rgb(0x85, 0x99, 0x00);

impl Theme {
    pub fn solarized_dark() -> Theme {
        Theme {
            dark: true,
            background: BASE03,
            background_faint: BASE02,
            background_highlight: BASE02,
            text: BASE0,
            text_faint: BASE1,
            info: BLUE,
            warning: ORANGE,
            error: RED,
            literal: BASE0,
            operator: BLUE,
            number: YELLOW,
            string: GREEN,
            deleting: RED,
        }
    }

    pub fn solarized_light() -> Theme {
        Theme {
            dark: true,
            background: BASE3,
            background_faint: BASE2,
            background_highlight: BASE2,
            text: BASE0,
            text_faint: BASE1,
            info: BLUE,
            warning: ORANGE,
            error: RED,
            literal: BASE0,
            operator: BLUE,
            number: YELLOW,
            string: GREEN,
            deleting: RED,
        }
    }
}
