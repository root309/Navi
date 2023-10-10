use std::io::stdout;
use crossterm::{
    execute,
    event::{read, KeyEvent, KeyCode},
    terminal::{Clear, ClearType},
    cursor,
    style::{Print, SetForegroundColor, Color},
};
use crate::git_functions;


pub fn display_palette(branches: &Vec<String>) -> usize {
    let graph_lines = git_functions::get_git_tree_graph();
    // セッション生成
    execute!(stdout(), Clear(ClearType::All)).unwrap();
    let padding = 45;
    for line in &graph_lines {
        print!("{:padding$}", "", padding = padding);
        println!("{}", line);
    }
    move_cursor_to_bottom();  // まずカーソルを一番下に移動
    display_branches(&branches, 0);
    let (init_cursor_x, init_cursor_y) = cursor::position().unwrap();
    let mut current_selection = 0;

    loop {
        match read().unwrap() {
            crossterm::event::Event::Key(KeyEvent { code, .. }) => match code {
                KeyCode::Down => {
                    current_selection = (current_selection + 1) % branches.len();
                    display_branches(&branches, current_selection);
                }
                KeyCode::Up => {
                    current_selection = if current_selection == 0 {
                        branches.len() - 1
                    } else {
                        current_selection - 1
                    };
                    display_branches(&branches, current_selection);
                }
                KeyCode::Enter => {
                    handle_branch_selection(current_selection, branches);
                    execute!(stdout(), cursor::MoveTo(init_cursor_x, init_cursor_y)).unwrap();
                    return current_selection;
                }
                KeyCode::Char('q') => {
                    break;
                }
                _ => {}
            },
            _ => {}
        }
    }
    current_selection
}
fn display_branches(branches: &Vec<String>, current_selection: usize) {
    // ターミナルのサイズを取得
    let (width, height) = crossterm::terminal::size().unwrap();
    let list_start_y = (height - branches.len() as u16 - 2).max(0); // 2は上下の罫線の分
    // Boxの色変更
    execute!(stdout(), cursor::MoveTo(0, list_start_y + 1),SetForegroundColor(Color::Blue)).unwrap();
    // ボックスの上端を描画
    execute!(stdout(), cursor::MoveTo(0, list_start_y), Print("┌"), Print("─".repeat(width as usize - 2)), Print("┐")).unwrap();
    for (index, branch) in branches.iter().enumerate() {
        // ボックスの左端
        execute!(stdout(), cursor::MoveTo(0, list_start_y + 1 + index as u16), Print("│")).unwrap();
        if index == current_selection {
            print!("> {}. {}", index + 1, branch);
        } else {
            print!("  {}. {}", index + 1, branch);
        }
        // ボックスの右端
        execute!(stdout(), cursor::MoveTo(width - 1, list_start_y + 1 + index as u16), Print("│")).unwrap();
    }
        // ボックスの下端を描画
    let box_bottom_y = list_start_y + 1 + branches.len() as u16;
    execute!(stdout(), cursor::MoveTo(0, box_bottom_y), Print("└"), Print("─".repeat(width as usize - 2)), Print("┘")).unwrap();
}
// ブランチ選択時のアクションを取り扱う関数
fn handle_branch_selection(selected_index: usize, branches: &Vec<String>) {
    let actions = ["チェックアウト", "ブランチの削除"];
    let selected_action = display_submenu(&actions);

    match selected_action {
        1 => {
            if let Err(e) = git_functions::checkout_branch(&branches[selected_index]) {
                println!("エラー: {}", e);
            }
            notify_checkout(&branches[selected_index]);
        }
        2 => {
            println!("'{}' を削除しました", branches[selected_index]);
            // TODO: ブランチの削除ロジックを追加
        }
        _ => {}
    }
}

fn display_submenu(options: &[&str]) -> usize {
    let mut current_selection = 0;

    loop {
        execute!(stdout(), Clear(ClearType::All)).unwrap();

        for (index, option) in options.iter().enumerate() {
            if index == current_selection {
                println!("> {}. {}", index + 1, option);
            } else {
                println!("  {}. {}", index + 1, option);
            }
        }

        match read().unwrap() {
            crossterm::event::Event::Key(KeyEvent { code, .. }) => match code {
                KeyCode::Down => {
                    current_selection = (current_selection + 1) % options.len();
                }
                KeyCode::Up => {
                    current_selection = if current_selection == 0 {
                        options.len() - 1
                    } else {
                        current_selection - 1
                    };
                }
                KeyCode::Enter => {
                    return current_selection + 1;
                }
                _ => {}
            },
            _ => {}
        }
    }
}

fn notify_checkout(branch: &str) {
    execute!(stdout(), Clear(ClearType::All)).unwrap();
    println!("'{}' をチェックアウトしました。Press Enter", branch);
    read().unwrap();
}
// カーソルをターミナルの一番下に移動
fn move_cursor_to_bottom() {
    let (cols, rows) = crossterm::terminal::size().unwrap();
    execute!(stdout(), cursor::MoveTo(0, rows - 1)).unwrap();
}
