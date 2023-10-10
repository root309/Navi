use git2::{Repository, BranchType};
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
// TODO:gitの機能を以下に書く