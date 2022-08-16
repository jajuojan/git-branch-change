use std::env;
mod repo;
use console::Key;
use console::Term;
use dialoguer::{theme::ColorfulTheme, FuzzySelect};
use repo::Branch;
use repo::Repo;

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum BranchType {
    Local,
    Remote,
}

fn branch_to_string(repo: &Repo, branches: &Vec<Branch>) -> Vec<String> {
    //TODO: do cleanup
    //TODO: do proper handling
    let mut res: Vec<String> = vec![];
    for b in branches {
        let mut n = b.name.to_owned();

        let local_matches: Vec<Branch> = repo
            .get_matching_local_branches(b)
            .into_iter()
            .filter(|bb| bb.name != b.name)
            .collect();

        if local_matches.len() > 0 {
            n += " [";
            for i in local_matches {
                n += &i.name.to_owned();
                n += " ";
            }
            n += "]";
        }

        res.push(n);
    }

    res
}

fn main() -> std::io::Result<()> {
    let current_dir = env::current_dir().unwrap();
    let repo = Repo::new(current_dir);

    //repo.get_current_local_branch();
    //return Ok(());

    //TODO: If uncommitted changes: exit
    //TODO: if state is not clean exit.

    // TODO: how to handle several remotes?
    // TODO: test with submodules

    let mut selected_branches = BranchType::Local;
    let mut branches = repo.get_local_branches();
    //TODO: also print matching local-name for remote if found

    loop {
        //TODO: print current branch
        //TODO: print local/remote info
        let selection = FuzzySelect::with_theme(&ColorfulTheme::default())
            .items(&branch_to_string(&repo, &branches))
            .default(0)
            .interact_on_opt(&Term::stderr())?;

        match selection {
            //TODO: f-button to fetch
            //TODO: d-button to delete branch ?
            //TODO: allow creating branches from tags (shift-tab?)
            Some(result) => match result.button {
                Key::Enter => {
                    // TODO: switch branch. pub fn checkout_tree(
                    // TODO: or checkout new local branch. pub fn branch(
                    // TODO: Check for existing branches
                    let branch = branches[result.selection.unwrap()].to_owned();
                    repo.checkout_branch(&branch);
                    return Ok(());
                }
                Key::Escape => {
                    // Debug
                    //println!("{:?}", repo.get_current_local_branch());
                    return Ok(());
                }
                Key::Tab | Key::BackTab => {
                    if selected_branches == BranchType::Local {
                        //TODO: some duplicates
                        branches = repo.get_remote_branches();
                        selected_branches = BranchType::Remote;
                    } else {
                        branches = repo.get_local_branches();
                        selected_branches = BranchType::Local;
                    }
                }
                _ => {}
            },
            None => println!("TODO: none selected"),
        }
    }
}
