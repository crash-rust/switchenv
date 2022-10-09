use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::path::Path;
use tauri::api::path as tauri_path;

#[derive(Serialize, Deserialize, Debug)]
struct EnowConfig {
    environment: String,
}

/**
 * change_enow_env命令用于修改enow启动环境，它接收一个环境参数，代表当前需要修改成什么环境的服务
 */

#[tauri::command]
pub fn change_enow_env(env: &str) -> String {
    let local_data_dir = tauri_path::local_data_dir().unwrap();
    let easinote_path = Path::join(&local_data_dir, "easinote");
    let config_path = Path::join(&easinote_path, "config");
    let env_config_path = Path::join(&config_path, "easinote_config.json");

    if easinote_path.exists() {
        if config_path.exists() {
            if env_config_path.exists() {
                let content = fs::read_to_string(&env_config_path).unwrap();
                let mut value: EnowConfig = serde_json::from_str(&content).unwrap();
                if env.contains("Test") {
                    (&mut value).environment = String::from("test");
                } else if env.contains("Prod") {
                    (&mut value).environment = String::from("production");
                } else {
                    (&mut value).environment = String::from("test");
                }

                let value_json = serde_json::to_string(&value).unwrap();
                fs::write(&env_config_path, value_json).unwrap();
            } else {
                File::create(&env_config_path).unwrap();
            }
        } else {
            File::create(&config_path).unwrap();
        }
    }

    env.to_string()
}
