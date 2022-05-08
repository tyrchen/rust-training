use anyhow::Result;
use oso_live::{Permission, Repo, RepoRole, User};

fn main() -> Result<()> {
    let perm = Permission::<User, RepoRole, Repo>::try_new().unwrap();

    let rust_repo = Repo::new("acme/rust", false);
    let oso_repo = Repo::new("osohq/oso", true);

    let mut tyr = User::new("tyr@acme.com", "Tyr Chen");
    let mut alice = User::new("alice@acme.com", "Alice Wang");

    tyr.add_role("acme/rust", "writer");
    alice.add_role("acme/rust", "finance");

    println!(
        "tyr allowed delete: {}", // no
        perm.is_allowed(tyr.clone(), "delete", rust_repo.clone())?
    );
    println!(
        "tyr allowed write: {}", // yes
        perm.is_allowed(tyr.clone(), "write", rust_repo.clone())?
    );

    println!(
        "alice allowed write: {}", // no
        perm.is_allowed(alice.clone(), "write", rust_repo.clone())?
    );

    println!(
        "alice allowed billing: {}", // yes
        perm.is_allowed(alice, "billing", rust_repo)?
    );

    println!(
        "tyr allowed read oso: {}", // yes
        perm.is_allowed(tyr, "read", oso_repo)?
    );
    Ok(())
}
