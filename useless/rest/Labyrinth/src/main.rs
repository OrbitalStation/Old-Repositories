use bear_lib_terminal::{
    terminal::{self, Event, KeyCode},
    geometry::Rect
};
use labyrinth::{
    field,
    player::{self, Direction}
};

fn main() {
    labyrinth::init();

    loop {
        if field::is_updated() || player::is_visibility_updated() { player::show_visible_area() }

        if let Some(event) = terminal::read_event() {
            match event {
                Event::Close => labyrinth::safe_exit(),
                Event::KeyPressed { key, .. } => {
                    terminal::clear(Some(Rect::from_point_values(0, 0, 40, 1)));
                    match key {
                        KeyCode::Up    => player::r#move(Direction::Up),
                        KeyCode::Right => player::r#move(Direction::Right),
                        KeyCode::Down  => player::r#move(Direction::Down),
                        KeyCode::Left  => player::r#move(Direction::Left),
                        KeyCode::Escape => labyrinth::safe_exit(),
                        _ => { }
                    }
                }
                _ => { }
            }
        }

        player::check();
        player::show_interface();
        labyrinth::tick::check();
        terminal::refresh();
    }
}
