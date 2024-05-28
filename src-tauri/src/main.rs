// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::{Deserialize, Serialize};
use sled::{Db};
use std::sync::Mutex;
use tauri::api::path::app_data_dir;

#[derive(Serialize, Deserialize, Debug)]
struct MyData {
    id: u32,
    name: String,
    age: u32,
}

struct AppState {
    db: Mutex<Db>,
}

#[tauri::command]
fn insert_data(state: tauri::State<AppState>, data: MyData) -> Result<(), String> {
    let db = state.db.lock().map_err(|_| "DB lock error")?;
    let key = data.id.to_string();
    let value = serde_json::to_vec(&data).map_err(|_| "Serialization error")?;
    db.insert(key, value).map_err(|_| "DB insert error")?;
    Ok(())
}

#[tauri::command]
fn get_data(state: tauri::State<AppState>, id: u32) -> Result<String, String> {
    let db = state.db.lock().map_err(|_| "DB lock error")?;
    let key = id.to_string();
    if let Some(value) = db.get(key).map_err(|_| "DB get error")? {
        let data: MyData = serde_json::from_slice(&value).map_err(|_| "Deserialization error")?;
        Ok(serde_json::to_string(&data).map_err(|_| "Serialization error")?)
    } else {
        Err("Data not found".into())
    }
}

fn main() {
    let app_data_dir = app_data_dir(&tauri::Config::default()).expect("AppData directory not found").join("capella_data");
    std::fs::create_dir_all(&app_data_dir).expect("Failed to create capella_data directory");

    let db_path = app_data_dir.join("capella_db");
    let db = sled::open(db_path).expect("DB open error");

    tauri::Builder::default()
        .manage(AppState {
            db: Mutex::new(db),
        })
        .invoke_handler(tauri::generate_handler![insert_data, get_data])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
