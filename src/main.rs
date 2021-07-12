use crate::cacher::Cacher;
use crate::cells_grid::CellsGridGame;
use crate::inp::InputReceived;
use piston_window::*;

mod cacher;
mod cells_grid;
mod inp;
mod states;

fn main() {
    let mut win: PistonWindow = WindowSettings::new("Cells Grid", [400, 400])
        .exit_on_esc(true)
        .resizable(true)
        .build()
        .unwrap_or_else(|e| {
            eprintln!("ERROR MAKING WIN: {}", e);
            std::process::exit(1);
        });

    let mut game = CellsGridGame::new(win.size());
    let mut cacher = Cacher::new();
    cacher.populate(&mut win);

    let mut mouse_pos = (0.0, 0.0);
    while let Some(e) = win.next() {
        let size = win.size();
        win.draw_2d(&e, |c, g, device| {
            game.render(size, &cacher, c, g, device);
        });

        if let Some(Button::Mouse(_)) = e.press_args() {
            game.input(InputReceived::Mouse(true, mouse_pos), win.size());
        }
        if let Some(sc) = e.mouse_scroll_args() {
            game.input(InputReceived::Scroll(sc[1], mouse_pos), win.size());
        }

        e.mouse_cursor(|p| mouse_pos = (p[0], p[1]));
    }
}
