use std::collections::HashMap;
use std::path::PathBuf;

use itertools::Itertools;

use crate::catalog_directory;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    // Example stuff:
    label: String,

    // this how you opt-out of serialization of a member
    #[serde(skip)]
    value: f32,
    #[serde(skip)]
    file_counts: HashMap<String, i128>,
    dropped_files: Vec<egui::DroppedFile>,
    picked_path: Option<PathBuf>,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            label: "Hello World!".to_owned(),
            value: 2.7,
            file_counts: HashMap::from([(String::from("No data"), 0),
                                        (String::from("No data"), 0),]),
            dropped_files: Vec::new(),
            picked_path: None,
        }
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customized the look at feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for TemplateApp {
    /// Called by the framework to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let Self { label, value, file_counts, dropped_files, picked_path} = self;

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        frame.quit();
                    }
                });
            });
        });

        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            ui.heading("Left Panel Title");

            if ui.button("Open folder...").clicked() {
                if let Some(path) = rfd::FileDialog::new()
                    .pick_folder() {
                        self.picked_path = Some(path);
                }
            }

            if let Some(picked_path) = &self.picked_path {
                ui.horizontal(|ui| {
                    ui.label("Picked file:");
                    ui.monospace(picked_path.display().to_string());
                });
            }

            if ui.button("Summarize").clicked() {
                *file_counts = catalog_directory(&self.picked_path.as_ref().unwrap());
            };

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 0.0;
                    ui.label("written with love by ");
                    ui.hyperlink_to("Brooke", "https://github.com/goingforbrooke");
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Central Panel Title");
            
            egui::warn_if_debug_build(ui);
            
            egui::Grid::new("test_table")
                .striped(true)
                .show(ui, |ui| {
                    ui.end_row();

                    for (extension, file_count) in file_counts.iter().sorted() {
                        ui.label(extension);
                        ui.label(file_count.to_string());
                        ui.end_row();
                    }

                    ui.end_row();
            });
        });
    }
}
