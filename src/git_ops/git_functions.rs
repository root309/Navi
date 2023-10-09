use git2::{Repository, BranchType};

pub fn list_branches() -> Vec<String> {
    let repo = Repository::open(".").unwrap();
    let branches = repo.branches(Some(BranchType::Local)).unwrap();
    branches.map(|b| b.unwrap().0.name().unwrap().unwrap().to_string()).collect()
}
pub fn checkout_branch(branch_name: &str) -> Result<(), git2::Error> {
    let repo = Repository::open(".")?;
    repo.set_head(&format!("refs/heads/{}", branch_name))?;
    Ok(())
}
pub fn delete_branch(branch_name: &str) {
    let repo = Repository::open(".").unwrap();
    let mut branch = repo.find_branch(branch_name, BranchType::Local).unwrap();  // `mut`を追加
    branch.delete().unwrap();
}
