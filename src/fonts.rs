use eframe::egui::FontDefinitions;

#[allow(clippy::too_many_lines)]
pub fn load() -> FontDefinitions {
    let mut fonts = FontDefinitions::default();

    fonts.font_data.insert(
        "ggsans_bold".to_owned(),
        eframe::egui::FontData::from_static(include_bytes!("../fonts/ggsans_bold.ttf")).into(),
    );

    fonts
        .families
        .get_mut(&eframe::egui::FontFamily::Proportional)
        .unwrap()
        .push("ggsans_bold".to_string());

    fonts.font_data.insert(
        "ggsans_bolditalic".to_owned(),
        eframe::egui::FontData::from_static(include_bytes!("../fonts/ggsans_bolditalic.ttf"))
            .into(),
    );

    fonts
        .families
        .get_mut(&eframe::egui::FontFamily::Proportional)
        .unwrap()
        .push("ggsans_bolditalic".to_string());

    fonts.font_data.insert(
        "ggsans_extrabold".to_owned(),
        eframe::egui::FontData::from_static(include_bytes!("../fonts/ggsans_extrabold.ttf")).into(),
    );

    fonts
        .families
        .get_mut(&eframe::egui::FontFamily::Proportional)
        .unwrap()
        .push("ggsans_extrabold".to_string());

    fonts.font_data.insert(
        "ggsans_extrabolditalic".to_owned(),
        eframe::egui::FontData::from_static(include_bytes!("../fonts/ggsans_extrabolditalic.ttf"))
            .into(),
    );

    fonts
        .families
        .get_mut(&eframe::egui::FontFamily::Proportional)
        .unwrap()
        .push("ggsans_extrabolditalic".to_string());

    fonts.font_data.insert(
        "ggsans_medium".to_owned(),
        eframe::egui::FontData::from_static(include_bytes!("../fonts/ggsans_medium.ttf")).into(),
    );

    fonts
        .families
        .get_mut(&eframe::egui::FontFamily::Proportional)
        .unwrap()
        .push("ggsans_medium".to_string());

    fonts.font_data.insert(
        "ggsans_mediumitalic".to_owned(),
        eframe::egui::FontData::from_static(include_bytes!("../fonts/ggsans_mediumitalic.ttf"))
            .into(),
    );

    fonts
        .families
        .get_mut(&eframe::egui::FontFamily::Proportional)
        .unwrap()
        .push("ggsans_mediumitalic".to_string());

    fonts.font_data.insert(
        "ggsans_normal".to_owned(),
        eframe::egui::FontData::from_static(include_bytes!("../fonts/ggsans_normal.ttf")).into(),
    );

    fonts
        .families
        .get_mut(&eframe::egui::FontFamily::Proportional)
        .unwrap()
        .push("ggsans_normal".to_string());

    fonts.font_data.insert(
        "ggsans_normalitalic".to_owned(),
        eframe::egui::FontData::from_static(include_bytes!("../fonts/ggsans_normalitalic.ttf"))
            .into(),
    );

    fonts
        .families
        .get_mut(&eframe::egui::FontFamily::Proportional)
        .unwrap()
        .push("ggsans_normalitalic".to_string());

    fonts.font_data.insert(
        "ggsans_semibold".to_owned(),
        eframe::egui::FontData::from_static(include_bytes!("../fonts/ggsans_semibold.ttf")).into(),
    );

    fonts
        .families
        .get_mut(&eframe::egui::FontFamily::Proportional)
        .unwrap()
        .insert(0, "ggsans_semibold".to_string());

    fonts.font_data.insert(
        "ggsans_semibolditalic".to_owned(),
        eframe::egui::FontData::from_static(include_bytes!("../fonts/ggsans_semibolditalic.ttf"))
            .into(),
    );

    fonts
        .families
        .get_mut(&eframe::egui::FontFamily::Proportional)
        .unwrap()
        .push("ggsans_semibolditalic".to_string());

    fonts.font_data.insert(
        "seguihis".to_owned(),
        eframe::egui::FontData::from_static(include_bytes!("../fonts/seguihis.ttf")).into(),
    );

    fonts
        .families
        .get_mut(&eframe::egui::FontFamily::Proportional)
        .unwrap()
        .push("seguihis".to_string());

    fonts.font_data.insert(
        "seguisym".to_owned(),
        eframe::egui::FontData::from_static(include_bytes!("../fonts/seguisym.ttf")).into(),
    );

    fonts
        .families
        .get_mut(&eframe::egui::FontFamily::Proportional)
        .unwrap()
        .push("seguisym".to_string());

    fonts
}
