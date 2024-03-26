#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;
use egui::{
    plot::{Line, Plot, Points, Value, Values},
    Color32,
};
use rand::distributions::{Distribution, Uniform};
use rand::prelude::*;

fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "My egui App",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    );
}

struct Data {
    line1: Line,      // cos function
    line2: Line,      // sin function
    scatter1: Points, // cos scatter points
    scatter2: Points, // sin scatter points
}

struct MyApp {
    plot_dataset: bool,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            plot_dataset: false,
        }
    }
}

fn get_data() -> Data {
    let public_bytes: Vec<u8> = Vec::from_iter(0..32);
    let public_bytes: [u8; 32] = public_bytes.try_into().unwrap();
    let seeded_rng1 = StdRng::from_seed(public_bytes);
    let seeded_rng2 = StdRng::from_seed(public_bytes);

    let sample = Uniform::from(0.0..1.0);
    let epsilon = Uniform::from(-0.1..0.1);
    let mut rng1 = StdRng::from_rng(seeded_rng1).unwrap();
    let mut rng2 = StdRng::from_rng(seeded_rng2).unwrap();

    let sin = (0..100).map(|i| {
        let x = (i as f64) * 0.01 * std::f64::consts::PI * 2.0;
        Value::new(x, x.sin())
    });
    let cos = (0..100).map(|i| {
        let x = (i as f64) * 0.01 * std::f64::consts::PI * 2.0;
        Value::new(x, x.cos())
    });

    let sin_data = (0..50).map(|_| {
        let x: f64 = sample.sample(&mut rng1) * std::f64::consts::PI * 2.0;
        Value::new(x, x.sin() + epsilon.sample(&mut rng1))
    });
    let cos_data = (0..50).map(|_| {
        let x: f64 = sample.sample(&mut rng2) * std::f64::consts::PI * 2.0;
        Value::new(x, x.cos() + epsilon.sample(&mut rng2))
    });

    let line1 = Line::new(Values::from_values_iter(sin))
        .width(3.0)
        .color(Color32::RED);
    let line2 = Line::new(Values::from_values_iter(cos))
        .width(3.0)
        .color(Color32::BLUE);
    let scatter1 = Points::new(Values::from_values_iter(sin_data))
        .radius(3.0)
        .shape(egui::plot::MarkerShape::Circle)
        .color(Color32::LIGHT_RED);
    let scatter2 = Points::new(Values::from_values_iter(cos_data))
        .radius(3.0)
        .shape(egui::plot::MarkerShape::Circle)
        .color(Color32::LIGHT_BLUE);

    Data {
        line1,
        line2,
        scatter1,
        scatter2,
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("My egui Application");
            if ui.button("Plot dataset").clicked() {
                self.plot_dataset = !self.plot_dataset;
            }

            let data = get_data();

            let plot = Plot::new("Data Plot");

            if self.plot_dataset {
                plot.view_aspect(2.0).show(ui, |plot_ui| {
                    plot_ui.line(data.line1);
                    plot_ui.line(data.line2);
                    plot_ui.points(data.scatter1);
                    plot_ui.points(data.scatter2);
                });
            } else {
                plot.view_aspect(2.0).show(ui, |plot_ui| {
                    plot_ui.line(data.line1);
                    plot_ui.line(data.line2);
                });
            }
        });
    }
}
