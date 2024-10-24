use eframe::egui;
use egui_commonmark::{CommonMarkCache, CommonMarkViewer};

use crate::demos;

enum SlideMarker {
    CounterExample,
    UiExample,
    ToggleWidget,
    IdClashesExample,
    UnknownMarker(String),
    DancingStrings,
    Painting,
}

struct Slide {
    markdown: String,
    markers: Vec<SlideMarker>,
}

impl Slide {
    pub fn new(text: &str) -> Self {
        use itertools::Itertools as _;
        let mut markers = vec![];
        let markdown = text
            .trim()
            .lines()
            .filter(|line| {
                if let Some(marker_str) = line.strip_prefix("!!!") {
                    let marker = match marker_str.trim() {
                        "counter_example" => SlideMarker::CounterExample,
                        "ui_example" => SlideMarker::UiExample,
                        "toggle_widget" => SlideMarker::ToggleWidget,
                        "id_clashes" => SlideMarker::IdClashesExample,
                        "dancing_strings" => SlideMarker::DancingStrings,
                        "painting" => SlideMarker::Painting,
                        _ => {
                            log::warn!("Unknown slide marker: {marker_str:?}");
                            SlideMarker::UnknownMarker(marker_str.to_owned())
                        }
                    };
                    markers.push(marker);
                    false
                } else {
                    true
                }
            })
            .join("\n");

        Self {
            markdown: markdown.to_owned(),
            markers,
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Presentation {
    #[serde(skip)]
    cm_cache: CommonMarkCache,
    #[serde(skip)]
    slides: Vec<Slide>,
    slide_nr: usize,
    counter: i32,
    some_bool: bool,
    #[serde(skip)]
    painting: crate::demos::painting::Painting,
}

impl eframe::App for Presentation {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.ui(ui);
        });
    }
}

impl Default for Presentation {
    fn default() -> Self {
        let slides = include_str!("../slides.md");
        let slides = slides.split("\n-------------------------------------------------------------------------------\n").map(Slide::new).collect::<Vec<_>>();

        Self {
            cm_cache: Default::default(),
            slides,
            slide_nr: 0,
            counter: 0,
            some_bool: false,
            painting: demos::painting::Painting::default(),
        }
    }
}

impl Presentation {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Load in images
        cc.egui_ctx
            .include_bytes("bytes://crates.png", include_bytes!("../images/crates.png"));
        cc.egui_ctx.include_bytes(
            "bytes://wasm-compilers.png",
            include_bytes!("../images/wasm-compile.png"),
        );
        cc.egui_ctx.include_bytes(
            "bytes://rust-wasm.png",
            include_bytes!("../images/rust_wa.png"),
        );
        cc.egui_ctx
            .include_bytes("bytes://hennge.png", include_bytes!("../images/hennge.png"));
        cc.egui_ctx.include_bytes(
            "bytes://ffmpeg-wasm.png",
            include_bytes!("../images/ffmpeg_wasm.png"),
        );
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Self::default()
    }

    pub fn ui(&mut self, ui: &mut egui::Ui) {
        let Self {
            cm_cache,
            slides,
            slide_nr,

            counter,
            some_bool,
            painting,
        } = self;

        ui.input(|i| {
            if i.key_pressed(egui::Key::ArrowRight) || i.key_pressed(egui::Key::Space) {
                *slide_nr = (*slide_nr + 1) % slides.len();
            }
            if i.key_pressed(egui::Key::ArrowLeft) {
                *slide_nr = (*slide_nr + slides.len() - 1) % slides.len();
            }
        });

        let preview_slide_nr = *slide_nr;

        let slide = &slides[preview_slide_nr];
        let Slide { markdown, markers } = slide;
        println!("{}", markdown);
        egui::CentralPanel::default()
            .frame(egui::Frame::none().fill(egui::Color32::WHITE))
            .show(ui.ctx(), |ui| {
                ui.horizontal(|ui| {
                    ui.add_space(20.0);
                    ui.vertical(|ui| {
                        CommonMarkViewer::new()
                            .max_image_width(Some(ui.available_width().floor() as _))
                            .show(ui, cm_cache, markdown);
                        for marker in markers {
                            slider_marker_ui(ui, marker, counter, some_bool, painting);
                        }
                    });
                    ui.add_space(20.0);
                });
            });

        egui::TopBottomPanel::bottom("logo_panel")
            .min_height(80.0)
            .show_separator_line(false)
            .frame(egui::Frame::none())
            .show_inside(ui, |ui| {
                ui.with_layout(
                    egui::Layout::centered_and_justified(egui::Direction::LeftToRight),
                    |ui| {
                        ui.add(
                            egui::Image::new("bytes://hennge.png")
                                .max_width(ui.available_width() / 4.0)
                                .rounding(10.0),
                        );
                    },
                );
            });
    }
}

