use std::env;
mod formatters;
mod repo;
use console::Key;
use console::Term;
use dialoguer::{theme::ColorfulTheme, FuzzySelect};
use formatters::branch_to_string;
use repo::Branch;
use repo::BranchType;
use repo::Repo;

fn branch_to_string2(repo: &Repo, branches: &Vec<Branch>, branch_type: &BranchType) -> Vec<String> {
    if *branch_type == BranchType::Local {
        return branch_to_string(branches, &None);
    }

    let local_branches: Vec<Branch> = repo.get_local_branches();
    return branch_to_string(branches, &Some(local_branches));
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

    let mut selected_branch_type = BranchType::Local;
    let mut branches = repo.get_local_branches();
    //TODO: also print matching local-name for remote if found

    loop {
        //TODO: print current branch
        //TODO: print local/remote info
        let selection = FuzzySelect::with_theme(&ColorfulTheme::default())
            .items(&branch_to_string2(&repo, &branches, &selected_branch_type))
            .default(0)
            .interact_on_opt(&Term::stderr())?;

        match selection {
            //TODO: f-button to fetch
            //TODO: d-button to delete branch ?
            //TODO: allow creating branches from tags (shift-tab?)
            Some(result) => match result.button {
                Key::Enter => {
                    // TODO: switch branch if already in local.
                    // TODO: or checkout new local branch.
                    // TODO: if several local branches, open new menu to select witch to checkout
                    let branch = branches[result.selection.unwrap()].to_owned();
                    repo.checkout_branch(&branch);
                    return Ok(());
                }
                Key::Escape => {
                    return Ok(());
                }
                Key::Tab | Key::BackTab => {
                    if selected_branch_type == BranchType::Local {
                        branches = repo.get_remote_branches();
                        selected_branch_type = BranchType::Remote;
                    } else {
                        branches = repo.get_local_branches();
                        selected_branch_type = BranchType::Local;
                    }
                }
                _ => {}
            },
            None => println!("TODO: none selected"),
        }
    }
}

#[cfg(test)]
mod tests {
    pub mod test_helpers;
}
