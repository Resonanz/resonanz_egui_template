use eframe::egui;
use eframe::egui::{FontData, FontDefinitions, FontFamily};
use std::collections::BTreeMap;

pub fn setup_custom_fonts(ctx: &egui::Context) {
    let mut fonts = FontDefinitions::default();

    //
    // Load font =====================================
    //
    fonts.font_data.insert(
        "lora".to_owned(),
        FontData::from_static(include_bytes!(
            "../assets/fonts/Lora-Italic-VariableFont_wght.ttf"
        )),
    );

    let mut newfam = BTreeMap::new();
    newfam.insert(FontFamily::Name("lora".into()), vec!["lora".to_owned()]);
    fonts.families.append(&mut newfam);

    //
    // Load font =====================================
    //
    fonts.font_data.insert(
        "bungee".to_owned(),
        FontData::from_static(include_bytes!("../assets/fonts/BungeeTint-Regular.ttf")),
    );

    let mut newfam = BTreeMap::new();
    newfam.insert(FontFamily::Name("bungee".into()), vec!["bungee".to_owned()]);
    fonts.families.append(&mut newfam);

    //
    // Load font =====================================
    //
    fonts.font_data.insert(
        "seven_seg".to_owned(),
        FontData::from_static(include_bytes!(
            "../assets/fonts/NotoSansSymbols2-Regular.ttf"
        )),
    );

    let mut newfam = BTreeMap::new();
    newfam.insert(
        FontFamily::Name("seven_seg".into()),
        vec!["seven_seg".to_owned()],
    );
    fonts.families.append(&mut newfam);

    //
    // Set fonts =====================================
    //
    ctx.set_fonts(fonts);
}
