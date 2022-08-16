use std::path::Path;

use git2::{BranchType, Oid, Reference, Repository};

pub struct Repo {
    pub raw_repo: Repository,
}

#[derive(Clone, Debug)]
pub struct Branch {
    pub name: String,
    pub commit_hash: String,
    //pub branch_type: BranchTyp,
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

    // TODO: rework this after POC is working
    fn get_branch_names(&self, filter: BranchType) -> Vec<String> {
        let branches = self.raw_repo.branches(Some(filter)).unwrap();
        let mut res = Vec::new();
        for branch in branches {
            let b = branch.unwrap().0.name().unwrap().unwrap().to_owned();
            res.push(b);
        }
        res
    }

    fn reference_to_branch(&self, r: &Reference) -> Branch {
        // TODO: Do proper error handling
        Branch {
            name: r.name().unwrap().to_string(),
            commit_hash: r.target().unwrap().to_string(),
        }
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
        }
    }

    pub fn get_local_branches(&self) -> Vec<Branch> {
        let branches = self.get_branch_names(BranchType::Local);
        branches
            .iter()
            .map(|f| self.get_branch_by_name(&f))
            .collect()
    }

    pub fn get_remote_branches(&self) -> Vec<Branch> {
        let branches = self.get_branch_names(BranchType::Remote);
        branches
            .iter()
            .map(|f| self.get_branch_by_name(&f))
            .collect()
    }

    pub fn get_current_local_branch(&self) -> Branch {
        let r = self.raw_repo.head().unwrap();
        self.reference_to_branch(&r)
    }

    pub fn get_matching_local_branches(&self, branch: &Branch) -> Vec<Branch> {
        self.get_local_branches()
            .into_iter()
            .filter(|b| b.commit_hash == branch.commit_hash)
            .collect::<Vec<Branch>>()
    }

    pub fn checkout_branch(&self, branch: &Branch) {
        //TODO: do proper error handling
        //TODO: Check if we are checking out a remote branch vs local
        //TODO: probably some unneeded redundancy, clean-up

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

        let b = self.get_branch_by_name(&new_branch_name);
        let r11 = self.raw_repo.set_head(&b.name);
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

    pub fn get_status(&self) {
        //TODO: implement
        let statuses = self.raw_repo.statuses(None).unwrap();
        //println!("{:?}", statuses.);
    }
}
