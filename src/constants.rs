use aptos_sdk::types::account_address::AccountAddress;
use once_cell::sync::Lazy;
use std::str::FromStr;

pub static RPC_LINK: &str = "https://fullnode.mainnet.aptoslabs.com/v1";
pub static TX_TIMEOUT: u64 = 10;

pub static COLLECTION_ID: Lazy<AccountAddress> = Lazy::new(|| {
    AccountAddress::from_str("0xd42cd397c41a62eaf03e83ad0324ff6822178a3e40aa596c4b9930561d4753e5")
        .unwrap()
});

pub static CONTRACT_ADDRESS: Lazy<AccountAddress> = Lazy::new(|| {
    AccountAddress::from_str("0x96c192a4e3c529f0f6b3567f1281676012ce65ba4bb0a9b20b46dec4e371cccd")
        .unwrap()
});

pub static TIME_SLEEP: [u64; 2] = [10, 60];
