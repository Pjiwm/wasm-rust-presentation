# egui
test
[www.egui.rs](www.egui.rs)
-------------------------------------------------------------------------------

# This presentation
### - What is it?
### - How does it work?
Demo:

!!!dancing_strings
-------------------------------------------------------------------------------

# What is WebAssembly?

## WebAssembly (Wasm): is a binary instruction format designed to run code efficiently on modern web browsers.
  ### - Provide a fast and compact runtime for executing code.
  ### - Enable high-performance applications, such as games, video editing, and scientific simulations, directly in the browser.
![wasm-compilers](bytes://wasm-compilers.png)

-------------------------------------------------------------------------------

# Use cases
## - Heavy Computation Tasks: simulations, data analysis
## - Games and Interactive 3D Graphics running in the browser with near-native execution speed.

## Success stories

### Adobe Photoshop: One of the most prominent examples of WebAssembly in action, Adobe has successfully ported Photoshop to the web using WASM.

### Figma: The popular design tool initially used WebAssembly for certain performance-critical parts of their application,
### though they’ve been less vocal about their WASM usage in recent years.

### npm libraries: Many npm packages use WebAssembly under the hood
### often for performance-critical operations compiled from C++ or Rust.

-------------------------------------------------------------------------------
# Painting demo test
<!-- ![hennge](bytes://hennge.png) -->
!!!painting
-------------------------------------------------------------------------------

# Context

How to write your own egui integration:
```rs
let mut ctx = egui::Context::default();

// Game loop:
loop {
    // Gather keyboard/mouse events:
    let raw_input = window.collect_input();

    // Run egui:
    let output = ctx.run(raw_input, |ctx| {
        egui::CentralPanel::default().show(&ctx, |ui| {
            ui.label("Hello world!");
            if ui.button("Click me").clicked() {
                // take some action here
            }
        });
    });

    // Set cursor icon, set clipboard, open url, …
    window.handle_platform_output(output.platform_output);

    let triangles = ctx.tessellate(output.shapes);
    window.paint(output.textures_delta, triangles);
}
```
-------------------------------------------------------------------------------

# Scopes

* Different layouts
* Containers widgets (`ScrollArea`, `Window`, …)
* Scopes with different styling
* …
-------------------------------------------------------------------------------

# Writing a widget

``` rs
fn toggle_widget(ui: &mut Ui, on: &mut bool) -> Response {
    let desired_size = ui.spacing().interact_size.y * vec2(2.0, 1.0);
    let (rect, mut response) = ui.allocate_exact_size(desired_size, Sense::click());
    if response.clicked() {
        *on = !*on;
        response.mark_changed();
    }
    response.widget_info(|| WidgetInfo::selected(WidgetType::Checkbox, *on, ""));

    if ui.is_rect_visible(rect) {
        let how_on = ui.ctx().animate_bool(response.id, *on);
        let visuals = ui.style().interact_selectable(&response, *on);
        let rect = rect.expand(visuals.expansion);
        let radius = 0.5 * rect.height();
        ui.painter()
            .rect(rect, radius, visuals.bg_fill, visuals.bg_stroke);
        let circle_x = lerp((rect.left() + radius)..=(rect.right() - radius), how_on);
        let center = pos2(circle_x, rect.center().y);
        ui.painter()
            .circle(center, 0.75 * radius, visuals.bg_fill, visuals.fg_stroke);
    }

    response
}
```

``` rs
toggle_widget(ui, &mut some_bool);
```

!!!toggle_widget
-------------------------------------------------------------------------------

# eframe
The official egui framework

* Windows, Mac, Linux, Android, iOS, Web
* `winit` on native
* `js-sys` on web
* Renders using either `glow` (OpenGL) or `wgpu`
-------------------------------------------------------------------------------

# Q&A
