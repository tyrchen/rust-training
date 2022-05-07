actor User {}

resource Repo {
    permissions = ["read", "write", "delete", "billing"];
    roles = ["reader", "writer", "admin", "finance"];

    "read" if "reader";
    "write" if "writer";
    "delete" if "admin";
    "billing" if "finance";

    "reader" if "writer";
    "writer" if "admin";

    "read" if "finance";
}

has_role(actor: User, role_name: String, repo: Repo) if
    role in actor.repo_roles and
    role.name = role_name and
    repo.name = role.repo;

has_permission(_actor: User, "read", repo: Repo) if
    repo.public;

allow(actor, action, resource) if
    has_permission(actor, action, resource);
