use crate::repo::Branch;

fn remove_ref(branch_name: &String) -> String {
    if branch_name.starts_with("refs/heads/") {
        (&branch_name[11..]).to_string()
    } else if branch_name.starts_with("refs/") {
        (&branch_name[5..]).to_string()
    } else {
        branch_name.to_string()
    }
}

pub fn branch_to_string(
    branches: &Vec<Branch>,
    secondary_branches: &Option<Vec<Branch>>,
) -> Vec<String> {
    if secondary_branches.is_none() {
        return branches.iter().map(|b| remove_ref(&b.name)).collect();
    }

    let mut res: Vec<String> = vec![];
    for branch in branches {
        let matching_local_branches: Vec<&Branch> = secondary_branches
            .as_ref()
            .unwrap()
            .iter()
            .filter(
                |b| matches!(b.upstream_branch_name.to_owned(), Some(x) if x==branch.short_name),
            )
            .collect();

        let mut name = remove_ref(&branch.short_name);
        let amount_of_matches = matching_local_branches.len();

        if amount_of_matches > 0 {
            name += " [";
            for (i, b) in (&matching_local_branches).iter().enumerate() {
                name += &remove_ref(&b.short_name);
                if i < amount_of_matches - 1 {
                    name += ", ";
                }
            }
            name += "]";
        }

        res.push(name);
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::formatters::branch_to_string;
    use crate::tests::test_helpers::test_helpers::get_local_branches;

    #[test]
    fn works_with_local_branches() {
        assert_eq!(
            branch_to_string(&get_local_branches(), &None),
            vec![
                "feature/issue-001",
                "my-local-branch",
                "abrefs/my-local-branch"
            ]
        );
    }

    #[test]
    fn works_with_single_remote_branch() {
        assert_eq!(
            branch_to_string(
                &vec![Branch {
                    short_name: "origin/feature/issue-001".to_string(),
                    name: "refs/remotes/origin/feature/issue-001".to_string(),
                    commit_hash: "abcdefghijklmnopqrstuvxyz1234567890aa01".to_string(),
                    ..Default::default()
                }],
                &Some(vec![Branch {
                    short_name: "my-local-branch".to_string(),
                    name: "refs/heads/my-local-branch".to_string(),
                    commit_hash: "abcdefghijklmnopqrstuvx555234567890aa01".to_string(),
                    upstream_branch_name: Some("origin/feature/issue-001".to_string()),
                    ..Default::default()
                }])
            ),
            vec!["origin/feature/issue-001 [my-local-branch]",]
        );
    }

    #[test]
    fn works_with_single_remote_branch_several_locals() {
        assert_eq!(
            branch_to_string(
                &vec![Branch {
                    short_name: "origin/feature/issue-001".to_string(),
                    name: "refs/remotes/origin/feature/issue-001".to_string(),
                    commit_hash: "abcdefghijklmnopqrstuvxyz1234567890aa01".to_string(),
                    ..Default::default()
                }],
                &Some(vec![
                    Branch {
                        short_name: "my-local-branch".to_string(),
                        name: "refs/heads/my-local-branch".to_string(),
                        commit_hash: "abcdefghijklmnoptyutuvxyz1234567890aa01".to_string(),
                        upstream_branch_name: Some("origin/feature/issue-001".to_string()),
                        ..Default::default()
                    },
                    Branch {
                        short_name: "feature/something".to_string(),
                        name: "refs/heads/feature/something".to_string(),
                        commit_hash: "abcdefghijklmnopqrstuvxyz1234567890aa01".to_string(),
                        upstream_branch_name: Some("origin/feature/issue-001".to_string()),
                        ..Default::default()
                    }
                ])
            ),
            vec!["origin/feature/issue-001 [my-local-branch, feature/something]",]
        );
    }

    #[test]
    fn works_with_several_remote_branch_several_locals() {
        assert_eq!(
            branch_to_string(
                &vec![
                    Branch {
                        short_name: "origin/feature/issue-001".to_string(),
                        name: "refs/remotes/origin/feature/issue-001".to_string(),
                        commit_hash: "abcdefghijklmno4680tuvxyz1234567890aa01".to_string(),
                        ..Default::default()
                    },
                    Branch {
                        short_name: "origin/feature/issue-001_new".to_string(),
                        name: "refs/remotes/origin/feature/issue-001_new".to_string(),
                        commit_hash: "abcdefghijklmno456stuvxyz1234567890aa01".to_string(),
                        ..Default::default()
                    },
                    Branch {
                        short_name: "origin/feature/issue-002".to_string(),
                        name: "refs/remotes/origin/feature/issue-002".to_string(),
                        commit_hash: "abcdefghijklmnopqrstuvxyz1234567890aa02".to_string(),
                        ..Default::default()
                    }
                ],
                &Some(vec![
                    Branch {
                        short_name: "should-not-match".to_string(),
                        name: "refs/heads/should-not-match".to_string(),
                        commit_hash: "abcdefghijklmnopqrstuvxyz1234567890aa09".to_string(),
                        upstream_branch_name: None,
                        ..Default::default()
                    },
                    Branch {
                        short_name: "my-local-branch".to_string(),
                        name: "refs/heads/my-local-branch".to_string(),
                        commit_hash: "abcdefghijklmnopqrstuvxyz1234567890aa01".to_string(),
                        upstream_branch_name: Some("origin/feature/issue-001".to_string()),
                        ..Default::default()
                    },
                    Branch {
                        short_name: "feature/something".to_string(),
                        name: "refs/heads/feature/something".to_string(),
                        commit_hash: "abcdefghijklmnopqrstuvxyz1234567890aa01".to_string(),
                        upstream_branch_name: Some("origin/feature/issue-001".to_string()),
                        ..Default::default()
                    }
                ])
            ),
            vec![
                "origin/feature/issue-001 [my-local-branch, feature/something]",
                "origin/feature/issue-001_new",
                "origin/feature/issue-002"
            ],
        );
    }
}
