use rosu_v2::prelude::*;

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
    let scores: Vec<Score> = osu.user_scores("8557950")
        .mode(GameMode::Osu)
        .best() // top scores; alternatively .recent(), .pinned(), or .firsts()
        .offset(10)
        .limit(5)
        .await
        .unwrap();
    println!("{:#?}",scores);


    // // Search non-nsfw loved mania maps matching the given query.
    // // Note that the order of called methods doesn't matter for any endpoint.
    // let search_result: BeatmapsetSearchResult = osu.beatmapset_search()
    //     .nsfw(false)
    //     .status(Some(RankStatus::Loved))
    //     .mode(GameMode::Mania)
    //     .query("blue army stars>3")
    //     .await
    //     .unwrap();
    //
    // // Get the french wiki page on the osu file format
    // let wiki_page: WikiPage = osu.wiki("fr")
    //     .page("Client/File_formats/osu_%28file_format%29")
    //     .await
    //     .unwrap();
}
