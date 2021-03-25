use colored::*;

use rand::Rng;

fn logo_plugin() {
    let windows = "\u{E62A}";
    let colored_logo = match rand::thread_rng().gen_range(0..4) {
        0 => windows.blue(),
        1 => windows.red(),
        2 => windows.yellow(),
        3 => windows.green(),
        _ => unreachable!(),
    };
    print!("{}", colored_logo);
}

fn path_plugin() {
    let cur_path = std::env::current_dir().unwrap();
    let path_str = cur_path.to_string_lossy();
    let styled_str = path_str.blue().bold();
    print!("{}", styled_str);
}

fn git_plugin() {
    let current_dir = std::env::current_dir().unwrap();
    let repo = match git2::Repository::discover(current_dir) {
        Ok(r) => r,
        Err(e) => match e.code() {
            git2::ErrorCode::NotFound => return,
            _ => panic!("{}", e),
        },
    };
    print!(" (git:");
    if repo.is_bare() {
        let bare_text = "bare".bold();
        print!("{})", bare_text);
    } else {
        let current_branch = match get_current_git_branch(&repo) {
            GitBranch::Branch(b) => b.green(),
            GitBranch::Headless => "no head".yellow(),
            GitBranch::Detached(c) => c.blue(),
        };
        print!("{}", current_branch);
        print!(")");
    }
}

enum GitBranch {
    Branch(String),
    Headless,
    Detached(String),
}

fn get_current_git_branch(repo: &git2::Repository) -> GitBranch {
    let head = match repo.revparse("HEAD") {
        Ok(h) => h,
        Err(_) => return GitBranch::Headless,
    };
    for branch in repo.branches(Some(git2::BranchType::Local)).unwrap() {
        let (b, _) = branch.unwrap();
        if b.is_head() {
            return GitBranch::Branch(b.name().unwrap().unwrap().to_string());
        }
    }
    let head_commit = match head.mode() {
        git2::RevparseMode::SINGLE => head.from().unwrap(),
        m => panic!("rev-parse mode was {:?}", m),
    };
    GitBranch::Detached(head_commit.id().to_string())
}

fn time_plugin() {
    let now = chrono::Local::now().format("%m-%d %H:%M");
    let now_styled = now.to_string().bold();
    print!("{}", now_styled);
}

fn main() {
    logo_plugin();
    print!(" [");
    path_plugin();
    print!("]");
    git_plugin();
    print!(" ");
    time_plugin();
}
