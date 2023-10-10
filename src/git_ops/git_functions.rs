use git2::{Repository, BranchType};
use std::process::Command;

// ブランチリスト
pub fn list_branches() -> Vec<String> {
    let repo = Repository::open(".").unwrap();
    let branches = repo.branches(Some(BranchType::Local)).unwrap();
    branches.map(|b| b.unwrap().0.name().unwrap().unwrap().to_string()).collect()
}
// ブランチチェックアウト
pub fn checkout_branch(branch_name: &str) -> Result<(), git2::Error> {
    let repo = Repository::open(".")?;
    repo.set_head(&format!("refs/heads/{}", branch_name))?;
    Ok(())
}
// ブランチ削除
pub fn delete_branch(branch_name: &str) {
    let repo = Repository::open(".").unwrap();
    let mut branch = repo.find_branch(branch_name, BranchType::Local).unwrap();
    branch.delete().unwrap();
}
// TreeDiagramの取得
pub fn get_git_tree_graph() -> Vec<String> {
    let output = Command::new("git")
        .arg("log")
        .arg("--graph")
        .arg("--oneline")
        .arg("--all")
        .output()
        .expect("Failed to execute git command");

    let output_str = String::from_utf8_lossy(&output.stdout);
    output_str.lines().map(|line| line.to_string()).collect()
}