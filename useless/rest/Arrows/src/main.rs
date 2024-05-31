use arrows::*;
use sfml::{
    graphics::{RenderWindow, RenderTarget, Color},
    window::{
        Event,
        Key,
        mouse::Button
    }
};

const DEFAULT_CHOSEN: field::Tile = field::Tile::new(field::TileType::Arrow, field::Rotation::Up);

fn main() {
    field::Tile::load("images/tiles.png");
    save::on_ctrl_l();
    let mut window = RenderWindow::new((1000, 1000), "Arrows", Default::default(), &Default::default());
    let mut last_time = std::time::Instant::now();
    let mut chosen_tile = DEFAULT_CHOSEN;
    while window.is_open() {
        for event in window.poll_event() {
            match event {
                Event::Closed => window.close(),
                Event::MouseButtonPressed { button, x, y } => {
                    let tile = field::Tile::from_raw(x, y);
                    match button {
                        Button::LEFT => if Key::LCONTROL.is_pressed() { tile.update_ty() } else { *tile = chosen_tile },
                        Button::RIGHT => {
                            tile.update_rot();
                            chosen_tile.update_rot();
                        },
                        Button::MIDDLE => {
                            chosen_tile = *tile;
                            chosen_tile.ty.deactivate()
                        },
                        _ => ()
                    }
                },
                Event::MouseMoved { x, y } => {
                    let tile = field::Tile::from_raw(x, y);
                    if Button::LEFT.is_pressed() { if Key::LCONTROL.is_pressed() { tile.update_ty() } else { *tile = chosen_tile } }
                },
                Event::KeyPressed { code: Key::R, .. } => chosen_tile = DEFAULT_CHOSEN,
                Event::KeyPressed { code: Key::C, .. } => {
                    *field::field() = [[field::Tile::VOID; field::SIZE]; field::SIZE];
                    chosen_tile = DEFAULT_CHOSEN
                },
                Event::KeyPressed { code: Key::L, ctrl: true, .. } => save::on_ctrl_l(),
                Event::KeyPressed { code: Key::S, ctrl: true, .. } => save::on_ctrl_s(),
                _ => ()
            }
        }

        if last_time.elapsed().as_millis() > 100 {
            field::execute();
            last_time = std::time::Instant::now()
        }
        window.clear(Color::WHITE);
        field::draw(&mut window);
        window.display()
    }
    save::on_ctrl_s()
}
