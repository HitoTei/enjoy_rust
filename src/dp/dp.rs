use nannou::prelude::*;
use std::cmp::max;

const ITEM_COUNT: usize = 16;
const CAPACITY: usize = 16;
const MAX_VALUE: usize = 16;

fn main() {
    nannou::app(model)
        .event(event)
        .simple_window(view)
        .run();
}

struct Model {
    i: usize,
    j: usize,
    dp: [[usize; CAPACITY + 1]; ITEM_COUNT + 1],
    costs: [usize; ITEM_COUNT],
    values: [usize; ITEM_COUNT],
}

// i は 1から
fn update(model: &mut Model) {
    model.j += 1;
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
    let cost = model.costs[i - 1];
    let value = model.values[i - 1];
    if j >= cost {
        model.dp[i][j] = max(
            model.dp[i - 1][j],
            model.dp[i - 1][j - cost] + value,
        );
    }
}

fn event(_app: &App, model: &mut Model, event: Event) {
    match event {
        Event::WindowEvent { id: _id, simple: window_event } => {
            if let Some(KeyPressed(key)) = window_event {
                match key {
                    Key::Right => { update(model) }
                    _ => {}
                }
            }
        }
        _ => {}
    }
}

fn model(app: &App) -> Model {
    app.set_loop_mode(LoopMode::Wait);

    let mut cost: [usize; ITEM_COUNT] = [0; ITEM_COUNT];
    let mut value: [usize; ITEM_COUNT] = [0; ITEM_COUNT];
    for i in 0..ITEM_COUNT {
        cost[i] = random_range(0, CAPACITY);
        value[i] = random_range(0, MAX_VALUE);
    }
    Model {
        i: 1,
        j: 0,
        dp: [[0; CAPACITY + 1]; ITEM_COUNT + 1],
        costs: cost,
        values: value,
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    let boundary = app.window_rect();
    draw.background().color(WHITE);

    let mut y_list: [f32; ITEM_COUNT] = [0.0; ITEM_COUNT];
    for i in 0..ITEM_COUNT {
        y_list[i] = map_range(
            (ITEM_COUNT - 1) - i,
            0,
            ITEM_COUNT,
            boundary.bottom() + 20.0,
            boundary.top());
    }

    for i in 0..ITEM_COUNT {
        draw
            .text(&format!("No: {},cost: {},value: {}", i + 1, model.costs[i], model.values[i])[..])
            .x_y(boundary.left() + 100.0, y_list[i])
            .color(BLACK);
    }

    if model.i > ITEM_COUNT {
        draw.text("FINISH").font_size(50).color(BLACK);
    } else {

        if model.j >= model.costs[model.i - 1] {
            draw// model.dp[i - 1][j - cost] + value,
                .text(
                    &format!("dp[No:{}][Cost:{}]: value: {} + No:{}(cost: {}): value: {}\n value: {}",
                             model.i - 1, // No
                             model.j - model.costs[model.i - 1], // Cost
                             model.dp[model.i - 1][model.j - model.costs[model.i - 1]], // value
                             model.i, // no
                             model.costs[model.i - 1], // cost
                             model.values[model.i - 1], // value
                             model.dp[model.i - 1][model.j - model.costs[model.i - 1]] + model.values[model.i - 1],
                    )[..]) // value
                .y(150.0)
                .w(boundary.w())
                .font_size(20)
                .color(BLUE);
        }

        draw
            .text(
                &format!("dp[No:{}][Cost:{}]: value: {}",
                         model.i - 1, model.j as i32, model.dp[model.i - 1][model.j])[..])
            .y(50.0)
            .w(boundary.w())
            .font_size(20)
            .color(RED);
        draw
            .text(
                &format!("dp[No:{}][Cost:{}]: value: {}",
                         model.i, model.j as i32, model.dp[model.i][model.j])[..])
            .y(0.0)
            .w(boundary.w())
            .font_size(20)
            .color(BLACK);
    }
    draw.to_frame(app, &frame).unwrap();
}
