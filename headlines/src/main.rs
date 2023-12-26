mod headlines;

use eframe::{
    egui::{CentralPanel, ScrollArea, Vec2, Visuals},
    epi::App,
    run_native, NativeOptions,
};
use headlines::{Headlines, NewsCardData};
use newsapi::NewsAPI;

fn fetch_news(api_key: &str, articles: &mut Vec<NewsCardData>) {
    if let Ok(response) = NewsAPI::new(api_key).fetch() {
        let response_articles = response.articles();

        for article in response_articles.iter() {
            let news = NewsCardData {
                title: article.title().to_string(),
                url: article.url().to_string(),
                desc: article
                    .desc()
                    .map(|s| s.to_string())
                    .unwrap_or("...".to_string()),
            };
            articles.push(news);
        }
    }
}

impl App for Headlines {
    fn setup(
        &mut self,
        ctx: &eframe::egui::CtxRef,
        _frame: &mut eframe::epi::Frame<'_>,
        _storage: Option<&dyn eframe::epi::Storage>,
    ) {
        fetch_news(&self.config.api_key, &mut self.articles);
        self.configure_fonts(ctx);
    }

    fn update(&mut self, ctx: &eframe::egui::CtxRef, frame: &mut eframe::epi::Frame<'_>) {
        if self.config.dark_mode {
            ctx.set_visuals(Visuals::dark());
        } else {
            ctx.set_visuals(Visuals::light());
        }

        if !self.api_key_initialized {
            self.render_config(ctx);
        } else {
            self.render_top_panel(ctx, frame);

            CentralPanel::default().show(ctx, |ui| {
                self.render_header(ui);

                ScrollArea::auto_sized().show(ui, |ui| {
                    self.render_news_cards(ui);
                });

                self.render_footer(ctx);
            });
        }
    }

    fn name(&self) -> &str {
        "Headlines"
    }
}

fn main() {
    tracing_subscriber::fmt::init();

    let mut win_options = NativeOptions::default();
    win_options.initial_window_size = Some(Vec2::new(540., 960.));

    let app = Headlines::new();

    run_native(Box::new(app), win_options)
}
