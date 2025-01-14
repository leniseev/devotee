use std::f64::consts;
use std::time::Duration;

use devotee::app;
use devotee::app::config;
use devotee::app::context::Context;
use devotee::app::input::key_mouse::{KeyMouse, VirtualKeyCode};
use devotee::app::root::Root;
use devotee::app::setup;
use devotee::visual::color;
use devotee::visual::prelude::*;
use devotee::visual::sprite::Sprite;
use rodio::source::{SineWave, Source};

fn main() {
    let init_config = setup::Setup::<Config>::new(
        Sprite::with_color(FourBits::Black),
        Default::default(),
        |_| Default::default(),
    )
    .with_title("twister")
    .with_scale(2);
    let app = app::App::with_setup(init_config).unwrap();

    app.run();
}

struct Config;

impl config::Config for Config {
    type Root = Twister;
    type Converter = Converter;
    type Input = KeyMouse;
    type RenderTarget = Sprite<FourBits, 128, 128>;

    fn converter() -> Self::Converter {
        Converter { transparent: None }
    }

    fn background_color() -> FourBits {
        FourBits::Black
    }
}

#[derive(Default)]
struct Twister {
    rotation: f64,
    twist: f64,
}

impl Root<Config> for Twister {
    fn update(&mut self, update: &mut Context<Config>) {
        if update.input().keys().just_pressed(VirtualKeyCode::Escape) {
            update.shutdown();
        }
        let delta = update.delta().as_secs_f64();
        if update.input().keys().is_pressed(VirtualKeyCode::Left) {
            self.rotation += delta;
        }
        if update.input().keys().is_pressed(VirtualKeyCode::Right) {
            self.rotation -= delta;
        }
        if update.input().keys().just_pressed(VirtualKeyCode::Space) {
            self.rotation = 0.0;
            self.twist = 0.0;

            update.sound_system_mut().map(|s| {
                s.play(Box::new(
                    SineWave::new(500.0).take_duration(Duration::from_secs_f64(0.05)),
                ))
            });
        }
        self.twist += delta;
    }

    fn render(&self, render: &mut Sprite<FourBits, 128, 128>) {
        let mut render = render.painter();
        render.clear(0.into());
        let resolution_x = render.width();
        let resolution_y = render.height();
        let rotation = 2.0 * self.rotation;
        let center = resolution_y as f64 / 2.0;

        render.rect_f(
            (resolution_x / 6, resolution_y / 6),
            (5 * resolution_x / 6, 5 * resolution_y / 6),
            draw(14.into()),
        );

        render.rect_f(
            (resolution_x / 4, resolution_y / 4),
            (3 * resolution_x / 4, 3 * resolution_y / 4),
            draw(15.into()),
        );

        let twist = 4.0 * consts::PI * (consts::FRAC_PI_4 * self.twist).cos();
        let width = 16.0;

        for x in 0..resolution_x {
            let twist = x as f64 / resolution_x as f64 * twist + rotation;
            let y1 = (f64::sin(twist) * width + center) as i32;
            let y2 = (f64::sin(twist + consts::FRAC_PI_2) * width + center) as i32;
            let y3 = (f64::sin(twist + consts::PI) * width + center) as i32;
            let y4 = (f64::sin(twist - consts::FRAC_PI_2) * width + center) as i32;

            if y1 < y2 {
                render.line((x, y1), (x, y2), draw(1.into()));
            }
            if y2 < y3 {
                render.line((x, y2), (x, y3), draw(2.into()));
            }
            if y3 < y4 {
                render.line((x, y3), (x, y4), draw(3.into()));
            }
            if y4 < y1 {
                render.line((x, y4), (x, y1), draw(4.into()));
            }
        }

        for y in 0..resolution_y {
            let twist = y as f64 / resolution_y as f64 * twist + rotation;
            let x1 = (f64::sin(twist) * width + center) as i32;
            let x2 = (f64::sin(twist + consts::FRAC_PI_2) * width + center) as i32;
            let x3 = (f64::sin(twist + consts::PI) * width + center) as i32;
            let x4 = (f64::sin(twist - consts::FRAC_PI_2) * width + center) as i32;

            if x1 < x2 {
                render.line((x1, y), (x2, y), draw(5.into()));
            }
            if x2 < x3 {
                render.line((x2, y), (x3, y), draw(6.into()));
            }
            if x3 < x4 {
                render.line((x3, y), (x4, y), draw(7.into()));
            }
            if x4 < x1 {
                render.line((x4, y), (x1, y), draw(8.into()));
            }
        }
    }
}

#[derive(Copy, Clone, PartialEq)]
enum FourBits {
    Black,
    DarkBlue,
    Eggplant,
    DarkGreen,
    Brown,
    DirtyGray,
    Gray,
    White,
    Red,
    Orange,
    Yellow,
    Green,
    LightBlue,
    Purple,
    Pink,
    Beige,
}

impl From<u8> for FourBits {
    #[inline]
    fn from(value: u8) -> Self {
        match value {
            0 => FourBits::Black,
            1 => FourBits::DarkBlue,
            2 => FourBits::Eggplant,
            3 => FourBits::DarkGreen,
            4 => FourBits::Brown,
            5 => FourBits::DirtyGray,
            6 => FourBits::Gray,
            7 => FourBits::White,
            8 => FourBits::Red,
            9 => FourBits::Orange,
            10 => FourBits::Yellow,
            11 => FourBits::Green,
            12 => FourBits::LightBlue,
            13 => FourBits::Purple,
            14 => FourBits::Pink,
            15 => FourBits::Beige,
            _ => FourBits::Black,
        }
    }
}

impl color::Color for FourBits {
    fn mix(self, other: Self) -> Self {
        match other {
            FourBits::Black => self,
            value => value,
        }
    }
}

struct Converter {
    transparent: Option<FourBits>,
}

impl color::Converter for Converter {
    type Palette = FourBits;
    #[inline]
    fn convert(&self, color: &Self::Palette) -> u32 {
        if matches!(&self.transparent, Some(transparent) if *transparent == *color) {
            return 0x00000000;
        }
        {
            match color {
                FourBits::Black => 0x00000000,
                FourBits::DarkBlue => 0x001d2b53,
                FourBits::Eggplant => 0x007e2553,
                FourBits::DarkGreen => 0x00008751,
                FourBits::Brown => 0x00ab5236,
                FourBits::DirtyGray => 0x005f574f,
                FourBits::Gray => 0x00c2c3c7,
                FourBits::White => 0x00fff1e8,
                FourBits::Red => 0x00ff004d,
                FourBits::Orange => 0x00ffa300,
                FourBits::Yellow => 0x00ffec27,
                FourBits::Green => 0x0000e436,
                FourBits::LightBlue => 0x0029adff,
                FourBits::Purple => 0x0083769c,
                FourBits::Pink => 0x00ff77a8,
                FourBits::Beige => 0x00ffccaa,
            }
        }
    }
}
