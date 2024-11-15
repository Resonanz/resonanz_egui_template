#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui::{self, include_image, Color32, FontFamily, FontId, RichText, Vec2};
use std::thread;
use std::time::Duration;
mod fonts;

fn main() -> eframe::Result {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_min_inner_size([800., 600.])
            .with_max_inner_size([800., 600.])
            .with_resizable(false),
        ..Default::default()
    };

    eframe::run_native(
        "Resonanz egui Template",
        options,
        Box::new(|cc| {
            // This gives us image support (note that
            // egui_extras in cargo.toml needs all_loaders)
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Ok(Box::new(MyApp::new(cc)))
        }),
    )
}

// A place to store our data
pub struct MyApp {}

impl Default for MyApp {
    fn default() -> Self {
        Self {}
    }
}

impl MyApp {
    // Called once before the first frame.
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Load custom fonts
        fonts::setup_custom_fonts(&cc.egui_ctx);
        Self {}
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("I am default heading text");
            ui.label("I am default label text");
            ui.label(
                RichText::new("I am default RichText proportional size 40")
                    .font(FontId::proportional(40.)),
            );
            ui.label(RichText::new("I am default RichText RED").color(Color32::RED));
            if ui
                .add(egui::Button::new("Spawn new thread").min_size(Vec2::new(120., 32.)))
                .clicked()
            {
                thread::spawn(move || loop {
                    let mut count: u32 = 0;
                    loop {
                        println!("Hi from Ted the thread. I print to stdout every 1_000 ms. Print count = {count}.");
                        count += 1;
                        thread::sleep(Duration::from_millis(1_000));
                        }
                });
            }
            ui.add(egui::Button::new(
                egui::RichText::new("Hello I'm a RichText bungee button").font(FontId {
                    size: 30.,
                    family: FontFamily::Name("bungee".into()),
                }),
            ));
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
            ui.add(
                egui::Image::new(include_image!("../assets/pics/ferris.png"))
                    .fit_to_exact_size(Vec2::new(50., 50.)),
            );
            // This image will scale to fit
            ui.image(egui::include_image!("../assets/pics/ferris.png"));
        });
    }
}
