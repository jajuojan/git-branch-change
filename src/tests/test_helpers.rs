//TODO: check if these are needed
#[cfg(test)]
pub mod test_helpers {
    use crate::repo::Branch;

    pub fn get_local_branches() -> Vec<Branch> {
        vec![
            Branch {
                name: "refs/feature/issue-001".to_string(),
                commit_hash: "abcdefghijklmnopqrstuvxyz1234567890aa01".to_string(),
                ..Default::default()
            },
            Branch {
                name: "refs/my-local-branch".to_string(),
                commit_hash: "abcdefghijklmnopqrstuvxyz1234567890aa02".to_string(),
                ..Default::default()
            },
            Branch {
                name: "refs/abrefs/my-local-branch".to_string(),
                commit_hash: "abcdefghijklmnopqrstuvxyz1234567890aa02".to_string(),
                ..Default::default()
            },
        ]
    }

    pub fn get_remote_branches() -> Vec<Branch> {
        vec![
            Branch {
                name: "refs/remotes/origin/feature/issue-001".to_string(),
                commit_hash: "abcdefghijklmnopqrstuvxyz1234567890aa01".to_string(),
                ..Default::default()
            },
            Branch {
                name: "refs/remotes/origin/feature/issue-002".to_string(),
                commit_hash: "abcdefghijklmnopqrstuvxyz1234567890aa02".to_string(),
                ..Default::default()
            },
            Branch {
                name: "refs/remotes/origin/feature/issue-003".to_string(),
                commit_hash: "abcdefghijklmnopqrstuvxyz1234567890aa03".to_string(),
                ..Default::default()
            },
            Branch {
                name: "refs/remotes/origin/feature/issue-004".to_string(),
                commit_hash: "abcdefghijklmnopqrstuvxyz1234567890aa04".to_string(),
                ..Default::default()
            },
        ]
    }
}
