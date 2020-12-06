use nannou::prelude::*;
use std::cmp::max;

const ITEM_COUNT: usize = 16;
const CAPACITY: usize = 100;
const MAX_VALUE: usize = 100;

fn main() {
    nannou::app(model).event(event).run();
}

struct Model {
    i: usize,
    j: usize,
    dp: [[usize; CAPACITY + 1]; ITEM_COUNT + 1],
    cost: [usize; ITEM_COUNT],
    value: [usize; ITEM_COUNT],
}

// i は 1から
fn update(model: &mut Model) {
    if model.j > CAPACITY {
        model.j = 0;
        model.i += 1;
    }
    let i = model.i;
    let j = model.j;
    if i > ITEM_COUNT {
        return;
    }
    model.dp[i][j] = model.dp[i - 1][j];
    let cost = dp[i - 1];
    let value = dp[i - 1];
    if j <= cost {
        model.dp[i][j] = max(
            model.dp[i - 1][j],
            model.dp[i - 1][j - cost] + value,
        );
    }

    model.j += 1;
}

fn event(_app: &App, _model: &mut Model, _event: Event) {}

fn model(app: &App) -> Model {
    app.set_loop_mode(LoopMode::Wait);
    let _window = app
        .new_window()
        .key_pressed(key_pressed)
        .view(view)
        .build()
        .unwrap();
    /*
        i: usize,
    j: usize,
    dp: [[usize; CAPACITY +1];ITEM_COUNT+1],
    cost: [usize;ITEM_COUNT],
    value: [usize;ITEM_COUNT],
     */
    let mut cost: [usize; ITEM_COUNT] = [0; ITEM_COUNT];
    let mut value: [usize; ITEM_COUNT] = [0; ITEM_COUNT];
    for i in 0..ITEM_COUNT {
        cost[i] = random_range(0,CAPACITY);
        value[i] = random_range(0,MAX_VALUE);
    }
    Model {
        i: 1,
        j: 0,
        dp: [[0; CAPACITY + 1]; ITEM_COUNT + 1],
        cost,
        value,
    }
}

fn key_pressed(_app: &App, model: &mut Model, key: Key) {
    match key {
        Key::Right => update(model),
        _ => {}
    };
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(WHITE);

    for i in 0..model.dp.len() {
        for j in 0..model.dp[0].len() {
            // TODO: dpの中身を描画する
            draw.text(&format!("{}",model.dp[i][j])[..]);
        }
    }

    draw.to_frame(app, &frame).unwrap();
}
