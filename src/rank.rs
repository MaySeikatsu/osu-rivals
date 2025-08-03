use rosu_v2::prelude::*;

// [tokio::main]
pub async fn v2() -> Result<(), Box<dyn std::error::Error>> {
    // Initilize Client
    let client_id: u64 = 42909;
    let client_secret = String::from("rBP4sRzwcGL9bYiqLb5fX1UXDuwtrY7LwWO8oJSh");
    let osu = Osu::new(client_id, client_secret).await.unwrap();

    let user_ext = osu.user("mayseikatsu").mode(GameMode::Osu).await?;
    let user_mania = osu.user("mayseikatsu").mode(GameMode::Mania).await?;
    println!("{:#?}", user_ext);
    // if let Some(stats) = &user_ext.statistics {
    //     if let Some(global_rank) = stats.global_rank {
    //         println!("Global rank: {}", global_rank);
    //     } else {
    //         println!("No global rank found for this user");
    //     }
    // } else {
    //     println!("No statistics found for this user");
    // }

    // let statistics = match user_ext.statistics {
    //     Some(x) => x,
    //     None => {
    //         println!("Error!");
    //         return Ok(());
    //     }
    // };
    // let rank = match statistics.global_rank {
    //     Some(rank) => rank,
    //     None => println!("Error!"),
    // };
    // println!("You should see your Rank:");
    let global_rank = match user_ext.statistics.clone().unwrap().global_rank {
        Some(x) => x,
        None => {
            println!("Error!");
            return Ok(());
        }
    };
    let country_rank = match user_ext.statistics.clone().unwrap().country_rank {
        Some(x) => x,
        None => {
            println!("Error!");
            return Ok(());
        }
    };
    // let pp = match user_ext.statistics.clone().unwrap().pp {
    //     Some(x) => x,
    //     None => {
    //         println!("Error!");
    //         return Ok(());
    //     }
    // };
    let max_combo = user_ext.statistics.clone().unwrap().max_combo; // let level = match user_ext.statistics.unwrap() {
    //     Some(x) => x,
    //     None => {
    //         println!("Error!");
    //         return Ok(());
    //     }
    // };

    println!("Username: {:?}", user_ext.username);
    println!("Avatar URL: {:?}", user_ext.avatar_url);
    println!("Is Online: {:?}", user_ext.is_online);
    println!("Playstyle: {:?}", user_ext.playstyle.unwrap());
    println!("Country: {:?}", user_ext.country);
    println!("Hightest Rank: {:?}", user_ext.highest_rank.unwrap().rank);
    println!("Your global rank is: {:#?}", global_rank);
    println!("Your country rank is: {:#?}", country_rank);
    println!("Join Date: {:#?}", user_ext.join_date);
    println!(
        "Accuracy: {:#?}",
        user_ext.statistics.clone().unwrap().accuracy
    );
    println!("PP: {:#?}", user_ext.statistics.clone().unwrap().pp);
    println!(
        "Max Combo: {:#?}",
        user_ext.statistics.clone().unwrap().max_combo
    );
    println!("Your global rank is: {:?}", max_combo);
    // println!("Your country rank is: {:#?}", pp);

    // println!("You should now see your score:");
    // let scores: Vec<Score> = osu
    //     .user_scores("mayseikatsu")
    //     .mode(GameMode::Osu)
    //     .best() // top scores; alternatively .recent(), .pinned(), or .firsts()
    //     .offset(00)
    //     .limit(1)
    //     .await
    //     .unwrap();
    // println!("{:#?}", scores);

    Ok(())
}
