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

fn update(_app: &App, _model: &mut Model, _update: Update) {
    if _model.que.is_empty() {
        return;
    }
    let (cost, crt) = match _model.que.peek() {
        Some(e) => *e,
        None => return,
    };
    if _model.cost[crt] < cost {
        _model.que.pop();
        _model.i = 0;
        return;
    }

    if _model.i >= _model.edge[crt].len() {
        _model.i = 0;
        _model.que.pop();
        return;
    }

    let next = _model.edge[crt][_model.i];
    _model.checking_line = (crt, next);

    let cost = cost + calc_cost(_model.node[crt], _model.node[next]);
    if _model.cost[next] > cost {
        _model.confirmed_line[next] = crt;
        _model.cost[next] = cost;
        _model.que.push((cost, next));
    }
    _model.i += 1;
}

fn view(app: &App, _model: &Model, frame: Frame) {
    let draw = app.draw();
    let boundary = app.window_rect();
    draw.background().color(WHITE);

    draw.text(&format!("que size is {}", _model.que.len())[..])
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
        for e in _model.edge[i].iter() {
            draw_line(
                _model.node[i],
                _model.node[*e],
                GOLD,
            );
        }
    }

    for i in 0.._model.confirmed_line.len() {
        if _model.confirmed_line[i] == NODE_SIZE {
            continue;
        }
        draw_line(
            _model.node[_model.confirmed_line[i]],
            _model.node[i],
            DARKBLUE,
        );
    }
    draw_line(
        _model.node[_model.checking_line.1],
        _model.node[_model.checking_line.0],
        CRIMSON,
    );
    for i in 0.._model.cost.len() {
        draw.text(&format!("{}", _model.cost[i])[..])
            .color(BLACK)
            .x_y(
                get_point(_model.node[i], boundary).x + 10.0,
                get_point(_model.node[i], boundary).y + 10.0,
            );
    }
    draw.ellipse()
        .x_y(
            get_point(_model.node[0], boundary).x,
            get_point(_model.node[0], boundary).y,
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
