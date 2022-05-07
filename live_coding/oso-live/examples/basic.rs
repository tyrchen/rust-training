use anyhow::Result;
use oso::{Oso, PolarClass};

#[allow(dead_code)]
#[derive(Clone, PolarClass)]
struct User {
    pub email: String,
    pub full_name: String,
    #[polar(attribute)]
    pub repo_roles: Vec<RepoRole>,
}

#[derive(Clone, PolarClass)]
struct Repo {
    #[polar(attribute)]
    pub name: String,
    #[polar(attribute)]
    pub public: bool,
}

#[derive(Clone, PolarClass)]
struct RepoRole {
    #[polar(attribute)]
    pub name: String,
    #[polar(attribute)]
    pub repo: String,
}

impl User {
    pub fn new(email: impl Into<String>, full_name: impl Into<String>) -> User {
        User {
            email: email.into(),
            full_name: full_name.into(),
            repo_roles: Vec::new(),
        }
    }

    pub fn add_role(&mut self, repo: impl Into<String>, role: impl Into<String>) {
        self.repo_roles.push(RepoRole {
            name: role.into(),
            repo: repo.into(),
        });
    }
}

impl Repo {
    pub fn new(name: impl Into<String>, public: bool) -> Repo {
        Repo {
            name: name.into(),
            public,
        }
    }
}

fn main() -> Result<()> {
    let mut oso = Oso::new();
    oso.register_class(User::get_polar_class_builder().build())?;
    oso.register_class(Repo::get_polar_class_builder().build())?;
    oso.register_class(RepoRole::get_polar_class_builder().build())?;

    oso.load_files(vec![format!(
        "{}/examples/app.polar",
        env!("CARGO_MANIFEST_DIR")
    )])?;

    let rust_repo = Repo::new("acme/rust", false);
    let oso_repo = Repo::new("osohq/oso", true);

    let mut tyr = User::new("tyr@acme.com", "Tyr Chen");
    let mut alice = User::new("alice@acme.com", "Alice Wang");

    tyr.add_role("acme/rust", "writer");
    alice.add_role("acme/rust", "finance");

    println!(
        "tyr allowed delete: {}", // no
        oso.is_allowed(tyr.clone(), "delete", rust_repo.clone())?
    );
    println!(
        "tyr allowed write: {}", // yes
        oso.is_allowed(tyr.clone(), "write", rust_repo.clone())?
    );

    println!(
        "alice allowed write: {}", // no
        oso.is_allowed(alice.clone(), "write", rust_repo.clone())?
    );

    println!(
        "alice allowed billing: {}", // no
        oso.is_allowed(alice, "billing", rust_repo)?
    );

    println!(
        "tyr allowed read oso: {}", // no
        oso.is_allowed(tyr, "read", oso_repo)?
    );
    Ok(())
}