fn slider_marker_ui(
    ui: &mut egui::Ui,
    marker: &SlideMarker,
    counter: &mut i32,
    some_bool: &mut bool,
    painting: &mut demos::painting::Painting,
) {
    match marker {
        SlideMarker::CounterExample => {
            ui.horizontal(|ui| {
                if ui.button("-").clicked() {
                    *counter -= 1;
                }

                ui.label(counter.to_string());

                if ui.button("+").clicked() {
                    *counter += 1;
                }
            });
        }

        SlideMarker::UiExample => {
            ui.label("Some text");
            ui.horizontal(|ui| {
                ui.label("More");
                ui.label("text");
            });
            ui.label("Even more text");
        }

        SlideMarker::ToggleWidget => {
            toggle_widget(ui, some_bool);
        }

        SlideMarker::IdClashesExample => {
            ui.label("Ok, different names:");
            ui.collapsing("First header", |ui| {
                ui.label("Contents of first foldable ui");
            });
            ui.collapsing("Second header", |ui| {
                ui.label("Contents of second foldable ui");
            });

            ui.add_space(16.0);

            ui.label("Oh-no, same name = same id source:");
            ui.collapsing("Collapsing header", |ui| {
                ui.label("Contents of first foldable ui");
            });
            ui.collapsing("Collapsing header", |ui| {
                ui.label("Contents of second foldable ui");
            });
        }

        SlideMarker::UnknownMarker(marker) => {
            ui.label(
                egui::RichText::new(format!("⚠ Unknown slider marker: {marker:?} ⚠"))
                    .color(ui.visuals().warn_fg_color),
            );
        }

        SlideMarker::DancingStrings => {
            demos::dancing_strings::ui(ui);
        }

        SlideMarker::Painting => {
            painting.ui(ui);
        }
    }
}

fn toggle_widget(ui: &mut egui::Ui, on: &mut bool) -> egui::Response {
    let desired_size = ui.spacing().interact_size.y * egui::vec2(2.0, 1.0);
    let (rect, mut response) = ui.allocate_exact_size(desired_size, egui::Sense::click());
    if response.clicked() {
        *on = !*on;
        response.mark_changed();
    }
    response.widget_info(|| egui::WidgetInfo::selected(egui::WidgetType::Checkbox, true, *on, ""));

    if ui.is_rect_visible(rect) {
        let how_on = ui.ctx().animate_bool(response.id, *on);
        let visuals = ui.style().interact_selectable(&response, *on);
        let rect = rect.expand(visuals.expansion);
        let radius = 0.5 * rect.height();
        ui.painter()
            .rect(rect, radius, visuals.bg_fill, visuals.bg_stroke);
        let circle_x = egui::lerp((rect.left() + radius)..=(rect.right() - radius), how_on);
        let center = egui::pos2(circle_x, rect.center().y);
        ui.painter()
            .circle(center, 0.75 * radius, visuals.bg_fill, visuals.fg_stroke);
    }

    response
}
