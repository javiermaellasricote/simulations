mod forest;
use forest::*;
use nannou::*;
use nannou::prelude::{DARKGRAY, GREEN, RED};

struct Model {
    _window_id: window::Id,
    iteration: i32,
    forest: Forest,
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(DARKGRAY);
    for col in model.forest.land.iter() {
        for plot in col.iter() {
            draw.rect()
                .w_h(1.0,1.0)
                .x(plot.position.x)
                .y(plot.position.y)
                .color(match plot.state {
                    State::Tree => GREEN,
                    State::Burning(_) => RED,
                    State::Empty => DARKGRAY,
                });
        }
    }
    draw.to_frame(app, &frame).unwrap();
    let file_name = format!("./output/{:?}.png", model.iteration);
    app.main_window().capture_frame(file_name);
}

fn init(app: &App) -> Model {
    Model {
        _window_id: app
            .new_window()
            .view(view)
            .build()
            .unwrap(),
        iteration: 0,
        forest: Forest::new((100,100), (50,50), 200),
    }
}


fn update(app: &App, model: &mut Model, _update: event::Update) {
    model.forest.update();
    model.iteration += 1;
    if model.iteration > 30 {
        app.quit();
    }
}

fn main() {
    app(init).update(update).run();
}
