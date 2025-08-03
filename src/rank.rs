use rosu_v2::{model::user, prelude::*};
use sparklines::spark;

// [tokio::main]
pub async fn v2() -> Result<(), Box<dyn std::error::Error>> {
    // Initilize Client
    let client_id: u64 = 42909;
    let client_secret = String::from("rBP4sRzwcGL9bYiqLb5fX1UXDuwtrY7LwWO8oJSh");
    let osu = Osu::new(client_id, client_secret).await.unwrap();

    let input_username = "mayseikatsu";

    let user_ext = osu.user(input_username).mode(GameMode::Osu).await?;
    let user_mania = osu.user(input_username).mode(GameMode::Mania).await?;
    // println!("{:#?}", user_ext); // Get all values from osu.user, to see possible options
    // println!("{:#?}", user_mania); // Get all values from osu.user, to see possible options

    // println!("You should see your Rank:");
    // UNWRAP is bad habbit and should be replaced with match or catch checks for fail safety,
    // therefore we extract statistics and each OPTION to prevent the program from crashing. This
    // should be done for each OPTION inside of the API Request, in our case we can make out
    // options by looking at the name + some (value1, value2 ) and we see its an OPTION (). It
    // kinda behaved like a class with multiple possible value/var types
    // Like this we can also geet rid of the clone() and unwrap() in our call, as visivle in
    // previous commit
    let statistics = match user_ext.statistics {
        Some(x) => x,
        None => {
            println!("Error!");
            return Ok(());
        }
    };
    let highest_rank = match user_ext.highest_rank {
        Some(x) => x,
        None => {
            println!("Error!");
            return Ok(());
        }
    };
    let playstyle = match user_ext.playstyle {
        Some(x) => x,
        None => {
            println!("Error!");
            return Ok(());
        }
    };
    let rank_history = match user_ext.rank_history {
        Some(x) => x,
        None => {
            println!("Error!");
            return Ok(());
        }
    };

    // Sub OPTION from STATISTICS OPTION
    let global_rank = match statistics.global_rank {
        Some(x) => x,
        None => {
            println!("Error!");
            return Ok(());
        }
    };
    let country_rank = match statistics.country_rank {
        Some(x) => x,
        None => {
            println!("Error!");
            return Ok(());
        }
    };
    let max_combo = statistics.max_combo; // let level = match user_ext.statistics.unwrap() {

    println!("Username: {:?}", user_ext.username);
    println!("Avatar URL: {:?}", user_ext.avatar_url);
    println!("Is Online: {:?}", user_ext.is_online);
    println!("Playstyle: {:?}", playstyle[0]); // playstyle[value] to get specific output value
    println!("Country: {:?}", user_ext.country);
    println!("Hightest Rank: {:?}", highest_rank.rank);
    println!("Your global rank is: {:#?}", global_rank);
    println!("Your country rank is: {:#?}", country_rank);
    println!("Join Date: {:#?}", user_ext.join_date.date());
    println!("Accuracy: {:#?}", statistics.accuracy);
    println!("PP: {:#?}", statistics.pp);
    println!("Max Combo: {:#?}", statistics.max_combo);
    let rank_history_f64: Vec<f64> = rank_history.iter().map(|&x| x as f64).collect();
    // println!("Rank History: {:#?}", spark(&rank_history_f64)); // still has to be inverted to not raise but fall
    // println!("Max Combo: {:?}", max_combo);

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
