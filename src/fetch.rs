use crate::{cache::{get_cache, set_cache}, colors, utils::{capitalize, command_exists, make_me_pretty, run_command}};

#[allow(dead_code)]
pub enum UptimeLength {
    Full,
    Medium,
    Short,
}

pub fn distro(architecture: bool) -> String {
    let mut name = String::from("Unknown");

    if let Ok(content) = std::fs::read_to_string("/etc/os-release") {
        for line in content.lines() {
            if let Some(value) = line.strip_prefix("PRETTY_NAME=") {
                name = value.trim_matches('"').to_string();
                break;
            }
        }        
    }

    if name == "Unknown" && std::env::var("TERMUX_VERSION").is_ok() {
        name = String::from("Termux");
    }

    if architecture {
        name.push(' ');
        name.push_str(&std::env::consts::ARCH);
    }

    name
} 

pub fn distro_id() -> String {
    let mut id = String::new();

    let os_file = if std::path::Path::new("/bedrock/etc/os-release").is_file() {
        "/bedrock/etc/os-release"
    } else if std::path::Path::new("/etc/os-release").is_file() {
        "/etc/os-release"
    } else {
        return String::new();
    };

    if let Ok(content) = std::fs::read_to_string(os_file) {
        for line in content.lines() {
            if let Some(value) = line.strip_prefix("ID=") {
                id = value.to_string();
                break;
            }
        }
    }

    id
}

pub fn model(version: bool) -> String {
    let device_dir = "/sys/devices/virtual/dmi/id/";
    let vendor_file = format!("{device_dir}sys_vendor");
    let device_name_file = format!("{device_dir}product_name");
    let device_version_file = format!("{device_dir}product_version");

    let mut product_info = String::new();
    if std::path::Path::new(&vendor_file).is_file() {
        if let Ok(content) = std::fs::read_to_string(&vendor_file) {
            product_info = content.trim().to_string();
        }
    }

    if std::path::Path::new(&device_name_file).is_file() {
        if let Ok(content) = std::fs::read_to_string(&device_name_file) {
            if !product_info.is_empty() {
                product_info.push(' ');
            }
            product_info.push_str(content.trim());

            if version && std::path::Path::new(&device_version_file).is_file() {
                if let Ok(content) = std::fs::read_to_string(&device_version_file) {
                    product_info.push_str(" (");
                    product_info.push_str(content.trim());
                    product_info.push(')');
                }
            }
        }
    }

    product_info
}

pub fn shell(version: bool) -> String {
    let shell_path = std::env::var("SHELL").unwrap_or_default();
    let mut shell = shell_path
        .rsplit('/')
        .next()
        .unwrap_or("")
        .to_string();

    if version {
        let shell_version = match shell.as_str() {
            "fish" => run_command("fish", &["--version"])
                .replace("fish, version ", "")
                .trim()
                .to_string(),
            
            "zsh" => run_command("zsh", &["--version"])
                .split_whitespace()
                .nth(1)
                .unwrap_or("")
                .to_string(),
            
            "bash" => std::env::var("BASH_VERSION")
                .unwrap_or_default()
                .split('(')
                .next()
                .unwrap_or("")
                .trim()
                .to_string(),

            _ => String::new(),
        };

        if !shell_version.is_empty() {
            shell.push(' ');
            shell.push_str(&shell_version);
        }
    }

    capitalize(&shell)
}

pub fn kernel(small: bool) -> String {
    let mut kernel_info = run_command("uname", &["-r"]);

    if !small {
        kernel_info.insert_str(0, "Linux ");
    }

    kernel_info
}

pub fn terminal() -> String {
    let term = std::env::var("TERM")
        .unwrap_or_default()
        .replace("xterm-", "")
        .to_string();

    capitalize(&term)
}

pub fn hostname() -> String {
    let user = std::env::var("USER").unwrap_or_else(|_| "unknown".to_string());
    let hostname = run_command("hostname", &[]);

    format!("{user}@{hostname}")
}

