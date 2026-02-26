use eframe::egui::FontDefinitions;

pub static GGSANS: &[u8] = include_bytes!("../fonts/ggsans_semibold.ttf");
pub static SEGUIHIS: &[u8] = include_bytes!("../fonts/seguihis.ttf");
pub static SEGUISYM: &[u8] = include_bytes!("../fonts/seguisym.ttf");
pub static NOTO_SANS_CHINENSE: &[u8] = include_bytes!("../fonts/NotoSansSC-SemiBold.ttf");
pub static NOTO_SANS_KOREAN: &[u8] = include_bytes!("../fonts/NotoSansKR-SemiBold.ttf");

pub fn load() -> FontDefinitions {
    let mut fonts = FontDefinitions::default();

    fonts.font_data.insert(
        "ggsans_semibold".to_owned(),
        eframe::egui::FontData::from_static(GGSANS).into(),
    );

    fonts
        .families
        .get_mut(&eframe::egui::FontFamily::Proportional)
        .unwrap()
        .insert(0, "ggsans_semibold".to_string());

    // ------------------------------------------------------------

    fonts.font_data.insert(
        "seguihis".to_owned(),
        eframe::egui::FontData::from_static(SEGUIHIS).into(),
    );

    fonts
        .families
        .get_mut(&eframe::egui::FontFamily::Proportional)
        .unwrap()
        .push("seguihis".to_string());

    // ------------------------------------------------------------

    fonts.font_data.insert(
        "seguisym".to_owned(),
        eframe::egui::FontData::from_static(SEGUISYM).into(),
    );

    fonts
        .families
        .get_mut(&eframe::egui::FontFamily::Proportional)
        .unwrap()
        .push("seguisym".to_string());

    // ------------------------------------------------------------

    fonts.font_data.insert(
        "noto-sans-sc".to_owned(),
        eframe::egui::FontData::from_static(NOTO_SANS_CHINENSE).into(),
    );

    fonts
        .families
        .get_mut(&eframe::egui::FontFamily::Proportional)
        .unwrap()
        .push("noto-sans-sc".to_string());

    // ------------------------------------------------------------

    fonts.font_data.insert(
        "noto-sans-kr".to_owned(),
        eframe::egui::FontData::from_static(NOTO_SANS_KOREAN).into(),
    );

    fonts
        .families
        .get_mut(&eframe::egui::FontFamily::Proportional)
        .unwrap()
        .push("noto-sans-kr".to_string());

    fonts
}
