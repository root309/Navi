mod git_ops;
mod ui;
use crate::git_ops::git_functions;
use std::io::stdout;
use crossterm::{
    execute,
    terminal::{Clear, ClearType, enable_raw_mode, disable_raw_mode},
};
use crate::git_ops::git_functions::{list_branches, list_remote_branches, create_branch_from_commit_interactive};
use dialoguer::Select;

fn main() {
    // ターミナルを初期化
    enable_raw_mode().unwrap();

    // アクションの選択肢
    let actions = ["List and select branches", "Create branch from a commit"];

    // メインメニューを表示
    let action = Select::new()
        .with_prompt("Choose an action")
        .default(0)
        .items(&actions[..])
        .interact()
        .unwrap();

    // 選択されたアクションに基づいて処理
    match action {
        0 => {
            let local_branches = list_branches();
            let remote_branches = list_remote_branches();
            let mut all_branches = local_branches;
            all_branches.extend(remote_branches.iter().cloned());
            ui::display_palette(&all_branches); // ブランチリストを表示する関数を呼び出し
        },
        1 => {
            if let Err(e) = git_ops::create_branch_from_commit_interactive() {
                eprintln!("Error: {}", e);
            }
        },
        _ => unreachable!(),
    }

    // ターミナルを終了
    execute!(stdout(), Clear(ClearType::All)).unwrap();
    disable_raw_mode().unwrap();
}
