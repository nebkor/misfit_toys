use nannou::prelude::*;
use nannou::ui::prelude::*;

struct Model {
    ui: Ui,
    ids: IDs,
    num_cols: usize,
    num_rows: usize,
}

struct IDs {
    grid_id: widget::Id,
    x_slider: widget::Id,
    y_slider: widget::Id,
}

fn main() {
    nannou::app(model_init)
        .size(1280, 1024)
        .update(update_model)
        .simple_window(draw_view)
        .run();
}

fn model_init(app: &App) -> Model {
    app.set_loop_mode(LoopMode::wait());

    let mut ui = app.new_ui().build().unwrap();
    let ids = IDs {
        grid_id: ui.generate_widget_id(),
        x_slider: ui.generate_widget_id(),
        y_slider: ui.generate_widget_id(),
    };

    Model {
        ui,
        ids,
        num_cols: 1,
        num_rows: 1,
    }
}

fn update_model(app: &App, model: &mut Model, _update: Update) {
    let ui = &mut model.ui.set_widgets();

    use widget::grid::{Axis, Lines};

    let (width, height) = app.window_rect().w_h();
    let width = width as u32 / 2;
    let height = height as u32 / 2;

    let lines = vec![
        Axis::X(Lines::step(width / model.num_cols as u32)),
        Axis::Y(Lines::step(height / model.num_rows as u32)),
    ]
    .into_iter();

    let grid = widget::grid::Grid::new(0, width, 0, height, lines)
        .w_h(width.into(), height.into())
        .color(color::ORANGE)
        .top_left_with_margin(20.0);

    grid.set(model.ids.grid_id, ui);

    fn slider(val: f32, min: f32, max: f32) -> widget::Slider<'static, f32> {
        widget::Slider::new(val, min, max)
            .w_h(200.0, 30.0)
            .label_font_size(15)
            .rgb(0.3, 0.3, 0.3)
            .label_rgb(1.0, 1.0, 1.0)
            .border(0.0)
    }

    for value in slider(model.num_cols as f32, 1.0, 20.0)
        .down_from(model.ids.grid_id, 20.0)
        .label("Columns")
        .set(model.ids.x_slider, ui)
    {
        model.num_cols = value as usize;
    }

    for value in slider(model.num_rows as f32, 1.0, 20.0)
        .right_from(model.ids.grid_id, 20.0)
        .label("Rows")
        .set(model.ids.y_slider, ui)
    {
        model.num_rows = value as usize;
    }
}

fn draw_view(app: &App, model: &Model, frame: Frame) {
    // Prepare to draw.
    let draw = app.draw();

    // Clear the background to purple.
    draw.background().color(BLACK);

    // Draw a blue ellipse with default size and position.
    //draw.ellipse().color(STEELBLUE);

    // Write to the window frame.
    draw.to_frame(app, &frame).unwrap();

    //dbg!(app.window(frame.window_id()).unwrap().scale_factor());

    let _ = model.ui.draw_to_frame(app, &frame);
}
