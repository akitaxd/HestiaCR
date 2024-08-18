use egui::{CentralPanel, ViewportCommand};
use crate::collection;
use crate::module::ModuleCollection;

#[derive(Default)]
pub struct UI {
    pub visible:bool,
}

impl eframe::App for UI {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        custom_window_frame(ctx, "hestia", |ui| unsafe {
            let collection_wrapper = collection.as_ref().unwrap();
            let mut lock = collection_wrapper.lock().unwrap();
            egui::CollapsingHeader::new("Combat")
                .default_open(false)
                .show(ui, |ui| {
                    egui::CollapsingHeader::new("Trigger Bot")
                        .default_open(false).show(ui,|ui| {
                        ui.checkbox(&mut lock.trigger_bot.enabled, "Enabled");
                        ui.add(egui::Slider::new(&mut lock.trigger_bot.sleep, 40..=170).text("Sleep"));
                    });
                    egui::CollapsingHeader::new("Aim Assist")
                        .default_open(false).show(ui,|ui| {
                        ui.checkbox(&mut lock.aim_assist.enabled, "Enabled");
                        ui.add(egui::Slider::new(&mut lock.aim_assist.speed, 0..=15).text("Speed"));
                        ui.add(egui::Slider::new(&mut lock.aim_assist.fov, 0.0..=120.0).text("Fov"));
                    });
                });
            ui.separator();
            egui::CollapsingHeader::new("Misc")
                .default_open(false)
                .show(ui, |ui| {
                    egui::CollapsingHeader::new("Fast Place")
                        .default_open(false).show(ui,|ui| {
                        ui.checkbox(&mut lock.fast_place.enabled , "Enabled");
                        ui.checkbox(&mut lock.fast_place.disable_on_rod , "Disable on Projectiles");
                    });
                    egui::CollapsingHeader::new("Rod Assist")
                        .default_open(false).show(ui,|ui| {
                        egui::CollapsingHeader::new("Rod Pullback Overrider")
                            .default_open(false).show(ui,|ui| {
                            ui.checkbox(&mut lock.rod_assist.pullback_enabled , "Enabled");
                            ui.checkbox(&mut lock.rod_assist.switch_on_pullback , "Switch on Pullback");
                            ui.add(egui::Slider::new(&mut lock.rod_assist.switch_delay, 5..=150).text("Switch Delay"));
                            ui.add(egui::Slider::new(&mut lock.rod_assist.pullback_delay, 50..=800).text("Pullback Delay"));
                            ui.add(egui::Slider::new(&mut lock.rod_assist.pullback_point, 1..=4).text("Pullback Point"));
                            ui.add(egui::Slider::new(&mut lock.rod_assist.trigger_point, 0..=4).text("Trigger Point"));
                        });
                        egui::CollapsingHeader::new("Rod Aim Assist")
                            .default_open(false).show(ui,|ui| {
                            ui.checkbox(&mut lock.rod_assist.aimbot_enabled , "Enabled");
                        });
                    });
                });
        });
    }

    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        egui::Rgba::TRANSPARENT.to_array()
    }
}

fn setup_custom_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();

    fonts.font_data.insert(
        "cpfont".to_owned(),
        egui::FontData::from_static(include_bytes!("assets/font.ttf")),
    );

    fonts.families.get_mut(&egui::FontFamily::Proportional).unwrap().insert(0, "cpfont".to_owned());

    ctx.set_fonts(fonts);
}


fn custom_window_frame(ctx: &egui::Context, title: &str, add_contents: impl FnOnce(&mut egui::Ui)) {
    setup_custom_fonts(ctx);
    catppuccin_egui::set_theme(ctx, catppuccin_egui::LATTE);
    let panel_frame = egui::Frame {
        fill: ctx.style().visuals.window_fill(),
        rounding: 6.0.into(),
        stroke: ctx.style().visuals.widgets.noninteractive.fg_stroke,
        outer_margin: 0.5.into(), // so the stroke is within the bounds
        ..Default::default()
    };
    CentralPanel::default().frame(panel_frame).show(ctx, |ui| {
        let app_rect = ui.max_rect();

        let title_bar_height = 32.0;
        let title_bar_rect = {
            let mut rect = app_rect;
            rect.max.y = rect.min.y + title_bar_height;
            rect
        };
        title_bar_ui(ui, title_bar_rect, title);
        let content_rect = {
            let mut rect = app_rect;
            rect.min.y = title_bar_rect.max.y;
            rect
        }
            .shrink(4.0);
        let mut content_ui = ui.child_ui(content_rect, *ui.layout(), None);
        add_contents(&mut content_ui);
    });
}

fn title_bar_ui(ui: &mut egui::Ui, title_bar_rect: eframe::epaint::Rect, title: &str) {
    use egui::*;

    let painter = ui.painter();

    let title_bar_response = ui.interact(
        title_bar_rect,
        Id::new("title_bar"),
        Sense::click_and_drag(),
    );
    painter.text(
        title_bar_rect.center(),
        Align2::CENTER_CENTER,
        title,
        FontId::proportional(20.0),
        ui.style().visuals.text_color(),
    );
    painter.line_segment(
        [
            title_bar_rect.left_bottom() + vec2(1.0, 0.0),
            title_bar_rect.right_bottom() + vec2(-1.0, 0.0),
        ],
        ui.visuals().widgets.noninteractive.bg_stroke,
    );
    if title_bar_response.double_clicked() {
        let is_maximized = ui.input(|i| i.viewport().maximized.unwrap_or(false));
        ui.ctx()
            .send_viewport_cmd(ViewportCommand::Maximized(!is_maximized));
    }

    if title_bar_response.drag_started_by(PointerButton::Primary) {
        ui.ctx().send_viewport_cmd(ViewportCommand::StartDrag);
    }
    ui.allocate_ui_at_rect(title_bar_rect, |ui| {
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.spacing_mut().item_spacing.x = 0.0;
            ui.visuals_mut().button_frame = false;
            ui.add_space(8.0);
            close_maximize_minimize(ui);
        });
    });
}
fn close_maximize_minimize(ui: &mut egui::Ui) {
    use egui::{Button, RichText};

    let button_height = 25.0;

    let close_response = ui
        .add(Button::new(RichText::new("❌").size(button_height)));
    if close_response.clicked() {
        ui.ctx().send_viewport_cmd(ViewportCommand::Close);
    }
    let minimized_response = ui
        .add(Button::new(RichText::new("➖").size(button_height)));
    if minimized_response.clicked() {
        ui.ctx().send_viewport_cmd(ViewportCommand::Minimized(true));
    }
}