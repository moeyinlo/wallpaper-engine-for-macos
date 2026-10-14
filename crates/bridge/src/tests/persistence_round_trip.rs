use crate::config::{AppConfig, ConfigStore, MonitorCfg, MonitorRender, MonitorSettingsCfg, SerializedSelector, WallpaperConfig};

#[test]
fn app_config_round_trips_monitor_assignment() {
    let root = tempfile::tempdir().unwrap();
    let store = ConfigStore::open(root.path().to_path_buf());
    let mut config = AppConfig::default();
    config.ui.filter.scene = false;
    config.monitors.push(MonitorCfg {
        selector: SerializedSelector::Primary,
        enabled: true,
        mode: "independent".to_string(),
        wallpaper: Some("3470764447".to_string()),
        mirror_target: None,
    });

    store.save_app_config(&config).unwrap();

    let reloaded = ConfigStore::open(root.path().to_path_buf())
        .load()
        .unwrap()
        .config;
    assert_eq!(reloaded, config);
}

#[test]
fn crop_position_defaults_and_round_trips_for_wallpaper_and_mirror() {
    let legacy_wallpaper: MonitorRender = serde_json::from_str(r#"{"scaling_mode":"fill","scaling_factor":1.25}"#).unwrap();
    assert_eq!((legacy_wallpaper.position_x, legacy_wallpaper.position_y), (0.5, 0.5));
    let legacy_mirror: MonitorSettingsCfg = serde_json::from_str(r#"{"kind":"primary","scaling_mode":"fit","scaling_factor":1.0}"#).unwrap();
    assert_eq!((legacy_mirror.position_x, legacy_mirror.position_y), (0.5, 0.5));

    let mut wallpaper = WallpaperConfig::default();
    wallpaper.monitors.push(MonitorRender {
        position_x: 0.2,
        position_y: 0.8,
        scaling_mode: "free".to_string(),
        ..MonitorRender::default()
    });
    let decoded: WallpaperConfig = serde_json::from_str(&serde_json::to_string(&wallpaper).unwrap()).unwrap();
    assert_eq!(decoded, wallpaper);
    assert_eq!(decoded.monitors[0].parse_scaling_mode(), wallpaper_core::project::ScalingMode::Free);
}
