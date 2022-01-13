use eframe::{egui, epi};
use sysinfo::{ProcessExt, System, SystemExt};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "persistence", serde(default))] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    track_dialog_open: bool
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            track_dialog_open: false,
        }
    }
}

impl epi::App for TemplateApp {
    fn name(&self) -> &str {
        "eframe template"
    }

    /// Called once before the first frame.
    fn setup(
        &mut self,
        _ctx: &egui::CtxRef,
        _frame: &epi::Frame,
        _storage: Option<&dyn epi::Storage>,
    ) {
        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        #[cfg(feature = "persistence")]
        if let Some(storage) = _storage {
            *self = epi::get_value(storage, epi::APP_KEY).unwrap_or_default()
        }
    }

    /// Called by the frame work to save state before shutdown.
    /// Note that you must enable the `persistence` feature for this to work.
    #[cfg(feature = "persistence")]
    fn save(&mut self, storage: &mut dyn epi::Storage) {
        epi::set_value(storage, epi::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::CtxRef, frame: &epi::Frame) {
        // let Self { label, value } = self;

        // Examples of how to create different panels and windows.
        // Pick whichever suits you.
        // Tip: a good default choice is to just keep the `CentralPanel`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                ui.menu_button("Timer", |ui| {
                    if ui.button("Track application...").clicked() {
                        self.track_dialog_open = true;
                    }
                    ui.separator();
                    if ui.button("Quit").clicked() {
                        frame.quit();
                    }
                });
            });
        });

        if self.track_dialog_open {
            egui::Window::new("Add tracked application").show(ctx, |ui| {
                let sys = System::new_all();
                for (_, process) in sys.processes() {
                    process.uid == sys.
                    ui.label(process.name());
                }
                if ui.button("Track").clicked() {
                    self.track_dialog_open = false;
                }
            });
        }
    }
}
