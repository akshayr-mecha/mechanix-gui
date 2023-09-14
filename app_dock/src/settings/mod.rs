use crate::errors::{AppDockError, AppDockErrorCodes};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use tracing::{info, debug};
use std::{env, fs::File, path::PathBuf};
use anyhow::bail;

/// # App Dock Settings
/// 
/// Struct representing the settings.yml configuration file,
/// this file lets you control the behavior of the app dock, 
/// apply custom theme and fonts
#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct AppDockSettings {
    pub app: AppSettings,
    pub window: WindowSettings, // Window Settings
    pub title: String,  // Sets the window title
    pub layout: LayoutSettings,
    pub modules: Modules
}

impl Default for AppDockSettings {
    fn default() -> Self {
        Self {
            app: AppSettings::default(),
            window: WindowSettings::default(),
            title: String::from("App Dock"),
            layout: LayoutSettings::default(),
            modules: Modules::default(),
        }
    }
}

/// # App Settings
/// 
/// Struct part of settings.yml to control the application 
/// behavior, includes optimizations and defaults
#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct AppSettings {
    pub id: Option<String>, // Process ID
    pub text_multithreading: bool,  // Enable text multithreading
    pub antialiasing: bool, // Enable antialiasing
    pub try_opengles_first: bool,   // Enable using OpenGL ES before OpenGL (only for flow)
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            id: Some(String::from("app-dock")),
            text_multithreading: false,
            antialiasing: false,
            try_opengles_first: true
        }
    }
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct WindowSize {
   pub default: (u32, u32),
    pub minimized: (u32, u32),
    pub maximized: (u32, u32),
    pub other: (u32, u32)
}
/// # Window Settings
/// 
/// Part of the settings.yml to control the behavior of 
/// the application window
#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct WindowSettings {
    pub size: WindowSize,   // Size of the window
    pub position: (i32, i32),   // Default position to start window
    pub min_size: Option<(u32, u32)>,    // Minimum size the window can be resized to
    pub max_size: Option<(u32, u32)>,   // Maximum size the window can be resized to
    pub visible: bool,   // Sets visibility of the window
    pub resizable: bool,    // Enables or disables resizing
    pub decorations: bool,  // Enables or disables the title bar
    pub transparent: bool,  // Enables transparency
    pub always_on_top: bool,    // Forces window to be always on top
    pub icon_path: Option<String>,
}

/// # Layout Settings
///
/// Part of the settings.yml to control the behavior of
/// the layout of options in the app dock.
#[derive(Debug, Deserialize, Clone, Serialize)]
#[derive(Default)]
pub struct LayoutSettings {
    pub left: Vec<String>, //Items that will in left side of app dock
    pub center: Vec<String>, //Items that will in center of app dock
    pub right: Vec<String>, //Items that will in right side of app dock
}

/// # Modules
///
/// App that will be visible in app drawer
#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct App {
    pub app_id: String,
    pub name: String,
    pub alias: String,
    pub icon: String
}
/// # Modules
///
/// App that will be visible in app drawer
#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct Home {
    pub icon: Option<String>,
}


/// # Modules
///
/// Options that will be visible in dock
#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct Modules {
    pub pinned_apps: Vec<App>,
    pub home: Home
}

impl Default for WindowSettings {
    fn default() -> Self {
        Self {
            size: WindowSize{
                default: (1024, 768),
                minimized: (1024, 768),
                maximized: (1024, 768),
                other: (1024, 768),
            },
            position: (0, 0),
            min_size: None,
            max_size: None,
            visible: true,
            resizable: true,
            decorations: true,
            transparent: false,
            always_on_top: false,
            icon_path: None
        }
    }
}



impl Default for Modules {
    fn default() -> Self {
        Self {
            pinned_apps: vec![],
            home: Home { icon: None },
        }
    }
}

/// # Reads Settings path from arg
/// 
/// Reads the `-s` or `--settings` argument for the path
pub fn read_settings_path_from_args() -> Option<String> {
    let args: Vec<String> = env::args().collect();
    if args.len() > 2 && (args[1] == "-s" || args[1] == "--settings") {
        debug!("Using settings path from argument - {}", args[2]);
        return Some(args[2].clone());
    }
    None
}

/// # Reads Settings YML 
/// 
/// Reads the `settings.yml` and parsers to AppDockSettings
/// 
/// **Important**: Ensure all fields are present in the yml due to strict parsing
pub fn read_settings_yml() -> Result<AppDockSettings> {
    let mut file_path = PathBuf::from(std::env::var("MECHA_APP_DOCK_SETTINGS_PATH")
        .unwrap_or(String::from("settings.yml"))); // Get path of the library

    // read from args
    let file_path_in_args = read_settings_path_from_args();
    if file_path_in_args.is_some() {
        file_path = PathBuf::from(file_path_in_args.unwrap());
    }

    info!(task = "read_settings", "settings file location - {:?}", file_path);

    // open file
    let settings_file_handle = match File::open(file_path) {
        Ok(file) => file,
        Err(e) => {
            bail!(AppDockError::new(
                AppDockErrorCodes::SettingsReadError,
                format!("cannot read the settings.yml in the path - {}", e),
            ));
        }
    };

    // read and parse
    let config: AppDockSettings = match serde_yaml::from_reader(settings_file_handle) {
        Ok(config) => config,
        Err(e) => {
            bail!(AppDockError::new(
                AppDockErrorCodes::SettingsParseError,
                format!("error parsing the settings.yml - {}", e),
            ));
        }
    };

    Ok(config)
}
