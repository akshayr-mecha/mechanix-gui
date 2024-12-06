use const_format::concatcp;

pub const APP_ID: &str = "mechanix-settings";

// default locations for the configuration file (settings.yml) and icons when settings.yml not found in env/arg
pub const HOME_DIR_CONFIG_PATH: &str = "/.config";
pub const USR_SHARE_PATH: &str = "/usr/share";
pub const APP_PATH: &str = "/mechanix/apps/settings-app";

pub const BASE_SETTINGS_PATH: &str = concatcp!(APP_PATH, "/settings.yml");
pub const ASSET_PATH: &str = concatcp!(USR_SHARE_PATH, APP_PATH, "/assets/"); // OG
pub const BACKGROUND_IMAGE: &str = concatcp!(ASSET_PATH, "mecha_background_color.png");

// settings
pub const BLUETOOTH_ICON: &str = concatcp!(ASSET_PATH, "bluetooth_icon.png");
pub const DISPLAY_ICON: &str = concatcp!(ASSET_PATH, "display_icon.png");
pub const APPEARANCE_ICON: &str = concatcp!(ASSET_PATH, "appearance_icon.png");
pub const BATTERY_ICON: &str = concatcp!(ASSET_PATH, "battery_icon.png");
pub const SOUND_ICON: &str = concatcp!(ASSET_PATH, "sound_icon.png");
pub const LOCK_ICON: &str = concatcp!(ASSET_PATH, "lock_icon.png");
pub const DATE_TIME_ICON: &str = concatcp!(ASSET_PATH, "date_time_icon.png");
pub const LANGUAGE_ICON: &str = concatcp!(ASSET_PATH, "language_icon.png");
pub const UPDATE_ICON: &str = concatcp!(ASSET_PATH, "update_icon.png");
pub const ABOUT_ICON: &str = concatcp!(ASSET_PATH, "about_icon.png");
pub const RIGHT_ARROW_ICON: &str = concatcp!(ASSET_PATH, "right_arrow.png");
pub const INFO_ICON: &str = concatcp!(ASSET_PATH, "info_icon.png");
pub const CONNECTED_ICON: &str = concatcp!(ASSET_PATH, "connected_icon.png");
pub const BACK_ICON: &str = concatcp!(ASSET_PATH, "back_icon.png");
pub const ADD_ICON: &str = concatcp!(ASSET_PATH, "back_icon.png");
pub const DELETE_ICON: &str = concatcp!(ASSET_PATH, "back_icon.png");
pub const TICK_ICON: &str = concatcp!(ASSET_PATH, "back_icon.png");

// wireless
pub const WIRELESS_OFF: &str = concatcp!(ASSET_PATH, "wireless/wireless_off_icon.png");
pub const WIRELESS_ON: &str = concatcp!(ASSET_PATH, "wireless/wireless_on_icon.png");
pub const WIRELESS_LOW: &str = concatcp!(ASSET_PATH, "wireless/wireless_low_icon.png");
pub const WIRELESS_WEAK: &str = concatcp!(ASSET_PATH, "wireless/wireless_weak_icon.png");
pub const WIRELESS_GOOD: &str = concatcp!(ASSET_PATH, "wireless/wireless_good_icon.png");
pub const WIRELESS_STRONG: &str = concatcp!(ASSET_PATH, "wireless/wireless_strong_icon.png");
pub const WIRELESS_ERROR: &str = concatcp!(ASSET_PATH, "wireless/wireless_error_icon.png");
pub const WIRELESS_SETTIGNS: &str = concatcp!(ASSET_PATH, "wireless/wireless_settings_icon.png");
pub const WIRELESS_NOT_FOUND: &str = concatcp!(ASSET_PATH, "wireless/wireless_not_found_icon.svg");

// wireless-secured
pub const SECURED_WIRELESS_OFF: &str =
    concatcp!(ASSET_PATH, "wireless/secured/secured_wireless_off_icon.png");
pub const SECURED_WIRELESS_ON: &str =
    concatcp!(ASSET_PATH, "wireless/secured/secured_wireless_on_icon.png");
pub const SECURED_WIRELESS_LOW: &str =
    concatcp!(ASSET_PATH, "wireless/secured/secured_wireless_low_icon.png");
pub const SECURED_WIRELESS_WEAK: &str = concatcp!(
    ASSET_PATH,
    "wireless/secured/secured_wireless_weak_icon.png"
);
pub const SECURED_WIRELESS_STRONG: &str = concatcp!(
    ASSET_PATH,
    "wireless/secured/secured_wireless_strong_icon.png"
);
pub const SECURED_WIRELESS_ERROR: &str = concatcp!(
    ASSET_PATH,
    "wireless/secured/secured_wireless_error_icon.png"
);
