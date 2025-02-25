fn main() {
    println!("cargo:rerun-if-changed=../happ-store.webhapp");
    tauri_build::build()
}
