use marine_rs_sdk::marine;
use marine_rs_sdk::module_manifest;
use marine_rs_sdk::MountedBinaryResult;
use marine_rs_sdk::WasmLoggerBuilder;

use eyre::Result;

module_manifest!();

#[marine]
pub fn set(key: String, value: String) -> String {
    let args = vec![String::from("SET"), key.clone(), value];

    log::info!("redis args {:?}", args);

    let value = unwrap_mounted_binary_result(redis(args)).expect("It always return string");

    match value.trim() {
        "OK" => return key,
        _ => return value.to_string(),
    }
}

#[marine]
pub fn get(key: String) -> String {
    let args = vec![String::from("GET"), key];

    log::info!("redis args {:?}", args);

    let value = unwrap_mounted_binary_result(redis(args)).expect(" must be string");

    match value.trim() {
        "(nil)" => return "".to_string(),
        _ => return value.trim().to_string(),
    }
}

pub fn main() {
    WasmLoggerBuilder::new()
        .with_log_level(log::LevelFilter::Info)
        .build()
        .unwrap();
}

fn unwrap_mounted_binary_result(result: MountedBinaryResult) -> Result<String> {
    result
        .into_std()
        .ok_or(eyre::eyre!(
            "stdout or stderr contains non valid UTF8 string"
        ))?
        .map_err(|e| eyre::eyre!("ipfs cli call failed: {}", e))
}

#[marine]
#[link(wasm_import_module = "host")]
extern "C" {
    /// Execute command, return result
    pub fn redis(cmd: Vec<String>) -> MountedBinaryResult;
}
