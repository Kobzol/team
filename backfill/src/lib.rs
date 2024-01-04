use octocrab::models::Permissions;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Repo {
    pub name: String,
    pub teams: Vec<Team>,
    pub collaborators: Vec<Collaborator>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Team {
    pub name: String,
    pub permission: String,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Collaborator {
    pub name: String,
    pub permissions: Permissions,
}
