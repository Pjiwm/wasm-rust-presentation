use egui::{
    containers::Frame, emath, epaint, epaint::PathStroke, hex_color, lerp, pos2, remap, vec2,
    Color32, Pos2, Rect, Ui,
};

pub fn ui(ui: &mut Ui) {
    Color32::from_black_alpha(240);
    Frame::canvas(ui.style()).show(ui, |ui| {
        ui.ctx().request_repaint();
        let time = ui.input(|i| i.time);

        let desired_size = ui.available_width() * vec2(1.0, 0.2);
        let (_id, rect) = ui.allocate_space(desired_size);

        let to_screen =
            emath::RectTransform::from_to(Rect::from_x_y_ranges(0.0..=1.0, -1.0..=1.0), rect);

        let mut shapes = vec![];

        for &mode in &[2, 3, 5] {
            let mode = mode as f64;
            let n = 120;
            let speed = 1.5;

            let points: Vec<Pos2> = (0..=n)
                .map(|i| {
                    let t = i as f64 / (n as f64);
                    let amp = (time * speed * mode).sin() / mode;
                    let y = amp * (t * std::f64::consts::TAU / 2.0 * mode).sin();
                    to_screen * pos2(t as f32, y as f32)
                })
                .collect();

            let thickness = 10.0 / mode as f32;
            shapes.push(epaint::Shape::line(
                points,
                PathStroke::new_uv(thickness, move |rect, p| {
                    let t = remap(p.x, rect.x_range(), -1.0..=1.0).abs();
                    let center_color = hex_color!("#5BCEFA");
                    let outer_color = hex_color!("#F5A9B8");

                    Color32::from_rgb(
                        lerp(center_color.r() as f32..=outer_color.r() as f32, t) as u8,
                        lerp(center_color.g() as f32..=outer_color.g() as f32, t) as u8,
                        lerp(center_color.b() as f32..=outer_color.b() as f32, t) as u8,
                    )
                }),
            ));
        }

        ui.painter().extend(shapes);
    });
}
