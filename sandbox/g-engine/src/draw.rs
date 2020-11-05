use super::GameState;
use console::Term;
use std::thread;
use std::time::Duration;

fn make_gui(i: i32) -> String {
    return format!("---{}/30-----------------------", i);
}

pub fn render(state: GameState) {
    let fps = 1000 / 30;
    let term = Term::stdout();
    let map_width = state.map.width;
    let tiles = state.map.tiles;
    let line_to_clear = map_width + 2;

    term.clear_screen();
    term.hide_cursor();

    loop {
        thread::sleep(Duration::from_millis(fps));

        let gui = make_gui(30);

        term.clear_last_lines(line_to_clear as usize);

        let mut x = 0;
        let mut y = 0;
        let mut s: String = "".to_owned();
        for tail in tiles.iter() {
            s.push_str("#");

            x += 1;
            if x > map_width as i32 - 1 {
                term.write_line(&*s);
                s = "".to_owned();
                x = 0;
                y += 1;
            }
        }
        term.write_line(&gui);
    }
}
