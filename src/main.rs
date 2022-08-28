use pyxel::{Pyxel, PyxelCallback};
use rand::prelude::*;

const W: f64 = 256.0;
const H: f64 = 256.0;
const FPS: f64 = 60.0;
const DT: f64 = 1.0 / FPS;

#[derive(Default, PartialEq)]
enum Weather {
    #[default]
    Clear,
    LightRain,
    HeavyRain,
    Smog,
    Gone,
}

impl Weather {
    fn get_color(&self) -> u8 {
        match self {
            Self::Clear => 6,
            Self::LightRain => 12,
            Self::HeavyRain => 5,
            Self::Smog => 1,
            Self::Gone => 0,
        }
    }

    fn get_name(&self) -> &'static str {
        match self {
            Self::Clear => "clear",
            Self::LightRain => "light rain",
            Self::HeavyRain => "heavy rain",
            Self::Smog => "smog",
            Self::Gone => "gone",
        }
    }
}

#[derive(Default)]
struct App {
    weather: Weather,
}

impl App {
    fn init() {
        let mut pyxel = Pyxel::new(
            W as u32,
            H as u32,
            Some("olcjam2022"),
            Some(FPS as u32),
            None,
            None,
            None,
            None,
        );

        let mut app = App::default();

        pyxel
            .image(0)
            .lock()
            .load(0, 0, "res/bank0.png", &pyxel::DEFAULT_COLORS);

        pyxel.run(&mut app);
    }
}

impl PyxelCallback for App {
    fn update(&mut self, px: &mut Pyxel) {
        if px.btnp(pyxel::KEY_1, None, None) {
            self.weather = Weather::Clear;
        } else if px.btnp(pyxel::KEY_2, None, None) {
            self.weather = Weather::LightRain;
        } else if px.btnp(pyxel::KEY_3, None, None) {
            self.weather = Weather::HeavyRain;
        } else if px.btnp(pyxel::KEY_4, None, None) {
            self.weather = Weather::Smog;
        } else if px.btnp(pyxel::KEY_5, None, None) {
            self.weather = Weather::Gone;
        }
    }

    fn draw(&mut self, px: &mut Pyxel) {
        px.pal(5, self.weather.get_color());
        if self.weather == Weather::Gone {
            // dark windows
            px.pal(10, 12);
            // dark building edges
            px.pal(15, 3);
            // dead trees & grass
            px.pal(3, 2);
        }
        px.blt(0.0, 0.0, 0, 0.0, 0.0, W, H, None);
        px.pal0();
        px.text(1.0, 1.0, self.weather.get_name(), 8);
    }
}

fn main() {
    App::init();
}