pub fn uptime(up: bool, length: UptimeLength) -> String {
    let uptime_seconds = std::fs::read_to_string("/proc/uptime")
        .unwrap()
        .split_whitespace()
        .next()
        .unwrap()
        .parse::<f64>()
        .unwrap();

    let total_minutes = (uptime_seconds / 60.0) as u64;

    let days = total_minutes / 1_440;
    let hours = (total_minutes % 1_440) / 60;
    let minutes = total_minutes % 60;

    let uptime_info = match length {
        UptimeLength::Full => {
            let mut parts = Vec::new();

            if days > 0 {
                parts.push(format!("{} days", days));
            }

            if hours > 0 {
                parts.push(format!("{} hours", hours));
            }

            if minutes > 0 {
                parts.push(format!("{} minutes", minutes));
            }

            parts.join(", ")
        }

        UptimeLength::Medium => {
            let mut parts = Vec::new();

            if days > 0 {
                parts.push(format!("{} days", days));
            }

            if hours > 0 {
                parts.push(format!("{} hrs", hours));
            }

            if minutes > 0 {
                parts.push(format!("{} mins", minutes));
            }

            parts.join(", ")
        }

        UptimeLength::Short => {
            let mut parts = Vec::new();

            if days > 0 {
                parts.push(format!("{}d", days));
            }

            if hours > 0 {
                parts.push(format!("{}h", hours));
            }

            if minutes > 0 {
                parts.push(format!("{}m", minutes));
            }

            parts.join(", ")
        }
    };

    if up {
        format!("up {}", uptime_info)
    } else {
        uptime_info
    }
}

fn nix_packages() -> Vec<(usize, String)> {
    if !command_exists("nix-store") {
        return Vec::new();
    }

    let mut results = Vec::new();

    // NixOS system profile
    let system_profile = "/run/current-system/sw";

    if std::path::Path::new(system_profile).exists() {
        let output = run_command("nix-store", &["-qR", system_profile]);
        let count = output.lines().count();

        if count > 0 {
            results.push((count, "nix-system".to_string()));
        }
    }

    // Nix user profile
    if let Ok(user) = std::env::var("USER") {
        let user_profile = format!("/etc/profiles/per-user/{user}");

        if std::path::Path::new(&user_profile).exists() {
            let output = run_command(
                "nix-store",
                &["-qR", &user_profile],
            );

            let count = output.lines().count();

            if count > 0 {
                results.push((count, "nix-user".to_string()));
            }
        }
    }

    // Nix default profile
    let default_profile = "/nix/var/nix/profiles/default";

    if std::path::Path::new(default_profile).exists() {
        let output = run_command(
            "nix-store",
            &["-qR", default_profile],
        );

        let count = output.lines().count();

        if count > 0 {
            results.push((count, "nix-default".to_string()));
        }
    }

    results
}

pub fn packages() -> String {
    let packages_queries = [
        ("kiss", &["-l"][..], "kiss"),
        ("pacman", &["-Qq"][..], "pacman"),
        ("dpkg-query", &["-f", ".\n", "-W"][..], "dpkg"),
        ("rpm", &["-qa"][..], "rpm"),
        ("xbps-query", &["-l"][..], "xbps"),
        ("apk", &["info"][..], "apk"),
        ("opkg", &["list-installed"][..], "opkg"),
        ("pacman-g2", &["-Q"][..], "pacman-g2"),
        ("lvu", &["installed"][..], "lvu"),
        ("tce-status", &["-i"][..], "tce-status"),
        ("pkg_info", &[][..], "pkg_info"),
        ("tazpkg", &["list"][..], "tazpkg"),
        ("gaze", &["installed"][..], "sorcery"),
        ("alps", &["showinstalled"][..], "alps"),
        ("butch", &["list"][..], "butch"),
        ("mine", &["-q"][..], "mine"),
        ("snap", &["list"][..], "snap"),
        ("flatpak", &["list"][..], "flatpak"),
    ];

    let mut total_pkgs = 0;
    let mut package_count = Vec::new();

    for (command, args, manager) in packages_queries {
        if command_exists(command) {
            let pkgs = run_command(command, args);
            let num_pkgs = pkgs.lines().count();

            if num_pkgs > 0 {
                package_count.push(format!("{num_pkgs} {manager}"));
                total_pkgs += num_pkgs;
            }
        }
    }
    
    // NixOS
    for (count, manager) in nix_packages() {
        package_count.push(format!("{count} {manager}"));
        total_pkgs += count;
    }

    if package_count.len() > 1 {
        format!("{total_pkgs}, ({})", package_count.join(", "))
    } else if package_count.len() == 1 {
        let manager = package_count[0]
            .split_whitespace()
            .nth(1)
            .unwrap_or("");

        format!("{total_pkgs}, {manager}")
    } else {
        String::new()
    }
}

