mod git_ops;
mod ui;
use crate::git_ops::git_functions;
use std::io::stdout;
use crossterm::{
    execute,
    terminal::{Clear, ClearType},
    event::{read, KeyEvent, KeyCode, KeyModifiers},
    cursor::{self, MoveTo},
};

fn main() {
    let branches = git_ops::list_branches();
    let selected_index = ui::display_palette(&branches);
    // ユーザーがEnterを押した後に画面をクリア
    execute!(stdout(), Clear(ClearType::All)).unwrap();
}
