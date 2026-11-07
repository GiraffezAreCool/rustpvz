/*
Hello it is GiraffezAreCool.
I have no idea what im doing, this is my first time using rust and vulkan.

umm idk what im doing pls help

*/

use ash::vk;

use crate::app::App;
use anyhow::{Ok, Result};
use winit::event_loop::EventLoop;

mod app;

//use rand::prelude::*;
//use std::io;

//Graphics Rendering

//Level
pub struct Level {
    id: u16,
    sun: i16,
    weather: Weather,
    boards: Vec<Board>,
    waves: u16,
    large_waves: u16,
    start_points: u16,
    increment_points: u16,
}

impl Level {
    fn change_sun(&mut self, sun: i16) {
        self.sun = self.sun + sun;
    }
}

pub struct StandardWeather {
    sun: i16,
    sun_rate: u16,
}

pub enum Weather {
    Sunny(StandardWeather),
    Night(StandardWeather),
}

//Plants
pub trait PlantTraits {
    fn plant_info(&self) {}
    fn take_damage(&mut self, damage: u16) -> u16 {
        damage * 0
    }
}

#[derive(Clone)]
pub enum Plant {
    Peashooter(Peashooter),
    Sunflower(Sunflower),
}

impl PlantTraits for Plant {
    fn plant_info(&self) {
        match self {
            Plant::Peashooter(pea) => pea.plant_info(),
            Plant::Sunflower(sun) => sun.plant_info(),
        }
    }

    fn take_damage(&mut self, damage: u16) -> u16 {
        match self {
            Plant::Peashooter(pea) => pea.take_damage(damage),
            Plant::Sunflower(sun) => sun.take_damage(damage),
        }
    }
}

#[derive(Clone)]
pub struct Peashooter {
    name: String,
    sun_cost: i32,
    recharge: u16,
    health: u16,
    damage: u16,
    rate: u16,
}

impl PlantTraits for Peashooter {
    fn plant_info(&self) {
        println!(
            "Name: {}\nSun Cost: {}\nRecharge: {}\nHealth: {}\nDamage: {}\nRate: {}",
            self.name, self.sun_cost, self.recharge, self.health, self.damage, self.rate
        );
    }

    fn take_damage(&mut self, damage: u16) -> u16 {
        self.health -= damage;
        return self.health;
    }
}

#[derive(Clone)]
pub struct Sunflower {
    name: String,
    sun_cost: i32,
    recharge: u16,
    health: u16,
    sun: u16,
    rate: u16,
}

impl PlantTraits for Sunflower {
    fn plant_info(&self) {
        println!(
            "Name: {}\nSun Cost{}\nRecharge {}\nHealth {}\nSun {}\nRate {}",
            self.name, self.sun_cost, self.recharge, self.health, self.sun, self.rate
        );
    }
}

//Zombies

#[derive(Clone)]
pub struct Basic {
    name: String,
    speed: f32,
    health: u16,
    damage: u16,
    rate: u16,
    value: u16,
    x: f32,
    y: u16,
}

pub enum Zombie {
    Basic(Basic),
}

//Tile

#[derive(Clone)]
pub enum Tile {
    Grass,
    Soil,
    Water,
    Tile,
}

//Board

#[derive(Clone)]
pub struct Board {
    width: u16,
    height: u16,
    board: Vec<Option<Plant>>,
    tile_types: Vec<Tile>,
}

fn to_index(x: u16, y: u16, w: u16) -> usize {
    if x > w - 1 {
        panic!("x is larger than width");
    } else {
        return (x + y * w) as usize;
    }
}

impl Board {
    fn new(width: u16, height: u16) -> Self {
        let board: Vec<Option<Plant>> = vec![None; (width * height) as usize];
        let tile_types: Vec<Tile> = vec![Tile::Grass; (width * height) as usize];
        return Self {
            width,
            height,
            board,
            tile_types,
        };
    }

    fn place_plant(&mut self, plant: Plant, x: u16, y: u16) {
        self.board[to_index(x, y, self.width)] = Some(plant);
    }

    fn get_plant(&mut self, x: u16, y: u16) -> &mut Plant {
        self.board[to_index(x, y, self.width) as usize]
            .as_mut()
            .expect("There are no plants at the specified coordinate")
    }

    fn remove_plant(&mut self, x: u16, y: u16) {
        self.board[to_index(x, y, self.width)] = None
    }

    fn print_board(&self) {
        let mut output: String = String::from("");
        let mut i: u16 = 0;
        for t in &self.board {
            match t {
                None => {
                    output.push('-');
                }
                p => match p {
                    Some(Plant::Peashooter(pea)) => output.push('P'),
                    Some(Plant::Sunflower(sun)) => output.push('S'),
                    None => output.push('p'),
                },
            }
            i += 1;
            if i == self.width {
                i = 0;
                output.push('\n');
            }
        }
        println!("{}", output);
    }
}

fn main() -> Result<()> {
    let lawn: Board = Board::new(9, 5);

    let mut level: Level = Level {
        id: 1,
        sun: 50,
        weather: Weather::Sunny(StandardWeather {
            sun: 50,
            sun_rate: 5,
        }),
        boards: vec![lawn],
        waves: 16,
        large_waves: 1,
        start_points: 1,
        increment_points: 1,
    };

    let peashooter = Plant::Peashooter(Peashooter {
        name: String::from("peashooter"),
        sun_cost: 100,
        recharge: 5,
        health: 300,
        damage: 20,
        rate: 5,
    });
    let sunflower = Plant::Sunflower(Sunflower {
        name: String::from("sunflower"),
        sun_cost: 50,
        recharge: 5,
        health: 300,
        sun: 25,
        rate: 24,
    });

    level.boards[0].place_plant(peashooter, 1, 1);
    level.boards[0].place_plant(sunflower, 0, 1);
    if level.boards[0].get_plant(1, 1).take_damage(300) <= 0 {
        level.boards[0].remove_plant(1, 1);
    }
    //level.boards[0].get_plant(1, 1).plant_info();
    level.boards[0].print_board();

    tracing_subscriber::fmt::init();

    let mut app = App::default();

    let event_loop = EventLoop::new()?;
    event_loop.run_app(&mut app)?;

    Ok(())
}
