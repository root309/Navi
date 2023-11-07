use git2::{Repository, BranchType};
use std::process::Command;

// ブランチリスト
pub fn list_branches() -> Vec<String> {
    let repo = Repository::open(".").unwrap();
    let branches = repo.branches(Some(BranchType::Local)).unwrap();
    branches.map(|b| b.unwrap().0.name().unwrap().unwrap().to_string()).collect()
}
pub fn list_remote_branches() -> Vec<String> {
    let repo = Repository::open(".").unwrap();
    let branches = repo.branches(Some(BranchType::Remote)).unwrap();
    branches.map(|b| b.unwrap().0.name().unwrap().unwrap().replace("refs/remotes/", "")).collect()
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
// ローカルに追跡ブランチを作成する関数
pub fn create_tracking_branch(local_branch_name: &str, remote_branch_name: &str) -> Result<(), git2::Error> {
    let repo = Repository::open(".")?;
    let remote_branch = repo.find_branch(remote_branch_name, BranchType::Remote)?;
    let commit = remote_branch.into_reference().peel_to_commit()?; // referenceが最終的に指すCommitオブジェクト
    repo.branch(local_branch_name, &commit, false)?;
    Ok(())
}
pub fn git_fetch_prune() -> Result<(), std::io::Error> {
    let status = Command::new("git")
        .args(&["fetch", "--prune"])
        .status()?;

    if status.success() {
        Ok(())
    } else {
        Err(std::io::Error::new(std::io::ErrorKind::Other, "git fetch --prune failed"))
    }
}

