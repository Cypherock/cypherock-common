use hex;
use crate::proto;
use crate::utils::*;

// Enum to easily switch firmware variants
enum FirmwareVariant {
    BtcOnly,
    MultiCoin,
}

impl FirmwareVariant {
    fn id(&self) -> u32 {
        match self {
            FirmwareVariant::BtcOnly => 1,
            FirmwareVariant::MultiCoin => 2,
        }
    }

    fn name(&self) -> String {
        match self {
            FirmwareVariant::BtcOnly => "btc-only".to_string(),
            FirmwareVariant::MultiCoin => "multi-coin".to_string(),
        }
    }
}

pub fn create_query() -> proto::core::Query {
    let mut query = proto::core::Query::default();
    let mut get_device_info = proto::get_device_info::Request::default();
    get_device_info.dummy = true;
    query.request = Some(proto::core::query::Request::GetDeviceInfo(get_device_info));
    query
}

pub fn parse_query(query: &proto::core::Query) {
    println!("Parsing query...");
    match &query.request {
        None => println!("None cmd"),
        Some(proto::core::query::Request::GetDeviceInfo(cmd)) => {
            println!("Is GetDeviceInfoCmd");
            println!("Dummy: {}", cmd.dummy);
        },
        _ => println!("Unsupported query"),
    }
}

pub fn create_result() -> proto::core::Result {
    let mut result = proto::core::Result::default();
    let mut get_device_info = proto::get_device_info::Response::default();

    // Choose firmware variant
    let firmware_variant = FirmwareVariant::BtcOnly;

    // Supported Coin #1 - BTC
    let mut coin_item_btc = proto::get_device_info::SupportedCoinItem::default();
    let mut version_btc = proto::common::Version::default();
    version_btc.major = 1;
    version_btc.minor = 0;
    version_btc.patch = 0;
    version_btc.variant_id = firmware_variant.id();
    version_btc.variant_str = firmware_variant.name();
    coin_item_btc.id = hex::decode("10").unwrap(); // BTC coin ID
    coin_item_btc.version = Some(version_btc);

    // Supported Coin #2 - ETH
    let mut coin_item_eth = proto::get_device_info::SupportedCoinItem::default();
    let mut version_eth = proto::common::Version::default();
    version_eth.major = 1;
    version_eth.minor = 1;
    version_eth.patch = 16;
    version_eth.variant_id = firmware_variant.id();
    version_eth.variant_str = firmware_variant.name();
    coin_item_eth.id = hex::decode("821034").unwrap(); // ETH coin ID
    coin_item_eth.version = Some(version_eth);

    // Firmware Version Info
    let mut firmware_version = proto::common::Version::default();
    firmware_version.major = 1;
    firmware_version.minor = 2;
    firmware_version.patch = 0;
    firmware_version.variant_id = firmware_variant.id();
    firmware_version.variant_str = firmware_variant.name();

    // Device Info
    get_device_info.device_serial = hex::decode("67458743").unwrap();
    get_device_info.firmware_version = Some(firmware_version);
    get_device_info.is_authenticated = true;

    // Coin support depends on variant
    get_device_info.coin_list = match firmware_variant {
        FirmwareVariant::BtcOnly => vec![coin_item_btc],
        FirmwareVariant::MultiCoin => vec![coin_item_btc, coin_item_eth],
    };

    result.response = Some(proto::core::result::Response::GetDeviceInfo(get_device_info));
    result
}

pub fn parse_result(result: proto::core::Result) {
    println!("Parsing result...");
    match result.response {
        None => println!("None cmd"),
        Some(proto::core::result::Response::GetDeviceInfo(cmd)) => {
            println!("Is GetDeviceInfoResult");
            println!("Device Serial: {:?}", hex::encode(&cmd.device_serial));

            if let Some(firmware_version) = &cmd.firmware_version {
                println!(
                    "Firmware Version: {}.{}.{}",
                    firmware_version.major, firmware_version.minor, firmware_version.patch
                );
                println!("Firmware Variant ID: {}", firmware_version.variant_id);
                println!("Firmware Variant: {}", firmware_version.variant_str);
            }

            println!("Is Authenticated: {}", cmd.is_authenticated);
            println!("Supported Coins: {}", cmd.coin_list.len());

            for coin in &cmd.coin_list {
                let version = coin.version.as_ref().unwrap();
                println!("\tCoin ID: {:?}", hex::encode(&coin.id));
                println!(
                    "\tVersion: {}.{}.{}",
                    version.major, version.minor, version.patch
                );
                println!("\tVariant ID: {}", version.variant_id);
                println!("\tVariant String: {}", version.variant_str);
            }
        },
        _ => println!("Unsupported result"),
    }
}

pub fn run() {
    println!();
    println!("********* Device Info: Started ************");

    // Query
    let query = create_query();
    let serialized_query = serialize(&query);
    let deserialized_query = deserialize_query(&serialized_query).expect("Query deserialization failed");
    println!("Serialized Query: {:?}", serialized_query);
    parse_query(&deserialized_query);

    println!();

    // Result
    let result = create_result();
    let serialized_result = serialize(&result);
    let deserialized_result = deserialize_result(&serialized_result).expect("Result deserialization failed");
    println!("Serialized Result: {:?}", serialized_result);
    parse_result(deserialized_result);

    println!("********* Device Info: Completed ************");
}
