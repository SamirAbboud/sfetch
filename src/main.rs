use crate::fetch::UptimeLength;

mod cache;
mod colors;
mod utils;
mod fetch;

fn main() {
    println!("Distro: {}", fetch::distro(true));
    println!("Distro Id: {}", fetch::distro_id());
    println!("Model: {}", fetch::model(true));
    println!("Shell: {}", fetch::shell(true));
    println!("Kernel: {}", fetch::kernel(true));
    println!("Terminal: {}", fetch::terminal());
    println!("Hostname: {}", fetch::hostname());
    println!("Uptime: {}", fetch::uptime(false, UptimeLength::Full));
    println!("Packages: {}", fetch::packages());
    println!("DE: {}", fetch::de());
    println!("WM: {}", fetch::wm(true));
    println!("GTK Theme: {}", fetch::gtk_theme());
    println!("Icon theme: {}", fetch::icon_theme());
    println!("Cursor Theme: {}", fetch::cursor_theme());
    println!("GTK Font: {}", fetch::gtk_font());
    println!("CPU: {}", fetch::cpu(2, false, false));
    println!("Memory: {}", fetch::memory(true, 2, true));
    println!("Colors: {}", fetch::color_palette(true, "   ", true));
    println!("Monitor: {}", fetch::monitor());
    println!("Disk: {}", fetch::disk("/", true, true, true, 2));
    println!("GPU: {}", fetch::gpu(true, false));
    println!("Driver: {}", fetch::gpu_driver(false));
}
