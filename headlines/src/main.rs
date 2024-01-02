mod headlines;

use std::{
    sync::mpsc::{channel, sync_channel},
    thread,
};

use eframe::{
    egui::{CentralPanel, ScrollArea, Vec2, Visuals},
    epi::App,
    run_native, NativeOptions,
};
use headlines::{Headlines, Msg, NewsCardData};
use newsapi::NewsAPI;

fn fetch_news(api_key: &str, news_tx: &mut std::sync::mpsc::Sender<NewsCardData>) {
    if let Ok(response) = NewsAPI::new(&api_key).fetch() {
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

            if let Err(error) = news_tx.send(news) {
                tracing::error!("Error sending news data: {}", error);
            }
        }
    } else {
        tracing::error!("failed fetching news");
    }
}

impl App for Headlines {
    fn setup(
        &mut self,
        ctx: &eframe::egui::CtxRef,
        _frame: &mut eframe::epi::Frame<'_>,
        _storage: Option<&dyn eframe::epi::Storage>,
    ) {
        let api_key = self.config.api_key.to_string();

        let (mut news_tx, news_rx) = channel();
        let (app_tx, app_rx) = sync_channel(1);

        self.news_rx = Some(news_rx);
        self.app_tx = Some(app_tx);

        thread::spawn(move || {
            if !api_key.is_empty() {
                fetch_news(&api_key, &mut news_tx);
            } else {
                loop {
                    match app_rx.try_recv() {
                        Ok(Msg::ApiKeySet(api_key)) => {
                            fetch_news(&api_key, &mut news_tx);
                        }
                        Err(e) => {
                            tracing::error!("failed receiving msg: {}", e);
                        }
                    }
                }
            }
        });

        self.configure_fonts(ctx);
    }

    fn update(&mut self, ctx: &eframe::egui::CtxRef, frame: &mut eframe::epi::Frame<'_>) {
        ctx.request_repaint();

        if self.config.dark_mode {
            ctx.set_visuals(Visuals::dark());
        } else {
            ctx.set_visuals(Visuals::light());
        }

        if !self.api_key_initialized {
            self.render_config(ctx);
        } else {
            self.preload_articles();

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
