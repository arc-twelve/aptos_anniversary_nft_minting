mod app;
mod constants;
mod mint;
mod utils;
use eyre::Result;
use tracing::error;

#[tokio::main]
async fn main() -> Result<()> {
    utils::set_logger();
    let res = mint::mint().await;

    match res {
        Ok(_) => (),
        Err(err) => {
            error!("{}", err);
        }
    }

    Ok(())
}
