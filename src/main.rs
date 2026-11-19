//! Runs the Rust application directly on the machine's native backend.

#![cfg_attr(target_os = "windows", windows_subsystem = "windows")]

#[cfg(target_os = "linux")]
use nsh_platform_linux as platform;
#[cfg(target_os = "macos")]
use nsh_platform_macos as platform;
#[cfg(target_os = "windows")]
use nsh_platform_windows as platform;

#[cfg(any(target_os = "linux", target_os = "macos", target_os = "windows"))]
fn main() -> impl std::process::Termination {
    env_logger::init();
    // 独立二进制不走 nsh 的打包流水线，nsh.json/theme.json 没人读：
    // 主题与像素字体直接编进 Root（Lo-PDA 四色绿 LCD 风格）。
    let tokens = nsh_base::tokens_of(Some(include_str!("../theme.json")))
        .unwrap_or_else(|e| panic!("theme.json 解析失败：{e}"));
    let dotgothic: std::sync::Arc<[u8]> =
        include_bytes!("../assets/fonts/DotGothic16-Regular.ttf").as_slice().into();
    let vt323: std::sync::Arc<[u8]> =
        include_bytes!("../assets/fonts/VT323-Regular.ttf").as_slice().into();
    let root = platform::Root::of::<hipda_nsh::App>()
        .tokens(tokens)
        .font(dotgothic)
        .font(vt323);
    platform::run_app(
        platform::AppSource::Rust(root),
        platform::AppOptions {
            title: "HiPDA·nsh 版".to_owned(),
            #[cfg(any(target_os = "linux", target_os = "windows"))]
            app_id: "cn.hipda.nsh.unofficial".to_owned(),
            size: (900, 680),
            ..Default::default()
        },
    )
}

#[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
compile_error!(
    "Run this binary on macOS, Windows or Linux; use the TSX entry with nsh run for iOS and Android."
);
