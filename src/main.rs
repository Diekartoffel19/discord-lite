#![windows_subsystem = "windows"] // no console window

use tao::{
    dpi::LogicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use wry::{NewWindowResponse, WebContext, WebViewBuilder, WebViewBuilderExtWindows};

const URL: &str = "https://discord.com/app";

fn main() -> wry::Result<()> {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Discord Lite")
        .with_inner_size(LogicalSize::new(1280.0, 800.0))
        .build(&event_loop)
        .expect("window");

    // Own profile dir so the login survives restarts.
    let data_dir = dirs::data_local_dir().unwrap().join("discord-lite");
    let mut ctx = WebContext::new(Some(data_dir));

    let _webview = WebViewBuilder::new_with_web_context(&mut ctx)
        .with_url(URL)
        // wry defaults + Chromium flags that trim background processes and memory.
        .with_additional_browser_args(
            "--disable-features=msWebOOUI,msPdfOOUI,msSmartScreenProtection \
             --disable-background-networking --disable-component-update \
             --disable-sync --process-per-site --no-first-run",
        )
        // Links that open a new window go to the default browser instead.
        .with_new_window_req_handler(|url, _| {
            if url.starts_with("http") {
                let _ = std::process::Command::new("explorer").arg(&url).spawn();
            }
            NewWindowResponse::Deny
        })
        .build(&window)?;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;
        if let Event::WindowEvent { event: WindowEvent::CloseRequested, .. } = event {
            *control_flow = ControlFlow::Exit;
        }
    });
}
