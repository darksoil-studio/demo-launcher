use tauri::AppHandle;
use tauri_plugin_holochain::HolochainExt;

use crate::APP_ID;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    HolochainPluginError(#[from] tauri_plugin_holochain::Error),
    #[error(transparent)]
    TauriError(#[from] tauri::Error),
}

// we must manually implement serde::Serialize
impl serde::Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

#[tauri::command]
pub async fn open_happ_store(handle: AppHandle) -> Result<(), Error> {
    handle
        .holochain()?
        .web_happ_window_builder(APP_ID.into(), None)
        .await?
        .build()?;

    Ok(())
}
