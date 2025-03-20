#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(rustdoc::missing_crate_level_docs)] // it's an example
use crate::code_example::CodeExample;
use crate::widget_gallery::WidgetGallery;
use eframe::{App, Frame};
use egui::Context;
use egui_extras::install_image_loaders;
use egui_solarized::Theme;
use std::fmt::Display;

mod code_example;
mod toggle_switch;
mod widget_gallery;

fn main() -> eframe::Result {
    env_logger::init();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1280.0, 1024.0])
            .with_drag_and_drop(true),

        ..Default::default()
    };

    eframe::run_native(
        "egui demo app",
        options,
        Box::new(|ctx| {
            install_image_loaders(&ctx.egui_ctx);
            egui_solarized::install(&ctx.egui_ctx);
            Ok(Box::new(DemoApp::default()))
        }),
    )
}

struct DemoApp {
    widget_gallery: bool,
    code_example: bool,
    current_theme: SolarizedTheme,
}

impl Default for DemoApp {
    fn default() -> Self {
        Self {
            widget_gallery: true,
            code_example: true,
            current_theme: SolarizedTheme::default(),
        }
    }
}

#[derive(PartialEq, Default, Debug, Copy, Clone)]
enum SolarizedTheme {
    #[default]
    Dark,
    Light,
}

impl Display for SolarizedTheme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            SolarizedTheme::Dark => "Dark".to_owned(),
            SolarizedTheme::Light => "Light".to_owned(),
        };
        write!(f, "{}", str)
    }
}

impl App for DemoApp {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        WidgetGallery::default().show(ctx, &mut self.widget_gallery);
        CodeExample::default().show(ctx, &mut self.code_example);
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            let current_theme = &mut self.current_theme;
            egui::ComboBox::from_label("Choose a theme")
                .selected_text(format!("{current_theme:?}"))
                .show_ui(ui, |ui| {
                    if ui
                        .selectable_value(current_theme, SolarizedTheme::Light, "Light")
                        .clicked()
                    {
                        ctx.set_visuals(Theme::solarized_light().into());
                    }
                    if ui
                        .selectable_value(current_theme, SolarizedTheme::Dark, "Dark")
                        .clicked()
                    {
                        ctx.set_visuals(Theme::solarized_dark().into());
                    }
                });
        });
    }
}

/// Create a [`Hyperlink`](egui::Hyperlink) to this egui source code file on github.
#[macro_export]
macro_rules! egui_github_link_file {
    () => {
        $crate::egui_github_link_file!("(source code)")
    };
    ($label: expr) => {
        egui::github_link_file!(
            "https://github.com/kpouer/egui-solarized/blob/master/",
            egui::RichText::new($label).small()
        )
    };
}

/// View some Rust code with syntax highlighting and selection.
pub(crate) fn rust_view_ui(ui: &mut egui::Ui, code: &str) {
    let language = "rs";
    let theme = egui_extras::syntax_highlighting::CodeTheme::from_memory(ui.ctx(), ui.style());
    egui_extras::syntax_highlighting::code_view_ui(ui, &theme, code, language);
}

/// Something to view in the demo windows
pub trait View {
    fn ui(&mut self, ui: &mut egui::Ui);
}

/// Something to view
pub trait Demo {
    /// Is the demo enabled for this integration?
    fn is_enabled(&self, _ctx: &egui::Context) -> bool {
        true
    }

    /// `&'static` so we can also use it as a key to store open/close state.
    fn name(&self) -> &'static str;

    /// Show windows, etc
    fn show(&mut self, ctx: &egui::Context, open: &mut bool);
}
