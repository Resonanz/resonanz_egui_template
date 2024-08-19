#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui::{self, include_image, Color32, FontFamily, FontId, RichText, Vec2};
mod helpers;

fn main() -> eframe::Result {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([800.0, 600.0]),
        ..Default::default()
    };

    eframe::run_native(
        "My egui App",
        options,
        Box::new(|cc| {
            // This gives us image support:
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Ok(Box::new(MyApp::new(cc)))
        }),
    )
}

struct MyApp {}

impl MyApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        helpers::setup_custom_fonts(&cc.egui_ctx);
        Self {}
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("I am default heading text");
            ui.label("I am default label text");
            ui.label(
                RichText::new("I am default RichText proportional").font(FontId::proportional(40.)),
            );
            ui.label(RichText::new("I am default RichText RED").color(Color32::RED));
            ui.label(
                egui::RichText::new("I am Lora Italic")
                    .color(egui::Color32::YELLOW)
                    .font(FontId {
                        size: 40.0,
                        family: FontFamily::Name("lora".into()),
                    }),
            );
            ui.label(
                egui::RichText::new("I am Bungee with background")
                    .color(egui::Color32::YELLOW)
                    .background_color(Color32::GRAY)
                    .font(FontId {
                        size: 40.0,
                        family: FontFamily::Name("bungee".into()),
                    }),
            );
            ui.label(
                egui::RichText::new("I am Nota Sans Symbols2 🯰🯱🯲🯳🯴🯵🯶🯷🯸🯹")
                    .color(egui::Color32::YELLOW)
                    .font(FontId {
                        size: 30.0,
                        family: FontFamily::Name("seven_seg".into()),
                    }),
            );
            // This image has defined size
            ui.add(egui::Image::new(
                include_image!("../assets/pics/ferris.png")).fit_to_exact_size(Vec2::new(50., 50.)),  
            );
            // This image will scale to fit
            ui.image(egui::include_image!("../assets/pics/ferris.png"));
        });
    }
}
