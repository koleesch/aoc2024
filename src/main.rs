use color_eyre::Result;

mod tui;
use tui::{app::App, tui::init};

fn main() {
    day01::day01_01(String::from("input/day01_1.txt"));
    day01::day01_02(String::from("input/day01_1.txt"));
}

/*fn main() -> Result<()> {
    color_eyre::install()?;
    let mut terminal = init()?;
    let app_result = App::default().run(&mut terminal);
    if let Err(err) = tui::tui::restore() {
        eprintln!(
            "failed to restore terminal. Run `reset` or restart your terminal to recover: {}",
            err
        );
    }
    app_result
}
*/
