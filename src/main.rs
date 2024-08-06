
use board::Board;


mod cell;
mod board;

slint::include_modules!();
fn main() {
    MainWindow::new().unwrap().run().unwrap();
}
