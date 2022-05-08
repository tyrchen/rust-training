use oso::{FromPolar, Oso, OsoError, PolarClass, ToPolar};
use std::{collections::HashSet, env, fs, hash::Hash, marker::PhantomData, sync::Arc};

#[allow(dead_code)]
#[derive(Clone, PolarClass)]
pub struct User {
    pub email: String,
    pub full_name: String,
    #[polar(attribute)]
    pub repo_roles: Vec<RepoRole>,
}

#[derive(Clone, PolarClass)]
pub struct Repo {
    #[polar(attribute)]
    pub name: String,
    #[polar(attribute)]
    pub public: bool,
}

#[derive(Clone, PolarClass)]
pub struct RepoRole {
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

#[derive(Clone)]
pub struct Permission<U, UR, R> {
    oso: Arc<Oso>,
    _user: PhantomData<U>,
    _user_role: PhantomData<UR>,
    _resource: PhantomData<R>,
}

impl<U, UR, R> Permission<U, UR, R>
where
    U: PolarClass + ToPolar,
    UR: PolarClass,
    R: PolarClass + ToPolar,
{
    pub fn try_new() -> Result<Self, OsoError> {
        let mut oso = Oso::new();
        oso.register_class(U::get_polar_class_builder().build())?;
        oso.register_class(UR::get_polar_class_builder().build())?;
        oso.register_class(R::get_polar_class_builder().build())?;

        Permission::<U, UR, R>::load_policies(&mut oso)?;

        Ok(Permission {
            oso: Arc::new(oso),
            _user: PhantomData,
            _user_role: PhantomData,
            _resource: PhantomData,
        })
    }

    pub fn is_allowed<A>(&self, actor: U, action: A, resource: R) -> Result<bool, OsoError>
    where
        A: ToPolar,
    {
        self.oso.is_allowed(actor, action, resource)
    }

    pub fn get_allowed_actions<A>(&self, actor: U, resource: R) -> Result<HashSet<A>, OsoError>
    where
        A: FromPolar + Eq + Hash,
    {
        self.oso.get_allowed_actions(actor, resource)
    }

    pub fn get_roles(&self, _actor: U, _resource: R) -> Result<HashSet<UR>, OsoError> {
        todo!()
    }

    fn load_policies(oso: &mut Oso) -> Result<(), OsoError> {
        let base_dir = env::var("POLICY_DIR")
            .unwrap_or_else(|_| format!("{}/rules", env!("CARGO_MANIFEST_DIR")));

        let files = fs::read_dir(base_dir)?
            .filter_map(|entry| entry.ok())
            .map(|entry| entry.path())
            .filter(|p| p.is_file() && p.extension() == Some(std::ffi::OsStr::new("polar")))
            .collect::<Vec<_>>();
        oso.load_files(files)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn permission_should_work() {
        let perm = Permission::<User, RepoRole, Repo>::try_new().unwrap();
        let mut user = User::new("tyr@acme.com", "Tyr");
        let repo = Repo::new("permission", false);
        user.add_role(&repo.name, "writer");
        assert!(perm.is_allowed(user.clone(), "read", repo.clone()).unwrap());
        let actions: HashSet<String> = perm.get_allowed_actions(user, repo).unwrap();
        assert_eq!(
            &actions,
            &["read".into(), "write".into()].into_iter().collect()
        );
    }
}
