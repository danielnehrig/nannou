// This example is a copy of the `simple_draw.rs` example, but captures each frame and writes them
// as a PNG image file to `/<path_to_nannou>/nannou/simple_capture/<frame_number>.png`

use nannou::prelude::*;

fn main() {
    nannou::sketch(view).run()
}

fn view(app: &App, frame: Frame) {
    let draw = app.draw();

    draw.background().color(CORNFLOWERBLUE);

    let win = app.window_rect();
    draw.tri()
        .points(win.bottom_left(), win.top_left(), win.top_right())
        .color(VIOLET);

    let t = app.time;
    draw.ellipse()
        .x_y(app.mouse.x * t.cos(), app.mouse.y)
        .radius(win.w() * 0.125 * t.sin())
        .color(RED);

    draw.line()
        .weight(10.0 + (t.sin() * 0.5 + 0.5) * 90.0)
        .caps_round()
        .color(PALEGOLDENROD)
        .points(win.top_left() * t.sin(), win.bottom_right() * t.cos());

    draw.quad()
        .x_y(-app.mouse.x, app.mouse.y)
        .color(DARKGREEN)
        .rotate(t);

    draw.rect()
        .x_y(app.mouse.y, app.mouse.x)
        .w(app.mouse.x * 0.25)
        .hsv(t, 1.0, 1.0);

    draw.to_frame(app, &frame).unwrap();

    // Create a path that we want to save this frame to.
    let file_path = app
        .project_dir()
        // Capture all frames to a directory called `/<path_to_nannou>/nannou/simple_capture`.
        .join(app.exe_name().unwrap())
        // Name each file after the number of the frame.
        .join(frame.nth().to_string())
        // The extension will be PNG. We also support tiff, bmp, gif, jpeg, webp and some others.
        .with_extension("png");

    // Capture the frame!
    app.main_window().capture_frame(file_path);
}
