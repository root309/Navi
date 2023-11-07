use git2::{Repository, BranchType, Commit, Oid, Error as Git2Error};
use std::process::Command;
use std::io;
use dialoguer::{Select, Input};
use std::fmt;

// エラー型を統合
#[derive(Debug)]
pub enum Error {
    Git(Git2Error),
    Io(io::Error),
}

impl From<Git2Error> for Error {
    fn from(err: Git2Error) -> Self {
        Error::Git(err)
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::Io(err)
    }
}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Git(err) => write!(f, "Git error: {}", err),
            Error::Io(err) => write!(f, "IO error: {}", err),
        }
    }
}
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
pub fn create_branch_from_commit_interactive() -> Result<(), Error> {
    let repo = Repository::open(".")?;

    let commits = get_commits(&repo)?;

    let selections: Vec<String> = commits.iter()
        .map(|commit| format!("{} - {}", commit.id(), commit.summary().unwrap_or("<no summary>")))
        .collect();

    let selection = Select::new()
        .with_prompt("Pick a commit to create a branch from")
        .default(0)
        .items(&selections[..])
        .interact()?;

    let branch_name: String = Input::new()
        .with_prompt("Enter new branch name")
        .interact_text()?;

    let commit = &commits[selection];
    repo.branch(&branch_name, &commit, false)?;

    println!("Branch '{}' created from commit '{}'", branch_name, commit.id());
    Ok(())
}

// コミットのVecを取得する関数
fn get_commits(repo: &Repository) -> Result<Vec<Commit>, Git2Error> {
    let mut revwalk = repo.revwalk()?;
    revwalk.push_head()?;
    revwalk.set_sorting(git2::Sort::TIME)?;

    let commits: Result<Vec<_>, _> = revwalk
        .map(|id| id.and_then(|id| repo.find_commit(id)))
        .collect();

    commits
}
