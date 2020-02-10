use crate::core::constants;
use crate::core::coordinate::Coord;
use crate::core::environment::Environment;
use crate::world::pacman::maze;
use crate::world::pacman::player::Player;
use crate::world::pacman::wall::Wall;
use crate::world::particle::Particle;
use crate::AgentImpl;
use rand::{seq::SliceRandom, thread_rng, Rng};
use std::cell::RefCell;
use std::rc::Rc;

pub struct Sma {
    pub environment: Environment,
}

impl Sma {
    pub fn new(width: i32, height: i32) -> Sma {
        let environment = Environment::new(width, height);

        Sma { environment }
    }

    pub fn tick(&mut self) {
        self.environment.tick()
    }

    pub fn get_agents(&self) -> &Vec<AgentImpl> {
        &self.environment.agents
    }

    pub fn gen_wator(&mut self) {}

    pub fn update_player_direction(&mut self, direction: Coord) {
        self.environment.update_player_direction(direction);
    }

    pub fn gen_pacman(&mut self) {
        // Player agent is always expected to live at index 0
        self.environment
            .add_agent(Rc::new(RefCell::new(Player::new())));

        maze::gen().iter().for_each(|wall| {
            self.environment
                .add_agent(Rc::new(RefCell::new(wall.clone())))
        })
        //        self.environment.add_agent(Rc::new(RefCell::new(Player::new())));
    }

    pub fn gen_particules(&mut self, density: i32) {
        let particules_number = (constants::size() / 100) * density;
        let mut vec: Vec<i32> = (0..constants::size()).collect();

        let mut rng = thread_rng();
        vec.shuffle(&mut rng);

        (0..(particules_number as usize)).for_each(|_| {
            let idx = vec.pop().unwrap();
            let x = idx % constants::max_width();
            let y = (idx - x) / constants::max_height();
            let coordinate = Coord(x, y);

            self.environment
                .add_agent(Rc::new(RefCell::new(Particle::new(coordinate))));
        });
    }
}
