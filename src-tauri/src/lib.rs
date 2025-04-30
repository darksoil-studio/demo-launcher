use holochain_types::web_app::WebAppBundle;
use std::path::PathBuf;
use tauri::{AppHandle, Manager, WebviewWindowBuilder};
use tauri_plugin_dialog::{DialogExt, MessageDialogButtons};
use tauri_plugin_holochain::{
    vec_to_locked, HolochainExt, HolochainPluginConfig, NetworkConfig,
};
#[cfg(not(mobile))]
use tauri::menu::{Menu, MenuItem, PredefinedMenuItem, Submenu};
#[cfg(not(mobile))]
use tauri_plugin_updater::UpdaterExt;

const APP_ID: &'static str = "happ-store";

pub fn webhapp_bundle() -> WebAppBundle {
    let bytes = include_bytes!("../../happ-store.webhapp");
    WebAppBundle::decode(bytes).expect("Failed to decode happ-store webhapp")
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(
            tauri_plugin_log::Builder::default()
                .level(log::LevelFilter::Warn)
                .build(),
        )
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_holochain::init(
            vec_to_locked(vec![]),
            HolochainPluginConfig::new(holochain_dir(), network_config()),
        ))
        .setup(|app| {
            #[cfg(mobile)]
            app.handle().plugin(tauri_plugin_barcode_scanner::init())?;
            #[cfg(not(mobile))]
            app.handle().plugin(tauri_plugin_updater::Builder::new().build())?;

            let handle = app.handle().clone();
            let result: anyhow::Result<()> = tauri::async_runtime::block_on(async move {
                #[cfg(not(mobile))]
                {
                    let updater = app.handle().updater()?;

                    if let Ok(Some(_update)) = updater.check().await {
                        WebviewWindowBuilder::new(app.handle(), "updater", tauri::WebviewUrl::App("".into())).title("Update Found").inner_size(400.0, 300.0).build()?;

                        return Ok(());
                    }
                }

                setup(handle.clone()).await?;

                let mut window_builder = app
                    .holochain()?
                    .web_happ_window_builder(String::from(APP_ID), None)
                    .await?;

                #[cfg(not(mobile))]
                {
                    window_builder = window_builder
                        .title(String::from("Demo Launcher"))
                        .inner_size(1200.0, 800.0)
                        .menu( 
                            Menu::with_items(
                                &handle,
                                &[&Submenu::with_items(
                                    &handle,
                                    "File",
                                    true,
                                    &[
                                        &MenuItem::with_id(
                                           & handle,
                                            "open-logs-folder",
                                            "Open Logs Folder",
                                            true,
                                            None::<&str>,
                                        )?,
                                        &MenuItem::with_id(
                                           & handle,
                                            "factory-reset",
                                            "Factory Reset",
                                            true,
                                            None::<&str>,
                                        )?,
                                        &PredefinedMenuItem::close_window(&handle, None)?,
                                    ],
                                )?],
                            )?
                        )
                        .on_menu_event(|window, menu_event| match menu_event.id().as_ref() {
                            "open-logs-folder" => {
                                let log_folder = window.app_handle()
                                    .path()
                                    .app_log_dir()
                                    .expect("Could not get app log dir");
                                if let Err(err) = tauri_plugin_opener::reveal_item_in_dir(log_folder.clone()) {
                                    log::error!("Failed to open log dir at {log_folder:?}: {err:?}");
                                }
                            }
                            "factory-reset" => {
                                let h = window.app_handle().clone();
                                window.app_handle()
                                        .dialog()
                                        .message("Are you sure you want to perform a factory reset? All your data will be lost.")
                                        .title("Factory Reset")
                                        .buttons(MessageDialogButtons::OkCancel)
                                        .show(move |result| match result {
                                            true => {
                                                if let Err(err) = std::fs::remove_dir_all(holochain_dir()) {
                                                    log::error!("Failed to perform factory reset: {err:?}");
                                                } else {
                                                    h.restart();
                                                }
                                            }
                                            false => {
        
                                            }
                                        });
                            }
                            _ => {}
                        });
                }

                window_builder.build()?;

                Ok(())
            });

            result?;

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

async fn setup(handle: AppHandle) -> anyhow::Result<()> {
    let admin_ws = handle.holochain()?.admin_websocket().await?;

    let installed_apps = admin_ws
        .list_apps(None)
        .await
        .map_err(|err| tauri_plugin_holochain::Error::ConductorApiError(err))?;

    if installed_apps
        .iter()
        .find(|app| app.installed_app_id.as_str().eq(APP_ID))
        .is_none()
    {
        handle
            .holochain()?
            .install_web_app(String::from(APP_ID), webhapp_bundle(), None, None, None)
            .await?;

        Ok(())
    } else {
        handle
            .holochain()?
            .update_web_app_if_necessary(String::from(APP_ID), webhapp_bundle())
            .await?;

        Ok(())
    }
}

fn network_config() -> NetworkConfig {
    let mut network_config = NetworkConfig::default();

    // Don't use the bootstrap service on tauri dev mode
    if tauri::is_dev() {
        network_config.bootstrap_url = url2::Url2::parse("http://localhost:34399");
        network_config.signal_url = url2::Url2::parse("ws://localhost:34399");
    } else {
        network_config.bootstrap_url = url2::Url2::parse("http://157.180.93.55:8888");
        network_config.signal_url = url2::Url2::parse("ws://157.180.93.55:8888");
    }

    // Don't hold any slice of the DHT in mobile
    if cfg!(mobile) {
        network_config.target_arc_factor = 0;
    }

    network_config
}

fn holochain_dir() -> PathBuf {
    if tauri::is_dev() {
        let tmp_dir = tempdir::TempDir::new("demo-launcher")
            .expect("Could not create temporary directory");

        // Convert `tmp_dir` into a `Path`, destroying the `TempDir`
        // without deleting the directory.
        let tmp_path = tmp_dir.into_path();
        tmp_path
    } else {
        app_dirs2::app_root(
            app_dirs2::AppDataType::UserData,
            &app_dirs2::AppInfo {
                name: "demo-launcher",
                author: std::env!("CARGO_PKG_AUTHORS"),
            },
        )
        .expect("Could not get app root")
        .join(std::env!("CARGO_PKG_VERSION"))
        .join("holochain")
    }
}

