use std::f64::consts;

use devotee::app;
use devotee::app::config;
use devotee::app::context::Context;
use devotee::app::input::key_mouse::{KeyMouse, VirtualKeyCode};
use devotee::app::setup;
use devotee::node::Node;
use devotee::util::vector::Vector;
use devotee::visual::canvas::Canvas;
use devotee::visual::color;
use devotee::visual::prelude::*;

fn main() {
    let init_config = setup::Setup::<Config>::default()
        .with_title("pentacle")
        .with_resolution((128, 128))
        .with_scale(2);
    let app = app::App::with_setup(init_config).unwrap();

    app.run();
}

struct Config;

impl config::Config for Config {
    type Node = PentacleNode;
    type Palette = TwoBits;
    type Converter = Converter;
    type Input = KeyMouse;

    fn converter() -> Self::Converter {
        Converter
    }

    fn background_color() -> Self::Palette {
        TwoBits::Black
    }
}

#[derive(Default)]
struct PentacleNode {
    rotation: f64,
}

impl Node<&mut Context<Config>, &mut Canvas<TwoBits>> for PentacleNode {
    fn update(&mut self, update: &mut Context<Config>) {
        if update.input().keys().just_pressed(VirtualKeyCode::Escape) {
            update.shutdown();
        }
        if update.input().keys().is_pressed(VirtualKeyCode::Z) {
            let delta = update.delta().as_secs_f64();
            self.rotation += delta;
        }
    }

    fn render(&self, render: &mut Canvas<TwoBits>) {
        render.clear(TwoBits::Black);

        let radius = 48.0 + 8.0 * self.rotation.sin();
        let center = Vector::new(64, 64);

        let mapper = |(x, y): (f64, f64)| (x.round() as i32 + 64, y.round() as i32 + 64);

        render.circle((64, 64), radius as i32, paint(TwoBits::White));
        render.filled_circle((64, 64), 32, paint(TwoBits::Gray));

        for i in 0..5 {
            let a = 0.2 * self.rotation + 2.0 / 5.0 * i as f64 * consts::PI;
            let b = a + consts::PI * 4.0 / 5.0;

            let first = (radius * a.cos(), radius * a.sin());
            let second = (radius * b.cos(), radius * b.sin());
            let center = (0.0, 0.0);
            let vertex = [first, second, center].map(mapper);
            render.filled_triangle(vertex, paint(TwoBits::Gray));
        }
        let radius = 48.0 - 8.0 * self.rotation.sin();
        for i in 0..5 {
            let a = -1.0 / 3.0 * self.rotation + 2.0 / 5.0 * i as f64 * consts::PI;
            let b = a + consts::PI * 4.0 / 5.0;

            let first = center
                + (
                    (radius * a.cos()).round() as i32,
                    (radius * a.sin()).round() as i32,
                );
            let second = center
                + (
                    (radius * b.cos()).round() as i32,
                    (radius * b.sin()).round() as i32,
                );
            render.line(first, second, paint(TwoBits::Red));
        }
    }
}

#[derive(Copy, Clone, PartialEq)]
enum TwoBits {
    Black,
    Gray,
    White,
    Red,
}

impl From<u8> for TwoBits {
    #[inline]
    fn from(value: u8) -> Self {
        match value {
            0 => TwoBits::Black,
            1 => TwoBits::Gray,
            2 => TwoBits::White,
            3 => TwoBits::Red,
            _ => TwoBits::Black,
        }
    }
}

struct Converter;

impl color::Converter for Converter {
    type Palette = TwoBits;
    #[inline]
    fn convert(&self, color: &Self::Palette) -> [u8; 4] {
        {
            match color {
                TwoBits::Black => [0x00, 0x00, 0x00, 0xff],
                TwoBits::Gray => [0x80, 0x80, 0x80, 0xff],
                TwoBits::White => [0xff, 0xff, 0xff, 0xff],
                TwoBits::Red => [0xff, 0x40, 0x40, 0xff],
            }
        }
    }
}
