use std::path::Path;

use git2::{Branch as RawBranch, BranchType as RawBranchType, Oid, Repository};

pub struct Repo {
    pub raw_repo: Repository,
}

#[derive(PartialEq, Eq, Clone, Debug, Default)]
pub enum BranchType {
    #[default]
    Local,
    Remote,
}

impl From<RawBranchType> for BranchType {
    fn from(branch_type_raw: RawBranchType) -> Self {
        match branch_type_raw {
            RawBranchType::Local => BranchType::Local,
            RawBranchType::Remote => BranchType::Remote,
        }
    }
}

#[derive(Default, Clone, Debug)]
pub struct Branch {
    pub short_name: String,
    pub name: String,
    pub commit_hash: String,
    pub upstream_branch_name: Option<String>,
    pub branch_type: BranchType,
}

impl Repo {
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        let raw_repo = match Repository::open(path) {
            Ok(repo) => repo,
            // TODO: Do a proper error handling here
            Err(e) => panic!("failed to open: {}", e),
        };
        Self { raw_repo }
    }

    fn get_branch_by_name(&self, branch_name: &str) -> Branch {
        let (object, reference) = self
            .raw_repo
            .revparse_ext(branch_name)
            .expect("Object not found");

        // TODO: Do proper error handling
        Branch {
            name: reference.unwrap().name().unwrap().to_string(),
            commit_hash: object.id().to_string(),
            ..Default::default()
        }
    }

    fn branch_to_branch(&self, raw_branch: RawBranch, branch_type_raw: &RawBranchType) -> Branch {
        let mut upstream_branch_name = None;
        if raw_branch.upstream().is_ok() && raw_branch.upstream().unwrap().name().is_ok() {
            upstream_branch_name = Some(
                raw_branch
                    .upstream()
                    .unwrap()
                    .name()
                    .unwrap()
                    .unwrap()
                    .to_string(),
            );
        }

        let short_name = (&raw_branch).name().unwrap().unwrap().to_string();
        let r = (raw_branch).into_reference();
        let commit_hash = if (&r).target().is_some() {
            (&r).target().unwrap().to_string()
        } else {
            "".to_string()
        };

        Branch {
            short_name,
            upstream_branch_name,
            commit_hash,
            name: (&r).name().unwrap().to_string(),
            branch_type: BranchType::from(*branch_type_raw),
            ..Default::default()
        }
    }

    fn get_branches(&self, branch_type: RawBranchType) -> Vec<Branch> {
        let raw_branches = self.raw_repo.branches(Some(branch_type)).unwrap();
        let branches: Vec<Branch> = raw_branches
            .into_iter()
            .map(|b| {
                let (aa, _) = b.unwrap();
                self.branch_to_branch(aa, &branch_type)
            })
            .collect();
        branches
    }

    pub fn get_local_branches(&self) -> Vec<Branch> {
        self.get_branches(RawBranchType::Local)
    }

    pub fn get_remote_branches(&self) -> Vec<Branch> {
        //TODO: remove HEAD
        self.get_branches(RawBranchType::Remote)
    }

    /*pub fn get_current_local_branch(&self) -> Branch {
        let r = self.raw_repo.head().unwrap();
        self.reference_to_branch(&r)
    }*/

    fn checkout_local(&self, branch: &Branch) {
        //TODO: do proper error handling
        //TODO: probably some unneeded redundancy, clean-up
        let r11 = self.raw_repo.set_head(&branch.name);
        match r11 {
            Ok(_) => (),
            Err(error) => panic!("TODO: set_head failed {:?}", error),
        };

        let r = self.raw_repo.checkout_head(None);
        match r {
            Ok(_) => (),
            Err(error) => panic!("TODO: checkout_head failed {:?}", error),
        };

        let r5 = self.raw_repo.checkout_index(None, None);
        match r5 {
            Ok(_) => (),
            Err(error) => panic!("TODO: checkout_index failed {:?}", error),
        };
    }

    fn checkout_remote(&self, branch: &Branch) {}

    fn branch_and_checkout_remote(&self, branch: &Branch) {
        //TODO: do proper error handling
        //TODO: probably some unneeded redundancy, clean-up

        // TODO: Check for existing branches
        // TODO: handle other than origin
        let new_branch_name = branch.name.replace("refs/remotes/origin/", "");
        //println!("{:?}, {:?}", branch.name, &new_branch_name);

        let oid = Oid::from_str(&branch.commit_hash).unwrap();
        let r2 = self.raw_repo.find_annotated_commit(oid);
        let annotated_commit = match r2 {
            Ok(a) => a,
            Err(error) => panic!("TODO: find_annotated_commit failed {:?}", error),
        };

        let r3 =
            self.raw_repo
                .branch_from_annotated_commit(&new_branch_name, &annotated_commit, false);
        match r3 {
            Ok(_) => (),
            Err(error) => panic!("TODO: branch_from_annotated_commit failed {:?}", error),
        };

        let treeish = self.raw_repo.revparse_single(&oid.to_string()).unwrap();
        let r6 = self.raw_repo.checkout_tree(&treeish, None);
        match r6 {
            Ok(_) => (),
            Err(error) => panic!("TODO: checkout_index failed {:?}", error),
        };

        let new_local_branch = self.get_branch_by_name(&new_branch_name);
        self.checkout_local(&new_local_branch);
    }

    pub fn checkout_branch(&self, branch: &Branch) {
        match branch.branch_type {
            BranchType::Local => self.checkout_local(&branch),
            BranchType::Remote => self.branch_and_checkout_remote(&branch),
        }
    }

    /*pub fn get_status(&self) {
        //TODO: implement
        let statuses = self.raw_repo.statuses(None).unwrap();
        //println!("{:?}", statuses.);
    }*/
}
