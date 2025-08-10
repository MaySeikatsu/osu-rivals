use rosu_v2::prelude::*;
// use tokio::io::{AsyncReadExt, AsyncWriteExt};
// use tokio::net::TcpListener;

// mod api_v2;
mod rank;
mod ratatui_test;

#[tokio::main]
async fn main() {
    // api_v2::v2().await;
    // v2().await;

    // let _ = rank::v2().await;
    ratatui_test::basic_tui();
}
