use egui::plot::{Line, PlotPoints};
use egui::widgets::plot::Plot;

struct Sinusoid {
    amp: f64,
    freq: f64,
    phase_offset: f64,
}

pub struct SignalApp {
    i: Sinusoid,
    q: Sinusoid,
}

impl Default for Sinusoid {
    fn default() -> Self {
        Self {
            amp: 1.0,
            freq: 1.0,
            phase_offset: 0.0,
        }
    }
}

impl Default for SignalApp {
    fn default() -> Self {
        Self {
            i: Default::default(),
            q: Default::default(),
        }
    }
}

fn i(a: f64, f: f64, t: f64, phi: f64) -> f64 {
    a * (2.0 * std::f64::consts::PI * f * t + phi).sin()
}

fn q(a: f64, f: f64, t: f64, phi: f64) -> f64 {
    a * (2.0 * std::f64::consts::PI * f * t + phi).cos()
}

impl SignalApp {
    /// Called once before the first frame.
    pub fn new(_: &eframe::CreationContext<'_>) -> Self {
        Default::default()
    }

    fn i_points(&self) -> PlotPoints {
        let n = 2048;
        let x0 = -2.0f64;
        let x1 = 2.0 * std::f64::consts::PI;
        let dx = (x1 - x0) / (n as f64);
        let points: PlotPoints = (0..=n)
            .map(|t| {
                let xi = x0 + (t as f64 * dx);
                [xi, i(self.i.amp, self.i.freq, xi, self.i.phase_offset)]
            })
            .collect();
        points
    }

    fn q_points(&self) -> PlotPoints {
        let n = 2048;
        let x0 = -2.0f64;
        let x1 = 2.0 * std::f64::consts::PI;
        let dx = (x1 - x0) / (n as f64);
        let points: PlotPoints = (0..=n)
            .map(|t| {
                let xi = x0 + (t as f64 * dx);
                [xi, q(self.q.amp, self.q.freq, xi, self.q.phase_offset)]
            })
            .collect();
        points
    }

    fn i_line(&self) -> Line {
        let points = self.i_points();
        Line::new(points)
            .style(egui::plot::LineStyle::Solid)
            .color(egui::Color32::RED)
            .name("I")
    }

    fn q_line(&self) -> Line {
        let points = self.q_points();
        Line::new(points)
            .style(egui::plot::LineStyle::Solid)
            .color(egui::Color32::BLUE)
            .name("Q")
    }

    fn iq_points(&self) -> PlotPoints {
        let n = 2048;
        let x0 = -2.0f64;
        let x1 = 2.0 * std::f64::consts::PI;
        let dx = (x1 - x0) / (n as f64);
        let points: PlotPoints = (0..=n)
            .map(|t| {
                let xi = x0 + (t as f64 * dx);
                [
                    xi,
                    i(self.i.amp, self.i.freq, xi, self.i.phase_offset)
                        + q(self.q.amp, self.q.freq, xi, self.q.phase_offset),
                ]
            })
            .collect();
        points
    }

    fn iq_line(&self) -> Line {
        Line::new(self.iq_points())
            .style(egui::plot::LineStyle::Solid)
            .color(egui::Color32::BLACK)
            .name("IQ")
    }
}

impl eframe::App for SignalApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Spec A");
            ui.add(
                egui::DragValue::new(&mut self.i.amp)
                    .speed(0.1)
                    .prefix("I A: "),
            );
            ui.add(
                egui::DragValue::new(&mut self.i.freq)
                    .speed(1.0)
                    .prefix("I f: "),
            );
            ui.add(
                egui::DragValue::new(&mut self.i.phase_offset)
                    .speed(0.1)
                    .prefix("I phi: "),
            );
            ui.add(
                egui::DragValue::new(&mut self.q.amp)
                    .speed(0.1)
                    .prefix("Q A: "),
            );
            ui.add(
                egui::DragValue::new(&mut self.q.freq)
                    .speed(1.0)
                    .prefix("Q f: "),
            );
            ui.add(
                egui::DragValue::new(&mut self.q.phase_offset)
                    .speed(0.1)
                    .prefix("Q phi: "),
            );
            Plot::new("I").show(ui, |plot_ui| {
                plot_ui.line(self.i_line());
                plot_ui.line(self.q_line());
                plot_ui.line(self.iq_line());
            });
            egui::warn_if_debug_build(ui);
        });
    }
}
