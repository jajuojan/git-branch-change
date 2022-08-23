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

#[derive(PartialEq, Eq, Clone, Debug, Default)]
pub enum State {
    #[default]
    LocalBranches,
    RemoteBranches,
    SeveralLocalBranches,
}

fn format_branch_to_string(
    repo: &Repo,
    branches: &Vec<Branch>,
    current_state: &State,
) -> Vec<String> {
    if *current_state == State::LocalBranches || *current_state == State::SeveralLocalBranches {
        return branch_to_string(branches, &None);
    }

    return branch_to_string(branches, &Some(repo.get_local_branches()));
}

fn main() -> std::io::Result<()> {
    let current_dir = env::current_dir().unwrap();
    let repo = Repo::new(current_dir);

    // TODO: If uncommitted changes: exit
    // TODO: if state is not clean exit.
    // TODO: how to handle several remotes?
    // TODO: test with submodules
    // TODO: highlight current branch on UI

    let mut current_state = State::LocalBranches;
    let mut branches = repo.get_local_branches();

    loop {
        let selection = FuzzySelect::with_theme(&ColorfulTheme::default())
            .items(&format_branch_to_string(&repo, &branches, &current_state))
            .default(0)
            .interact_on_opt(&Term::stderr())?;

        match selection {
            //TODO: f-button to fetch
            //TODO: d-button to delete branch ?
            //TODO: allow creating branches from tags (shift-tab?)
            Some(result) => match result.button {
                Key::Enter => {
                    let mut branch = branches[result.selection.unwrap()].to_owned();

                    if branch.branch_type == BranchType::Remote {
                        let local_branches = repo.get_matching_local_branches_for_remote(&branch);
                        if local_branches.len() == 1 {
                            branch = local_branches[0].to_owned();
                        } else if local_branches.len() > 1 {
                            current_state = State::SeveralLocalBranches;
                            branches = local_branches;
                            continue;
                        }
                    }

                    repo.checkout_branch(&branch);
                    return Ok(());
                }
                Key::Escape => {
                    if current_state == State::SeveralLocalBranches {
                        branches = repo.get_remote_branches();
                        current_state = State::RemoteBranches;
                        continue;
                    }
                    return Ok(());
                }
                Key::Tab | Key::BackTab => {
                    if current_state == State::LocalBranches {
                        branches = repo.get_remote_branches();
                        current_state = State::RemoteBranches;
                    } else {
                        branches = repo.get_local_branches();
                        current_state = State::LocalBranches;
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
