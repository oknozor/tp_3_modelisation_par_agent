#[macro_use]
extern crate lazy_static;
extern crate serde;
#[macro_use]
extern crate serde_derive;

use std::fs::File;
use std::io::prelude::*;

use nannou::event::Key;
use nannou::prelude::*;

use racman_lib::core::coordinate::Coord;
use racman_lib::core::sma::Sma;
use racman_lib::RgbColor;

mod user_config;

lazy_static! {
    pub static ref CONFIG: user_config::Config = {
        let mut file = File::open("config.json").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        serde_json::from_str(&contents).expect("expected json")
    };
}

fn main() {
    nannou::app(model).update(update).run();
}

struct Grid {
    sma: Sma,
}

impl Grid {
    fn new() -> Self {
        //        CONFIG.fish_breed_time,
        //        CONFIG.shark_breed_time,
        //        CONFIG.shark_starve_time,
        //        CONFIG.borderless,
        let mut sma = Sma::new(CONFIG.x as i32, CONFIG.y as i32);
        match CONFIG.mode.as_str() {
            "pacman" => sma.gen_pacman(),
            "particules" => sma.gen_particules(CONFIG.particle_number),
            _ => panic!("Unkown grid mode!"),
        }

        Grid { sma }
    }

    fn display(&self, draw: &app::Draw) {
        let offset = CONFIG.cell_size;

        let width = CONFIG.x * offset;
        let height = CONFIG.y * offset;
        draw.rect()
            .w_h(width, height)
            .rgb(1.0, 1.0, 1.0)
            .stroke(rgb(0.0, 0.0, 0.0));

        self.sma.get_agents().iter().for_each(|agent| {
            let agent = agent.clone();

            let x = agent.borrow().coordinates().0 as f32;
            let y = agent.borrow().coordinates().1 as f32;
            let x = (x * CONFIG.cell_size) - width / 2.0 + offset / 2.0;
            let y = (y * CONFIG.cell_size) - height / 2.0 + offset / 2.0;
            self.display_agent(&draw, agent.borrow().color(), x, y);
        });
    }

    fn display_agent(&self, draw: &app::Draw, color: &RgbColor, x: f32, y: f32) {
        draw.rect()
            .x_y(x, y)
            .w_h(CONFIG.cell_size, CONFIG.cell_size)
            .rgb(color.0, color.1, color.2);
    }
}

struct Model {
    pub grid: Grid,
    pub pause: bool,
    pub step: bool,
}

fn model(app: &App) -> Model {
    app.new_window()
        .with_maximized(true)
        .event(window_event)
        .view(view)
        .build()
        .unwrap();

    let grid = Grid::new();
    Model {
        grid,
        pause: true,
        step: false,
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    if !model.pause {
        model.grid.sma.tick();
    } else if model.step {
        model.grid.sma.tick();
        model.pause = true;
        model.step = false;
    }
}

fn view(app: &App, m: &Model, frame: &Frame) {
    // Begin drawing
    let draw = app.draw();
    draw.background().color(rgb(1.0, 1.0, 1.0));

    m.grid.display(&draw);

    // Write the result of our drawing to the window's frame.
    draw.to_frame(app, &frame).unwrap();
}

fn window_event(_: &App, model: &mut Model, event: WindowEvent) {
    match event {
        KeyPressed(key) => match key {
            Key::N => {
                model.pause = true;
                model.step = true;
            }
            Key::Space => model.pause = !model.pause,
            Key::Up => model.grid.sma.update_player_direction(Coord(0, 1)),
            Key::Down => model.grid.sma.update_player_direction(Coord(0, -1)),
            Key::Right => model.grid.sma.update_player_direction(Coord(1, 0)),
            Key::Left => model.grid.sma.update_player_direction(Coord(-1, 0)),
            _ => (),
        },
        _ => {}
    }
}
