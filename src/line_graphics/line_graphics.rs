use nannou::prelude::*;

fn main() {
    nannou::app(model).event(event).run();
}

struct Model {
    count: f32,
    do_draw_no1: bool,
    do_draw_no2: bool,
    do_draw_no3: bool,
}
fn event(_app: &App, _model: &mut Model, _event: Event) {

}

fn model(app: &App) -> Model {
    app.set_loop_mode(LoopMode::Wait);
    let _window = app
        .new_window()
        .key_pressed(key_pressed)
        .view(view)
        .build()
        .unwrap();
    Model{
        count: 0.0,
        do_draw_no1: true,
        do_draw_no2: false,
        do_draw_no3: false,
    }
}

fn key_pressed(_app: &App, model: &mut Model, key: Key) {
    model.count += match key {
        Key::Right | Key::Up => 0.05,
        Key::Left | Key::Down => -0.05,
        _ => 0.0
    };
    match key {
        Key::Key1 => model.do_draw_no1 = !model.do_draw_no1,
        Key::Key2 => model.do_draw_no2 = !model.do_draw_no2,
        Key::Key3 => model.do_draw_no3 = !model.do_draw_no3,
        _ => {}
    }
}

fn view(app: &App, model: &Model, frame: Frame){
    println!("view was called");
    let draw = app.draw();
    draw.background().color(WHITE);

    const INIT_LEN: f32 = 300.0;
    const INIT_ANGLE: f32 = 1.0;
    let step = 2.0;
    let mut len = INIT_LEN;
    let mut angle = INIT_ANGLE;
    let mut x = 0.0;
    let mut y = 0.0;

    let mut draw_graphics = |angle_step: f32, len_step: f32, color: Rgb8|{
        len = INIT_LEN;
        angle = INIT_ANGLE;
        x = 0.0;
        y = 0.0;
        while len > 10.0 {
            draw.line()
                .start(pt2(x,y))
                .end(pt2(x + angle.cos() * len,y + angle.sin() * len))
                .color(color);
            x  += angle.cos() * len;
            y  += angle.sin() * len;
            angle += angle_step;
            len -= len_step;
        };
    };
    let count = model.count;
    if model.do_draw_no1 {
        draw_graphics(count, step,RED);
    }
    if model.do_draw_no2  {
        draw_graphics(-count,map_range(count.sin(),-1.0,1.0,1.0,5.0),GREEN);
    }
    if model.do_draw_no3 {
        draw_graphics(INIT_ANGLE,map_range(count.sin(),-1.0,1.0,1.0,5.0),BLUE);
    }

    draw.to_frame(app, &frame).unwrap();
}
