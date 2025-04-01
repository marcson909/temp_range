use loco_rs::cli;
use migration::Migrator;
use temp_range::app::App;

#[tokio::main]
async fn main() -> loco_rs::Result<()> {
    cli::main::<App, Migrator>().await
}
