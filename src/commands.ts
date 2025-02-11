import { InstalledAppId } from "@holochain/client";
import { invoke } from "@tauri-apps/api/core";

export async function openHappStore() {
  return invoke("open_happ_store");
}

export async function openHapp(appId: InstalledAppId) {
  return invoke("open_happ", {
    appId,
  });
}
