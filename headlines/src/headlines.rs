use std::borrow::Cow;

use eframe::egui::{
    Button, Color32, CtxRef, FontDefinitions, FontFamily, Hyperlink, Label, Layout, Separator,
    TextStyle, TopBottomPanel, Ui,
};

pub const PADDING: f32 = 5.0;
pub const WHITE: Color32 = Color32::from_rgb(255, 255, 255);
pub const CYAN: Color32 = Color32::from_rgb(0, 255, 255);

pub struct Headlines {
    articles: Vec<NewsCardData>,
}

impl Headlines {
    pub fn new() -> Self {
        let iter = (0..20).map(|a| NewsCardData {
            title: format!("title{}", a),
            desc: format!("desc{}", a),
            url: format!("https://example.com/{}", a),
        });

        Self {
            articles: Vec::from_iter(iter),
        }
    }

    pub fn configure_fonts(&self, ctx: &eframe::egui::CtxRef) {
        let mut font_def = FontDefinitions::default();
        font_def.font_data.insert(
            "MesloLGS".to_string(),
            Cow::Borrowed(include_bytes!("../../MesloLGS_NF_Regular.ttf")),
        );
        font_def.family_and_size.insert(
            eframe::egui::TextStyle::Heading,
            (FontFamily::Proportional, 35.),
        );
        font_def.family_and_size.insert(
            eframe::egui::TextStyle::Body,
            (FontFamily::Proportional, 20.),
        );
        font_def
            .fonts_for_family
            .get_mut(&FontFamily::Proportional)
            .unwrap()
            .insert(0, "MesloLGS".to_string());

        ctx.set_fonts(font_def)
    }

    pub fn render_news_cards(&self, ui: &mut eframe::egui::Ui) {
        for a in &self.articles {
            ui.add_space(PADDING);

            let title = format!("▶ {}", a.title);
            ui.colored_label(WHITE, title);

            ui.add_space(PADDING);
            let desc = Label::new(&a.desc).text_style(eframe::egui::TextStyle::Button);
            ui.add(desc);

            ui.style_mut().visuals.hyperlink_color = CYAN;
            ui.add_space(PADDING);
            ui.with_layout(Layout::right_to_left(), |ui| {
                ui.add(Hyperlink::new(&a.url).text("read more ⤴"));
            });

            ui.add_space(PADDING);
            ui.add(Separator::default());
        }
    }

    pub fn render_header(&self, ui: &mut Ui) {
        ui.vertical_centered(|ui| {
            ui.heading("Hacker News");
        });

        ui.add_space(PADDING);

        let sep = Separator::default().spacing(20.);
        ui.add(sep);
    }

    pub fn render_footer(&self, ctx: &CtxRef) {
        TopBottomPanel::bottom("footer").show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(10.);

                ui.add(Label::new("API source: newsapi.org").monospace());

                ui.add(
                    Hyperlink::new("https://github.com/emilk/egui")
                        .text("Made with egui")
                        .text_style(TextStyle::Monospace),
                );

                ui.add(
                    Hyperlink::new("https://github.com/ryanyogan/eguihn")
                        .text("ryanyogan/eguihn")
                        .text_style(TextStyle::Monospace),
                );

                ui.add_space(PADDING);
            });
        });
    }

    pub fn render_top_panel(&self, ctx: &CtxRef) {
        TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.add_space(10.);

            eframe::egui::menu::bar(ui, |ui| {
                // Logo
                ui.with_layout(Layout::left_to_right(), |ui| {
                    ui.add(Label::new("📰").text_style(eframe::egui::TextStyle::Heading));
                });

                // Controls
                ui.with_layout(Layout::right_to_left(), |ui| {
                    let close_button = ui.add(Button::new("❌").text_style(TextStyle::Body));
                    let refresh_button = ui.add(Button::new("🔁").text_style(TextStyle::Body));
                    let theme_button = ui.add(Button::new("🌙").text_style(TextStyle::Body));
                });
            });

            ui.add_space(10.);
        });
    }
}

struct NewsCardData {
    title: String,
    desc: String,
    url: String,
}
