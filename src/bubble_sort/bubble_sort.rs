use nannou::prelude::*;
use rand::seq::SliceRandom;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    _window: window::Id,
    speeds: Vec<i32>,
    i: usize,
    j: usize
}

const SIZE: usize = 128;
fn model(app: &App) -> Model {
    let _window = app.new_window().view(view).build().unwrap();
    let mut speeds: Vec<i32> = Vec::with_capacity(SIZE);

    for i in 1..SIZE {
        speeds.push(i as i32);
    }

    let mut rng = rand::thread_rng();
    speeds.shuffle(&mut rng);
    Model { _window , speeds, i: 0, j: 0 }
}


fn update(_app: &App, _model: &mut Model, _update: Update) {
    if _model.j >= _model.speeds.len() {
        _model.j = 0;
        _model.i += 1;
    }
    if _model.i >= _model.speeds.len() {
        reinitialize_model(_model);
        return;
    }
    let i = _model.i;
    let j = _model.j;
    if _model.speeds[i] > _model.speeds[j] {
        _model.speeds.swap(i,j);
    }
    _model.j += 1;
}

fn view(app: &App, _model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(PLUM);
    let boundary = app.window_rect();

    for i in 0.._model.speeds.len() {
        let y = map_range(
            _model.speeds[i] as f32,
            0.0,
            SIZE as f32,
            0.0,
            boundary.h(),
        ) ;
        let x = map_range(
            i as f32,
            0.0,
            _model.speeds.len() as f32,
            boundary.left(),
            boundary.right(),
        );

        draw.rect()
            .x_y(x,boundary.bottom())
            .w_h(10.0,y).color(
            if i == _model.i { FIREBRICK }
            else if i == _model.j { STEELBLUE }
            else { AZURE }
        );
    }

    draw.to_frame(app, &frame).unwrap();
}

fn reinitialize_model(_model: &mut Model){
    let mut rng = rand::thread_rng();
    _model.speeds.shuffle(&mut rng);
    _model.i = 0;
    _model.j = 0;
}
