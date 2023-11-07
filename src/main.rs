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
    let local_branches = git_ops::list_branches();
    let remote_branches = git_ops::list_remote_branches();
    let mut all_branches = local_branches;
    all_branches.extend(remote_branches.iter().cloned());
    let selected_index = ui::display_palette(&all_branches);
    // ユーザーがEnterを押した後に画面をクリア
    execute!(stdout(), Clear(ClearType::All)).unwrap(); 
}
