use crate::app::App;
use crate::utils;
use aptos_sdk::types::transaction::ExecutionStatus;
use eyre::Result;
use tracing::{error, info, instrument, warn};

#[instrument]
pub async fn mint() -> Result<()> {
    let wallets = utils::get_account_list().await?;

    for private_key in wallets {
        let nft_number = utils::get_random_number();

        let app = App::new(&private_key).await;

        let app = match app {
            Ok(app) => app,
            Err(err) => {
                error!("Error: {}", err);
                continue;
            }
        };

        app.print_balance().await;

        let signed_transaction = app.conctruct_tx(nft_number).await;

        match app
            .client
            .simulate_bcs_with_gas_estimation(&signed_transaction, true, true)
            .await
        {
            Ok(sim_result) => {
                if let ExecutionStatus::MoveAbort { info, .. } = sim_result.inner().info.status() {
                    match info {
                        Some(ref info)
                            if info.reason_name == "EINSUFFICIENT_MAX_PER_USER_BALANCE" =>
                        {
                            warn!("{} has already minted the NFT", app.account.address());
                            continue;
                        }
                        _ => {}
                    }
                }
            }
            Err(e) => {
                error!("Transaction simulation failed: {e}");
                continue;
            }
        }

        let tx = app.client.submit(&signed_transaction).await;

        match tx {
            Ok(pending_tx) => {
                let res = app.client.wait_for_transaction(pending_tx.inner()).await;

                if let Ok(transaction) = res {
                    if transaction.inner().success() {
                        let hash = pending_tx.inner().hash;
                        info!("Success! Tx hash: {}", hash);
                        utils::sleep().await;
                    }
                }
            }
            Err(er) => {
                error!("Error transaction: {er}");
                utils::sleep().await;
                continue;
            }
        }
    }

    info!("Done");

    Ok(())
}
