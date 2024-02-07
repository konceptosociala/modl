use eframe::{egui, CreationContext};
use egui_modal::Modal;
use rfd::FileDialog;

use crate::i18n;

use self::config::Config;

pub mod config;
pub mod error;
pub mod i18n;

pub struct ModlApp {
    config: Config,
}

impl ModlApp {
    pub fn new(config: Config, _cc: &CreationContext) -> Self {
        ModlApp {
            config,
        }
    }
}

impl eframe::App for ModlApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let c = &self.config;

        egui::TopBottomPanel::top("top_menu").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button(i18n!(c, "menu_file"), |ui| {
                    if ui.button(i18n!(c, "menu_file_new")).clicked() {
                        let files = FileDialog::new()
                            .add_filter("text", &["txt", "rs"])
                            .add_filter("rust", &["rs", "toml"])
                            .set_directory("/")
                            .save_file();
                    }

                    if ui.button(i18n!(c, "menu_file_open")).clicked() {
                        todo!()
                    }

                    if ui.button(i18n!(c, "menu_file_save")).clicked() {
                        todo!()
                    }

                    if ui.button(i18n!(c, "menu_file_save_as")).clicked() {
                        todo!()
                    }

                    if ui.button(i18n!(c, "menu_file_import")).clicked() {
                        todo!()
                    }

                    if ui.button(i18n!(c, "menu_file_export")).clicked() {
                        todo!()
                    }

                    if ui.button(i18n!(c, "menu_file_exit")).clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                })
            });
        });
    }
}