pub fn de() -> String {
    std::env::var("DESKTOP_SESSION")
        .or_else(|_| std::env::var("XDG_SESSION_DESKTOP"))
        .unwrap_or_default()
}

pub fn wm(protocol: bool) -> String {
    let mut des = de();

    if des.is_empty() {
        des = String::from("none");
    }
    des = des.to_lowercase();

    let mut res = match des.as_str() {
        value if value.contains("gnome") => String::from("Mutter"),
        value if value.contains("plasma") => String::from("KWin"),
        value if value.contains("xfce") => String::from("Xfwm"),
        value if value.contains("lxqt") => String::from("Openbox"),
        value if value.contains("cinnamon") => String::from("Muffin"),
        value if value.contains("mate") => String::from("Marco"),
        _ => capitalize(&des),
    };

    if protocol {
        let session = std::env::var("XDG_SESSION_TYPE")
            .unwrap_or_else(|_| String::from("None"))
            .trim()
            .to_string();
        res.push_str(&format!(" ({})", capitalize(&session)));
    }

    res
}

fn gtk_fetch(param: &str) -> String {
    let home = std::env::var("HOME").unwrap_or_default();

    let paths = [
        format!("{home}/.config/gtk-3.0/settings.ini"),
        String::from("/etc/gtk-3.0/settings.ini"),
        String::from("/usr/share/gtk-3.0/settings.ini"),
    ];

    for path in paths {
        if !std::path::Path::new(&path).is_file() {
            continue;
        }

        if let Ok(content) = std::fs::read_to_string(&path) {
            for line in content.lines() {
                if let Some(value) = line.strip_prefix(&format!("{param}=")) {
                    return make_me_pretty(value);
                }
            }
        }
    }

    String::new()
}

pub fn gtk_theme() -> String {
    let mut theme = make_me_pretty(
        &run_command(
            "gsettings",
            &["get", "org.gnome.desktop.interface", "gtk-theme"],
        )
    );

    if theme.is_empty() {
        theme = gtk_fetch("gtk-theme-name");
    }

    theme
}


pub fn icon_theme() -> String {
    let mut theme = make_me_pretty(
        &run_command(
            "gsettings",
            &["get", "org.gnome.desktop.interface", "icon-theme"],
        )
    );

    if theme.is_empty() {
        theme = gtk_fetch("gtk-icon-theme-name");
    }

    theme
}


pub fn cursor_theme() -> String {
    let mut theme = make_me_pretty(
        &run_command(
            "gsettings",
            &["get", "org.gnome.desktop.interface", "cursor-theme"],
        )
    );

    if theme.is_empty() {
        theme = gtk_fetch("gtk-cursor-theme-name");
    }

    theme
}

pub fn gtk_font() -> String {
    let mut font = make_me_pretty(
        &run_command(
            "gsettings",
            &["get", "org.gnome.desktop.interface", "font-name"],
        )
    );

    if font.is_empty() {
        font = gtk_fetch("gtk-font-name");
    }

    font
}

