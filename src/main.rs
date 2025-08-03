use rosu_v2::prelude::*;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

// mod api_v1;
// mod api_v2;
mod rank;

#[tokio::main]
async fn main() {
    // v1();
    // api_v2::v2().await;
    rank::v2().await;
    // v2().await;
}

// #[tokio::main]
pub async fn v2() {
    // Initilize Client
    let client_id: u64 = 42909;
    let client_secret = String::from("rBP4sRzwcGL9bYiqLb5fX1UXDuwtrY7LwWO8oJSh");
    let osu = Osu::new(client_id, client_secret).await.unwrap();

    // Get peppy's top 10-15 scores in osu!standard.
    // Note that the username here can only be used because of the `cache` feature.
    // If you are fine with just providing user ids, consider disabling this feature.
    println!("You should now see your score:");
    let scores: Vec<Score> = osu
        .user_scores("mayseikatsu")
        .mode(GameMode::Osu)
        .best() // top scores; alternatively .recent(), .pinned(), or .firsts()
        .offset(00)
        .limit(1)
        .await
        .unwrap();
    println!("{:#?}", scores);

    // Search non-nsfw loved mania maps matching the given query.
    // Note that the order of called methods doesn't matter for any endpoint.
    // let search_result: BeatmapsetSearchResult = osu.beatmapset_search()
    //     .nsfw(false)
    //     .status(Some(RankStatus::Loved))
    //     .mode(GameMode::Mania)
    //     .query("blue army stars>3")
    //     .await
    //     .unwrap();
    // println!("{:#?}",search_result);

    // Get the french wiki page on the osu file format
    // let wiki_page: WikiPage = osu.wiki("fr")
    //     .page("Client/File_formats/osu_%28file_format%29")
    //     .await
    //     .unwrap();
    // println!("{:#?}",wiki_page);
}
