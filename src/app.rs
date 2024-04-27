use std::sync::OnceLock;

use base64::Engine;
use egui::{ahash::HashSet, RichText};

use crate::constants::IS_WEB;

#[derive(Default, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
enum ConverstionSelected {
    #[default]
    Encode,
    Decode,
}

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct Base64Encoder {
    selected: ConverstionSelected,
    input: String,
    output: String,
}

// TODO: replace with LazyLock When it comes to stable
static VALID_CHARS: OnceLock<HashSet<char>> = OnceLock::new();
impl Base64Encoder {
    const ENGINE: base64::engine::GeneralPurpose = base64::engine::general_purpose::STANDARD;

    // TODO: replace with LazyLock When it comes to stable
    fn get_valid_chars() -> &'static HashSet<char> {
        VALID_CHARS.get_or_init(|| {
            let i = base64::alphabet::STANDARD.as_str().chars();
            HashSet::from_iter(i)
        })
    }
}

impl Default for Base64Encoder {
    fn default() -> Self {
        Self {
            selected: ConverstionSelected::Encode,
            input: "Hello World!".to_owned(),
            output: "".to_owned(),
        }
    }
}

impl Base64Encoder {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for Base64Encoder {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui
        let Self {
            selected,
            input,
            output,
        } = self;

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::menu::bar(ui, |ui| {
                // NOTE: no File->Quit on web pages!
                if !IS_WEB {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                    ui.add_space(16.0);
                }

                egui::widgets::global_dark_light_mode_buttons(ui);
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            ui.horizontal(|ui| {
                ui.heading("Base64");
                match selected {
                    ConverstionSelected::Encode => {
                        ui.heading(RichText::new("Encoder").strong());
                        ui.heading("/ Decoder");
                    }
                    ConverstionSelected::Decode => {
                        ui.heading("Encoder /");
                        ui.heading(RichText::new("Decoder").strong());
                    }
                }
            });
            // ui.heading("Base64 Encoder / Decoder");

            // NOTE: select the Converstion type
            let converstion_input_resp = ui.horizontal(|ui| {
                ui.label("Converstion: ");
                egui::ComboBox::from_label("Select Converstion")
                    .selected_text(format!("{selected:?}"))
                    .show_ui(ui, |ui| {
                        [
                            ui.selectable_value(selected, ConverstionSelected::Encode, "Encode"),
                            ui.selectable_value(selected, ConverstionSelected::Decode, "Decode"),
                        ]
                    })
            });

            ui.label("Input");
            let input_resp = egui::ScrollArea::vertical().show(ui, |ui| {
                ui.add(
                    egui::TextEdit::multiline(input)
                        .font(egui::TextStyle::Monospace) // for cursor height
                        .code_editor()
                        .hint_text("Type something!")
                        .desired_rows(10)
                        .lock_focus(false)
                        .desired_width(f32::INFINITY), // .layouter(&mut layouter),
                )
            });

            ui.separator();

            ui.label("Output");
            egui::ScrollArea::vertical().show(ui, |ui| {
                let converstion_changed = converstion_input_resp
                    .inner
                    .inner
                    .is_some_and(|i| i.iter().any(|r| r.changed()));

                if converstion_changed {
                    std::mem::swap(input, output);
                }

                if input_resp.inner.changed() || converstion_changed {
                    let code = input.trim();
                    *output = match selected {
                        ConverstionSelected::Encode => Self::ENGINE.encode(code),
                        ConverstionSelected::Decode => {
                            // let valid_chars: Arc<HashSet<char>> = VALID_CHARS.get_or_init(|| {
                            //     let i = base64::alphabet::STANDARD.as_str().chars();
                            //     HashSet::from_iter(i).into()
                            // });
                            let code: String =
                                code.chars().filter(|c| Self::get_valid_chars().contains(c)).collect();
                            Self::ENGINE
                                .decode(code)
                                .inspect_err(|e| eprintln!("Failed to decode base64: {e}"))
                                .ok()
                                .map(|i| String::from_utf8_lossy(&i).to_string())
                                .unwrap_or("Invalid Input".to_string())
                        }
                    };
                }
                ui.add(
                    egui::TextEdit::multiline(output)
                        .font(egui::TextStyle::Monospace) // for cursor height
                        .desired_rows(10)
                        .desired_width(f32::INFINITY), // .layouter(&mut layouter),
                );
            });

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                powered_by_egui_and_eframe(ui);
                egui::warn_if_debug_build(ui);
            });
        });
    }
}

fn powered_by_egui_and_eframe(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label("Powered by ");
        ui.hyperlink_to("egui", "https://github.com/emilk/egui");
        ui.label(" and ");
        ui.hyperlink_to(
            "eframe",
            "https://github.com/emilk/egui/tree/master/crates/eframe",
        );
        ui.label(".");
    });
}
