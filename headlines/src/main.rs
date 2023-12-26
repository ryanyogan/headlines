mod headlines;

use eframe::{
    egui::{CentralPanel, ScrollArea, Vec2, Visuals},
    epi::App,
    run_native, NativeOptions,
};
use headlines::Headlines;

impl App for Headlines {
    fn setup(
        &mut self,
        ctx: &eframe::egui::CtxRef,
        _frame: &mut eframe::epi::Frame<'_>,
        _storage: Option<&dyn eframe::epi::Storage>,
    ) {
        self.configure_fonts(ctx);
    }

    fn update(&mut self, ctx: &eframe::egui::CtxRef, frame: &mut eframe::epi::Frame<'_>) {
        if self.config.dark_mode {
            ctx.set_visuals(Visuals::dark());
        } else {
            ctx.set_visuals(Visuals::light());
        }

        self.render_config(ctx);
        self.render_top_panel(ctx, frame);

        CentralPanel::default().show(ctx, |ui| {
            self.render_header(ui);

            ScrollArea::auto_sized().show(ui, |ui| {
                self.render_news_cards(ui);
            });

            self.render_footer(ctx);
        });
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