fn format_frequency(freq: u64, round_to: usize) -> String {
    let freq_ghz = freq as f64 / 1_000_000.0;
    if freq_ghz > 1.0 {
        format!("{freq_ghz:.precision$}GHz", precision = round_to)
    } else {
        let freq_mhz = freq as f64 / 1_000.0;
        format!("{freq_mhz:.precision$}MHz", precision = round_to)
    }
}

pub fn cpu(round_to: usize, full_name: bool, colorize: bool) -> String {
    let content = std::fs::read_to_string("/proc/cpuinfo").unwrap_or_default();

    let cpu_data: Vec<&str> = content
        .lines()
        .filter(|line| line.starts_with("model name"))
        .collect();

    let cpu_count = cpu_data.len();
    let mut cpu_info = cpu_data
        .first()
        .and_then(|line| line.split_once(':'))
        .map(|(_, value)| value.trim().to_string())
        .unwrap_or_default();

    if !full_name {
        if cpu_info.contains("AMD") {
            cpu_info = cpu_info.replace("AMD", "");
        } else if cpu_info.contains("Intel(R) Core(TM)") {
            cpu_info = cpu_info.replace("Intel(R) Core(TM)", "");
        }
    }

    let max_freq = std::fs::read_to_string(
        "/sys/devices/system/cpu/cpu0/cpufreq/cpuinfo_max_freq",
    )
    .unwrap_or_default()
    .trim()
    .parse::<u64>()
    .unwrap_or(0);

    let cpu_freq_info = format_frequency(max_freq, round_to);

    let mut full_cpu_info = format!(
        "{} ({}) @ {}",
        cpu_info.trim(),
        cpu_count,
        cpu_freq_info
    );

    if colorize {
        if cpu_info.contains("AMD") {
            full_cpu_info = format!(
                "{}{}{}",
                colors::get_color(1, true),
                full_cpu_info,
                colors::reset()
            );
        } else if cpu_info.contains("INTEL")
            || cpu_info.contains("i7")
            || cpu_info.contains("i5")
            || cpu_info.contains("i3")
        {
            full_cpu_info = format!(
                "{}{}{}",
                colors::get_color(4, true),
                full_cpu_info,
                colors::reset()
            );
        }
    }
    
    full_cpu_info
}

pub fn memory(
    gib: bool, 
    round_to: usize, 
    colorize: bool,
    reset_color: &str,
) -> String {
    let content = match std::fs::read_to_string("/proc/meminfo") {
        Ok(content) => content,
        Err(_) => return String::new(),
    };

    let mut memory_total = 0.0;
    let mut memory_used = 0.0;
    let mut memory_free = 0.0;

    for line in content.lines() {
        let mut parts = line.split_whitespace();

        let name = match parts.next() {
            Some(name) => name,
            None => continue,
        };

        let value: f64 = match parts.next().and_then(|value| value.parse().ok()) {
            Some(value) => value,
            None => continue,
        };

        match name {
            "MemTotal:" => memory_total = value / 1024.0,
            "MemFree:" | "Buffers:" | "Cached:" | "SReclaimable:" => {
                memory_free += value / 1024.0;
            }
            "Shmem:" => memory_used += value / 1024.0,
            _ => {}
        }
    }

    memory_used = memory_total + memory_used - memory_free;
    let memory_percent = if memory_total > 0.0 {
        ((memory_used / memory_total) * 100.0).floor() as u64
    } else {
        0
    };

    let (used, total) = if gib {
        (
            format!("{:.precision$} GiB", memory_used / 1024.0, precision = round_to),
            format!("{:.precision$} GiB", memory_total / 1024.0, precision = round_to),
        )
    } else {
        (
            format!("{:.precision$} MiB", memory_used, precision = round_to),
            format!("{:.precision$} MiB", memory_total, precision = round_to),
        )
    };
    
    let mut percent = memory_percent.to_string();

    if colorize {
        percent = format!(
            "{}{}%{}",
            colors::get_color(2, true),
            percent,
            reset_color,
        );
    } else {
        percent.push('%');
    }

    format!("{used} / {total} ({percent})")
}

