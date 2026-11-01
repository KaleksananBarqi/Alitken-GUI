use crate::utils;
use serde::{Deserialize, Serialize};
use std::fs;
#[cfg(target_os = "windows")]
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct IntegrationStatus {
    pub sendto_active: bool,
    pub executable_path: String,
}

/// Resolves the Windows SendTo folder: %APPDATA%\Microsoft\Windows\SendTo
#[cfg(target_os = "windows")]
pub fn get_sendto_folder() -> Option<PathBuf> {
    if let Some(appdata) = dirs::config_dir() {
        let sendto = appdata.join("Microsoft").join("Windows").join("SendTo");
        if sendto.exists() || fs::create_dir_all(&sendto).is_ok() {
            return Some(sendto);
        }
    }
    None
}

/// Resolves the target shortcut path for SendTo
#[cfg(target_os = "windows")]
pub fn get_sendto_shortcut_path() -> Option<PathBuf> {
    get_sendto_folder().map(|dir| dir.join("Alitken Media Converter.lnk"))
}

/// Checks if the SendTo / File Manager shortcut exists
#[cfg(target_os = "windows")]
pub fn is_sendto_active() -> bool {
    if let Some(path) = get_sendto_shortcut_path() {
        path.exists()
    } else {
        false
    }
}

#[cfg(not(target_os = "windows"))]
pub fn is_sendto_active() -> bool {
    if let Some(home) = dirs::home_dir() {
        let kde_service = home.join(".local/share/kio/servicemenus/alitken.desktop");
        let nautilus_script = home.join(".local/share/nautilus/scripts/Convert with Alitken");
        let desktop_entry = home.join(".local/share/applications/com.alitken.mediaconverter.desktop");
        return kde_service.exists() || nautilus_script.exists() || desktop_entry.exists();
    }
    false
}

/// Creates or deletes the SendTo / File Manager shortcut
#[cfg(target_os = "windows")]
pub fn set_sendto_shortcut(enable: bool) -> Result<bool, String> {
    let shortcut_path = get_sendto_shortcut_path()
        .ok_or_else(|| "Could not locate Windows SendTo directory".to_string())?;

    if enable {
        let current_exe = std::env::current_exe()
            .map_err(|e| format!("Failed to resolve current executable path: {}", e))?;

        let target_str = current_exe.to_string_lossy();
        let shortcut_str = shortcut_path.to_string_lossy();

        // Create shortcut using PowerShell WScript.Shell
        let ps_script = format!(
            "$w = New-Object -ComObject WScript.Shell; $s = $w.CreateShortcut('{}'); $s.TargetPath = '{}'; $s.Description = 'Alitken Media Converter & Video Splitter'; $s.Save()",
            shortcut_str.replace("'", "''"),
            target_str.replace("'", "''")
        );

        let output = utils::create_hidden_cmd("powershell")
            .args(["-NoProfile", "-Command", &ps_script])
            .output()
            .map_err(|e| format!("Failed to execute PowerShell: {}", e))?;

        if !output.status.success() {
            let err = String::from_utf8_lossy(&output.stderr);
            let err_msg = format!("Failed to create SendTo shortcut: {}", err);
            utils::log_error(&err_msg);
            return Err(err_msg);
        }

        utils::log_info(&format!("Created SendTo shortcut at {:?}", shortcut_path));
        Ok(true)
    } else {
        if shortcut_path.exists() {
            fs::remove_file(&shortcut_path)
                .map_err(|e| format!("Failed to remove SendTo shortcut: {}", e))?;
        }
        utils::log_info("Removed SendTo shortcut");
        Ok(false)
    }
}

#[cfg(not(target_os = "windows"))]
pub fn set_sendto_shortcut(enable: bool) -> Result<bool, String> {
    let home = dirs::home_dir().ok_or_else(|| "Could not locate user home directory".to_string())?;
    let kde_dir = home.join(".local/share/kio/servicemenus");
    let kde_service = kde_dir.join("alitken.desktop");
    let nautilus_dir = home.join(".local/share/nautilus/scripts");
    let nautilus_script = nautilus_dir.join("Convert with Alitken");
    let app_dir = home.join(".local/share/applications");
    let desktop_entry = app_dir.join("com.alitken.mediaconverter.desktop");

    if enable {
        let current_exe = std::env::current_exe()
            .map_err(|e| format!("Failed to resolve current executable path: {}", e))?;
        let exe_path_str = current_exe.to_string_lossy();

        // 1. KDE Dolphin Service Menu
        let _ = fs::create_dir_all(&kde_dir);
        let kde_content = format!(
            "[Desktop Entry]\n\
             Type=Service\n\
             ServiceTypes=KonqPopupMenu/Plugin\n\
             MimeType=video/*;image/*;audio/*;\n\
             Actions=convertWithAlitken;\n\n\
             [Desktop Action convertWithAlitken]\n\
             Name=Convert with Alitken\n\
             Icon=com.alitken.mediaconverter\n\
             Exec=\"{}\" %F\n",
            exe_path_str
        );
        let _ = fs::write(&kde_service, kde_content);

        // 2. GNOME Nautilus Script
        let _ = fs::create_dir_all(&nautilus_dir);
        let nautilus_content = format!(
            "#!/bin/bash\n\
             \"{}\" \"$@\"\n",
            exe_path_str
        );
        let _ = fs::write(&nautilus_script, nautilus_content);
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let _ = fs::set_permissions(&nautilus_script, fs::Permissions::from_mode(0o755));
        }

        // 3. User XDG Desktop Entry
        let _ = fs::create_dir_all(&app_dir);
        let desktop_content = format!(
            "[Desktop Entry]\n\
             Name=Alitken\n\
             Comment=Alitken Media Converter & Video Splitter\n\
             Exec=\"{}\" %U\n\
             Icon=com.alitken.mediaconverter\n\
             Terminal=false\n\
             Type=Application\n\
             Categories=AudioVideo;AudioVideoEditing;\n\
             MimeType=video/*;audio/*;image/*;\n",
            exe_path_str
        );
        let _ = fs::write(&desktop_entry, desktop_content);

        utils::log_info("Created Linux file manager shortcuts (KDE Service Menu, Nautilus Script, XDG Desktop)");
        Ok(true)
    } else {
        if kde_service.exists() {
            let _ = fs::remove_file(&kde_service);
        }
        if nautilus_script.exists() {
            let _ = fs::remove_file(&nautilus_script);
        }
        if desktop_entry.exists() {
            let _ = fs::remove_file(&desktop_entry);
        }
        utils::log_info("Removed Linux file manager shortcuts");
        Ok(false)
    }
}/// Returns full integration status
pub fn get_integration_status() -> IntegrationStatus {
    let current_exe = std::env::current_exe()
        .map(|p| p.to_string_lossy().to_string())
        .unwrap_or_default();

    IntegrationStatus {
        sendto_active: is_sendto_active(),
        executable_path: current_exe,
    }
}
