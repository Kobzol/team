use std::path::PathBuf;

use backfill::Repo;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let input = "repos.json";

    let repos: Vec<Repo> = serde_json::from_slice(&std::fs::read(input)?)?;
    let mut existing = 0;
    for repo in &repos {
        let path = PathBuf::from("../teams").join(format!("{}.toml", repo.name));
        if path.exists() {
            existing += 1;
            continue;
        }
        std::fs::write(
            path,
            format!(
                r#"name = "{}"
"#,
                repo.name
            ),
        )?;
    }
    println!("Existing: {existing}");
    println!("Created: {}", repos.len() - existing);

    Ok(())
}
