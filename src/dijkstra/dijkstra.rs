use nannou::prelude::*;
use std::collections::BinaryHeap;
use nannou::math::cgmath::num_traits::abs;
use std::time::Duration;
use nannou::app::DrawScalar;

// a[0] から a[SIZE] までの最短経路を求める (ダイクストラ法)
fn main() {
    nannou::app(model)
        .update(update)
        .run();
}

const NODE_SIZE: usize = 20;
const EDGE_SIZE: usize = 5;

struct Model {
    _window: window::Id,
    node: [(i32, i32); NODE_SIZE],
    cost: [i32; NODE_SIZE],
    edge: [[usize; EDGE_SIZE]; NODE_SIZE],
    que: BinaryHeap<(i32, usize)>,
    // edgeの何番目を参照しているか
    i: usize,
    // キュー
    checking_line: (usize, usize),
    // 今確認しているライン
    confirmed_line: [usize; NODE_SIZE],
}

fn model(app: &App) -> Model {
    let _window = app.new_window().view(view).build().unwrap();
    let mut cost = [i32::MAX; NODE_SIZE];
    cost[0] = 0;

    let mut node: [(i32, i32); NODE_SIZE] = [(0, 0); NODE_SIZE];
    for i in 0..NODE_SIZE {
        node[i] = (
            random_range(0, NODE_SIZE * 2) as i32,
            random_range(0, NODE_SIZE * 2) as i32,
        );
    }

    let mut edge: [[usize; EDGE_SIZE]; NODE_SIZE] = [[0; EDGE_SIZE]; NODE_SIZE];
    for i in 0..edge.len() {
        let size = EDGE_SIZE;
        for j in 0..size {
            let node = random_range(0, NODE_SIZE - 1);
            edge[i][j] = node;
        }
    }

    let mut que: BinaryHeap<(i32, usize)> = BinaryHeap::new();
    que.push((0, 0));

    Model {
        _window,
        node,
        cost,
        que,
        edge,
        checking_line: (0, 1),
        confirmed_line: [NODE_SIZE; NODE_SIZE],
        i: 0,
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    if model.que.is_empty() {
        return;
    }
    let (cost, crt) = match model.que.peek() {
        Some(e) => *e,
        None => return,
    };
    if model.cost[crt] < cost {
        model.que.pop();
        model.i = 0;
        return;
    }

    if model.i >= model.edge[crt].len() {
        model.i = 0;
        model.que.pop();
        return;
    }

    let next = model.edge[crt][model.i];
    model.checking_line = (crt, next);

    let cost = cost + calc_cost(model.node[crt], model.node[next]);
    if model.cost[next] > cost {
        model.confirmed_line[next] = crt;
        model.cost[next] = cost;
        model.que.push((cost, next));
    }
    model.i += 1;
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    let boundary = app.window_rect();

    draw.background().color(WHITE);

    draw.text(&format!("que size is {}", model.que.len())[..])
        .x_y(boundary.left() + 60.0, boundary.top() - 30.0)
        .font_size(20)
        .color(BLACK);

    let draw_line = |start: (i32, i32), end: (i32, i32), color: Rgb8| {
        draw.arrow().start(
            get_point(start, boundary)
        ).end(
            get_point(end, boundary)
        ).weight(2.0).color(color);
    };

    for i in 0..NODE_SIZE {
        for e in model.edge[i].iter() {
            draw_line(
                model.node[i],
                model.node[*e],
                GOLD,
            );
        }
    }

    for i in 0..model.confirmed_line.len() {
        if model.confirmed_line[i] == NODE_SIZE {
            continue;
        }
        draw_line(
            model.node[model.confirmed_line[i]],
            model.node[i],
            DARKBLUE,
        );
    }
    draw_line(
        model.node[model.checking_line.1],
        model.node[model.checking_line.0],
        CRIMSON,
    );
    for i in 0..model.cost.len() {
        draw.text(&format!("{}", model.cost[i])[..])
            .color(BLACK)
            .x_y(
                get_point(model.node[i], boundary).x + 10.0,
                get_point(model.node[i], boundary).y + 10.0,
            );
    }
    draw.ellipse()
        .x_y(
            get_point(model.node[0], boundary).x,
            get_point(model.node[0], boundary).y,
        )
        .w_h(15.0, 15.0).color(GREEN);
    draw.to_frame(app, &frame).unwrap();
}

fn calc_cost(start: (i32, i32), end: (i32, i32)) -> i32 {
    let x_dif = abs(start.0 - end.0);
    let y_dif = abs(start.1 - end.1);
    x_dif + y_dif
}

fn get_point(point: (i32, i32), boundary: geom::Rect<DrawScalar>) -> Point2 {
    pt2(
        map_range(point.0 as f32,
                  0.0,
                  (NODE_SIZE * 2) as f32,
                  boundary.left(),
                  boundary.right()),
        map_range(point.1 as f32,
                  0.0,
                  (NODE_SIZE * 2) as f32,
                  boundary.bottom(),
                  boundary.top()),
    )
}
