use crate::SharedData;
use eframe::egui::{self};
use std::sync::{Arc, Mutex};

pub struct Kitten {
    data: Arc<Mutex<SharedData>>,
}

impl Kitten {
    pub fn new(cc: &eframe::CreationContext<'_>, shared_data: Arc<Mutex<SharedData>>) -> Self {
        cc.egui_ctx.set_visuals(egui::Visuals::dark());

        Self {
            data: shared_data.clone(),
        }
    }
}

impl eframe::App for Kitten {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let counter = self.data.lock().unwrap().counter;

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Hello World!");
            ui.heading(counter.to_string());
        });
    }
}
