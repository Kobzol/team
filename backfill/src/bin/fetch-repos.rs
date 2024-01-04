use backfill::{Collaborator, Repo, Team};
use octocrab::models::Repository;
use octocrab::Octocrab;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let token = ""; // TODO: fill
    let org = "rust-lang";

    let client = octocrab::OctocrabBuilder::new()
        .personal_token(token.to_string())
        .build()?;
    let mut gh_repos = vec![];
    let mut page = 0u32;
    loop {
        let mut repos = client
            .orgs(org)
            .list_repos()
            .page(page)
            .per_page(100)
            .send()
            .await?;
        for repo in repos.take_items() {
            gh_repos.push(repo);
        }

        if repos.next.is_none() {
            break;
        } else {
            page += 1;
        }
    }

    let repositories: Vec<Repo> =
        futures_util::future::join_all(gh_repos.into_iter().map(|repo| handle_repo(repo, &client)))
            .await
            .into_iter()
            .collect::<anyhow::Result<_>>()?;

    println!("{}", serde_json::to_string_pretty(&repositories)?);

    Ok(())
}

async fn handle_repo(repo: Repository, client: &Octocrab) -> anyhow::Result<Repo> {
    // Teams
    let mut team_page = 0u32;
    let mut teams = vec![];
    loop {
        let mut team_response = client
            .repos(&repo.owner.as_ref().unwrap().login, &repo.name)
            .list_teams()
            .per_page(100)
            .page(team_page)
            .send()
            .await?;
        for team in team_response.take_items() {
            teams.push(Team {
                name: team.name,
                permission: team.permission,
            });
        }

        if team_response.next.is_none() {
            break;
        } else {
            team_page += 1;
        }
    }

    // Collaborators
    let mut collabs_page = 0u32;
    let mut collaborators = vec![];
    loop {
        let mut collab_response = client
            .repos(&repo.owner.as_ref().unwrap().login, &repo.name)
            .list_collaborators()
            .per_page(100)
            .page(collabs_page)
            .send()
            .await?;
        for collaborator in collab_response.take_items() {
            collaborators.push(Collaborator {
                name: collaborator.author.login,
                permissions: collaborator.permissions,
            });
        }

        if collab_response.next.is_none() {
            break;
        } else {
            collabs_page += 1;
        }
    }
    Ok(Repo {
        name: repo.name,
        teams,
        collaborators,
    })
}
