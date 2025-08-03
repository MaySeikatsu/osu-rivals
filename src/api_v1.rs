use rosu::{model::*, Osu, OsuResult,};
use time::OffsetDateTime;

#[tokio::main]
pub async fn v1() -> OsuResult<()> {
    // Initialize the client
    let osu = Osu::new("8d3e8ce43189c0b05be4bd0f048807d703a0740b");

    // --- Retrieving top scores ---
    let mut scores = osu.top_scores("Badewanne3")
        .mode(GameMode::Mania)
        .limit(4)
        .await?;
    match scores.pop() {
        Some(score) => {
            // Retrieve user of the score
            let user = score.get_user(&osu).mode(GameMode::Osu).await?;
            // ...
        }
        None => println!("No top scores found"),
    }

    // --- Retrieving beatmaps ---
    let mut maps = osu.beatmaps()
        .mode(GameMode::Mania)
        .limit(3)
        .since(OffsetDateTime::from_unix_timestamp(1542150088).unwrap())
        .mapset_id(945496)
        .await?;
    if let Some(map) = maps.pop() {
        let leaderboard: Vec<Score> = map
            .get_global_leaderboard(&osu)
            .limit(13)
            .await?;
        // ...
    }
    // --- Retrieving user ---
    let user: Option<User> = osu.user("Badewanne3").await?;
    // ...

    // --- Retrieving match ---
    let osu_match: Match = osu.osu_match(58494587).await?;
    // ...

    Ok(())
}
