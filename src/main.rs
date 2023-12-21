mod git_ops;
mod ui;
use crate::git_ops::git_functions;
use crate::git_ops::git_functions::{
    create_branch_from_commit_interactive, list_branches, list_remote_branches,
};
use crossterm::{
    cursor::{MoveTo, Show, Hide},
    style::Print,
    execute,
    event::{read, Event, KeyCode},
    terminal::{
        disable_raw_mode, enable_raw_mode, Clear, ClearType, EnterAlternateScreen,
        LeaveAlternateScreen,
    },
};
use std::io::{stdout, Write};
use std::process;

fn main() -> crossterm::Result<()> {
    enable_raw_mode().unwrap();

    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen, Hide)?; // 代替スクリーンを開始し、カーソルを隠す

    let actions = ["List and select branches", "Create branch from a commit"];
    let mut selected = 0;

    loop {
        execute!(stdout, MoveTo(0, 0), Clear(ClearType::All))?; // 画面をクリアし、カーソルを左上に移動

        // メニューの表示
        for (index, action) in actions.iter().enumerate() {
            execute!(stdout, MoveTo(0, index as u16))?; // カーソルを適切な位置に移動
            if index == selected {
                execute!(stdout, Print(format!("> {}\n", action)))?; // Printコマンドを使用
            } else {
                execute!(stdout, Print(format!("  {}\n", action)))?; // Printコマンドを使用
            }
        }
        stdout.flush()?;

        // キー入力の処理
        match read()? {
            Event::Key(event) => {
                match event.code {
                    KeyCode::Char('k') => if selected > 0 { selected -= 1 },
                    KeyCode::Char('j') => if selected < actions.len() - 1 { selected += 1 },
                    KeyCode::Enter => break,
                    KeyCode::Char('q') => {
                        disable_raw_mode().unwrap();
                        execute!(
                            stdout,
                            LeaveAlternateScreen, // 代替スクリーンを終了
                            Show
                        ).unwrap();
                        std::process::exit(0);
                    }
                    _ => (),
                }
            }
            _ => (),
        }
    }

    // 選択されたアクションに基づいて処理
    match selected {
        0 => {
            let local_branches = list_branches();
            let remote_branches = list_remote_branches();
            let mut all_branches = local_branches;
            all_branches.extend(remote_branches.iter().cloned());
            ui::display_palette(&all_branches); // ブランチリストを表示する関数を呼び出し
        }
        1 => {
            if let Err(e) = git_ops::create_branch_from_commit_interactive() {
                eprintln!("Error: {}", e);
            }
        }
        _ => unreachable!(),
    }

    execute!(stdout, LeaveAlternateScreen, Show)?; // 代替スクリーンを終了し、カーソルを表示
    disable_raw_mode().unwrap();
    Ok(())
}