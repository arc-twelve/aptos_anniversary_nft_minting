use crate::constants;
use aptos_sdk::{
    bcs,
    coin_client::CoinClient,
    crypto::ValidCryptoMaterialStringExt,
    move_types::{ident_str, language_storage::ModuleId},
    rest_client::Client as ApiClient,
    transaction_builder::TransactionBuilder,
    types::{
        chain_id::ChainId,
        transaction::{EntryFunction, SignedTransaction, TransactionPayload},
        AccountKey, LocalAccount,
    },
};
use constants::{COLLECTION_ID, CONTRACT_ADDRESS, RPC_LINK, TX_TIMEOUT};
use eyre::{Context, Result};
use std::time::{SystemTime, UNIX_EPOCH};
use tracing::{info, instrument};

#[derive(Debug)]
pub struct App {
    pub client: ApiClient,
    pub account: LocalAccount,
}

impl App {
    #[instrument]
    pub async fn new(private_key: &str) -> Result<Self> {
        let client = ApiClient::new(RPC_LINK.parse()?);

        let private_key_ed =
            aptos_sdk::crypto::ed25519::Ed25519PrivateKey::from_encoded_string(private_key)
                .context("Failed to encode privete key")?;

        let accont_key = AccountKey::from_private_key(private_key_ed);
        let account_address = accont_key.authentication_key().account_address();

        let account = client
            .get_account(account_address)
            .await
            .context("Failed to retrieve account information")?;
        let seq_number = account.inner().sequence_number;

        let account = LocalAccount::from_private_key(private_key, seq_number).map_err(|_| {
            eyre::eyre!("Failed to create LocalAccount from private key and sequence number")
        })?;

        Ok(Self { client, account })
    }

    #[instrument(skip_all)]
    pub async fn print_balance(&self) {
        let coin_client = CoinClient::new(&self.client);

        let balance = coin_client
            .get_account_balance(&self.account.address())
            .await
            .unwrap();
        let balance = balance as f64 / 100_000_000.0;

        let short_address = &self.account.address().to_string()[0..10];

        info!("Account: {}... Balance: {} APT", short_address, balance);
    }

    #[instrument]
    pub async fn conctruct_tx(&self, nft_amount: u64) -> SignedTransaction {
        let args = vec![
            bcs::to_bytes(&*COLLECTION_ID).unwrap(),
            bcs::to_bytes(&Some(nft_amount)).unwrap(),
        ];

        let payload = TransactionPayload::EntryFunction(EntryFunction::new(
            ModuleId::new(
                *CONTRACT_ADDRESS,
                ident_str!("unmanaged_launchpad").to_owned(),
            ),
            ident_str!("mint").to_owned(),
            vec![],
            args,
        ));

        let timeout = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
            + TX_TIMEOUT;

        let raw_transaction = TransactionBuilder::new(payload, timeout, ChainId::mainnet())
            .sender(self.account.address())
            .sequence_number(self.account.sequence_number())
            .max_gas_amount(5000)
            .gas_unit_price(100)
            .build();

        self.account.sign_transaction(raw_transaction)
    }
}
