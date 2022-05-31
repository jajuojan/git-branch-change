use git2::{BranchType, Repository};

fn main() {
    let repo = match Repository::open("C:\\temp\\temp-repo") {
        Ok(repo) => repo,
        Err(e) => panic!("failed to open: {}", e),
    };

    let branches = repo.branches(Some(BranchType::Local)).unwrap();
    for branch in branches {
        println!("{}", branch.unwrap().0.name().unwrap().unwrap());
    }
}