pub fn color_palette(background: bool, char: &str, normal_only: bool) -> String {
    let mut out = vec![String::new()];
    let fg = !background;

    for i in 0..16 {
        if normal_only && i == 8 {
            return out[0].clone();
        }

        if i == 8 {
            out.push(String::new());
        }

        out[i / 8].push_str(colors::get_color(i, fg));
        out[i / 8].push_str(char);
        out[i / 8].push_str(colors::reset());
    }

    format!("%^&{}", out.join("%!&"))
}

pub fn monitor() -> String {
    let drm_path = std::path::Path::new("/sys/class/drm");

    let entries = match std::fs::read_dir(drm_path) {
        Ok(entries) => entries,
        Err(_) => return String::new(),
    };

    let mut monitors = Vec::new();

    for entry in entries.flatten() {
        let path = entry.path();

        let status = match std::fs::read_to_string(path.join("status")) {
            Ok(status) => status,
            Err(_) => continue,
        };

        if status.trim() != "connected" {
            continue;
        }

        let modes = match std::fs::read_to_string(path.join("modes")) {
            Ok(modes) => modes,
            Err(_) => continue,
        };

        if let Some(mode) = modes.lines().next() {
            monitors.push(mode.to_string());
        }
    }

    if monitors.len() > 1 {
        format!("%^&{}", monitors.join("%!&"))
    } else {
        monitors.first().cloned().unwrap_or_default()
    }
}

fn filesystem_type(path: &str) -> String {
    let path = match std::ffi::CString::new(path) {
        Ok(path) => path,
        Err(_) => return String::new(),
    };

    let mut stat = std::mem::MaybeUninit::<libc::statfs>::uninit();

    let result = unsafe {
        libc::statfs(path.as_ptr(), stat.as_mut_ptr())
    };

    if result != 0 {
        return String::new();
    }

    let stat = unsafe { stat.assume_init() };

    match stat.f_type as u64 {
        0x9123683E => "btrfs",
        0xEF53 => "ext4",
        0x58465342 => "xfs",
        0x01021994 => "tmpfs",
        0x6969 => "nfs",
        0x794C7630 => "overlay",
        _ => "unknown",
    }
    .to_string()
}

pub fn disk(
    path: &str,
    colorize: bool,
    file_system: bool,
    percent: bool,
    round_mem_to: usize,
    reset_color: &str,
) -> String {
    let c_path = match std::ffi::CString::new(path) {
        Ok(path) => path,
        Err(_) => return String::new(),
    };

    let mut stat = std::mem::MaybeUninit::<libc::statvfs>::uninit();

    let result = unsafe {
        libc::statvfs(c_path.as_ptr(), stat.as_mut_ptr())
    };

    if result != 0 {
        return String::new();
    }

    let stat = unsafe { stat.assume_init() };

    let block_size = stat.f_frsize as u64;
    let total_space = block_size * stat.f_blocks as u64;
    let free_space = block_size * stat.f_bfree as u64;
    let used_space = total_space - free_space;

    let bytes_per_gib = 1024_u64.pow(3);

    let (total_space_gib, used_space_gib) = if round_mem_to > 0 {
        (
            total_space as f64 / bytes_per_gib as f64,
            used_space as f64 / bytes_per_gib as f64,
        )
    } else {
        (
            (total_space / bytes_per_gib) as f64,
            (used_space / bytes_per_gib) as f64,
        )
    };

    let mut res = if round_mem_to > 0 {
        format!(
            "{:.precision$} GiB / {:.precision$} GiB",
            used_space_gib,
            total_space_gib,
            precision = round_mem_to,
        )
    } else {
        format!(
            "{} GiB / {} GiB",
            used_space_gib as u64,
            total_space_gib as u64,
        )
    };

    if percent {
        let percent_used = (used_space as f64 / total_space as f64) * 100.0;

        let percent = if colorize {
            format!(
                "{}{:.2}%{}",
                colors::get_color(2, true),
                percent_used,
                reset_color,
            )
        } else {
            format!("{:.2}%", percent_used)
        };

        res.push_str(&format!(" ({percent})"));
    }

    if file_system {
        let fs = filesystem_type(path);

        if !fs.is_empty() {
            res.push_str(&format!(" - {fs}"));
        }
    }

    res
}

