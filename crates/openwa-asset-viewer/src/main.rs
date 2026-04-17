use std::path::PathBuf;

use eframe::egui;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([800.0, 600.0]),
        ..Default::default()
    };
    eframe::run_native(
        "OpenWA Asset Viewer",
        options,
        Box::new(|_cc| Ok(Box::new(AssetViewer::default()))),
    )
}

#[derive(Default)]
struct AssetViewer {
    loaded_file: Option<PathBuf>,
}

impl eframe::App for AssetViewer {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Open…").clicked() {
                        if let Some(path) = rfd::FileDialog::new()
                            .add_filter("WA Assets", &["img", "dir", "spr", "bnk", "WAgame"])
                            .add_filter("All files", &["*"])
                            .pick_file()
                        {
                            self.loaded_file = Some(path);
                        }
                        ui.close_menu();
                    }
                    if ui.button("Quit").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            if let Some(path) = &self.loaded_file {
                ui.label(format!("Loaded: {}", path.display()));
            } else {
                ui.centered_and_justified(|ui| {
                    ui.label("Open a file via File → Open…");
                });
            }
        });
    }
}