pub fn gpu(full_name: bool, colorize: bool, reset_color: &str) -> String {
    let mut gpus = get_cache("gpus", 30).unwrap_or_default();

    if gpus.is_empty() && command_exists("lspci") {
        let output = run_command("lspci", &[]);

        for line in output.lines() {
            if !line.contains("VGA") && !line.contains("3D") {
                continue;
            }

            if let Some(gpu) = line
                .split('[')
                .nth(1)
                .and_then(|gpu| gpu.split(']').next())
            {
                gpus.push(gpu.to_string());
            }
        }

        set_cache("gpus", &gpus);
    }

    for gpu in &mut gpus {
        let gpu_lower = gpu.to_lowercase();

        if full_name {
            if gpu_lower.contains("geforce") && !gpu_lower.contains("nvidia") {
                *gpu = format!("NVIDIA {gpu}");
            } else if gpu_lower.contains("radeon") && !gpu_lower.contains("amd") {
                *gpu = format!("AMD {gpu}");
            } else if gpu_lower.contains("graphics") && !gpu_lower.contains("intel") {
                *gpu = format!("Intel {gpu}");
            }
        }


        if colorize {
            if gpu.contains("NVIDIA") {
                *gpu = format!(
                    "{}{}{}",
                    colors::get_color(2, true),
                    gpu,
                    reset_color,
                );
            } else if gpu.contains("AMD") {
                *gpu = format!(
                    "{}{}{}",
                    colors::get_color(1, true),
                    gpu,
                    reset_color,
                );
            } else if gpu.contains("Intel") {
                *gpu = format!(
                    "{}{}{}",
                    colors::get_color(5, true),
                    gpu,
                    reset_color,
                );
            }
        }
    }

    match gpus.len() {
        0 => String::new(),
        1 => gpus[0].clone(),
        _ => format!("{}", gpus.join(" / ")),
    }
}

pub fn gpu_driver(single_driver: bool) -> String {
    let mut drivers = get_cache("drivers", 10).unwrap_or_default();

    if drivers.is_empty() && command_exists("lspci") {
        let output = run_command("lspci", &[]);

        for line in output.lines() {
            if !line.contains("VGA") && !line.contains("3D") {
                continue;
            }

            let pci_id = match line.split_whitespace().next() {
                Some(id) => id,
                None => continue,
            };

            let driver_output = run_command(
                "lspci",
                &["-vv", "-s", pci_id],
            );

            let driver = driver_output
                .lines()
                .find_map(|line| {
                    line.trim()
                        .strip_prefix("Kernel driver in use:")
                        .map(str::trim)
                });

            if let Some(driver) = driver {
                if driver == "nvidia" {
                    let version = std::fs::read_to_string("/proc/driver/nvidia/version")
                        .ok()
                        .and_then(|content| {
                            content
                                .split("  ")
                                .nth(1)
                                .map(str::trim)
                                .map(String::from)
                        })
                        .unwrap_or_default();

                    let kernel_version = run_command("uname", &["-r"]);

                    let dkms_path = format!(
                        "/lib/modules/{}/updates/dkms",
                        kernel_version.trim()
                    );

                    let driver_name = if std::path::Path::new(&dkms_path).exists() {
                        format!("nvidia-dkms {}", version)
                    } else {
                        format!("nvidia {}", version)
                    };

                    drivers.push(driver_name);
                } else {
                    drivers.push(driver.to_string());
                }
            }
        }

        set_cache("drivers", &drivers);
    }

    if drivers.is_empty() {
        String::new()
    } else if drivers.len() > 1 && !single_driver {
        format!("{}", drivers.join(" / "))
    } else {
        drivers[0].clone()
    }
}
