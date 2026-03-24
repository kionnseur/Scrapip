use tao::event::{Event, WindowEvent};
use tao::event_loop::{ControlFlow, EventLoop};
use tao::window::WindowBuilder;
use wry::WebViewBuilder;

use crate::views::Views;
use crate::views::builder::Builder;
use std::sync::{Arc, Mutex};

mod db;
mod views;

fn main() {
    let conn = db::init_db("scrapip.db").unwrap();
    let db = Arc::new(Mutex::new(conn));

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("ScraPiP")
        .build(&event_loop)
        .unwrap();

    let mut view_builder = views::builder::ViewsBuilder::default();
    let home_view = view_builder
        .set_header("<header><h1>Header</h1></header>")
        .set_body("<main><p>Home page of ScraPiP.</p></main>")
        .set_footer("<footer><p>Footer</p></footer>")
        .set_style("<style>body{animation:a 2s infinite alternate}@keyframes a{to{transform:scale(1.1)}}</style>")
        .set_script("<script>alert('Home page loaded');</script>")
        .build();

    let _webview = generate_webview(&window, home_view);

    event_loop.run(move |event, _, control_flow| {
        *control_flow = match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => ControlFlow::Exit,
            _ => ControlFlow::Wait,
        };
    });
}

fn generate_webview(window: &tao::window::Window, html_content: Views) -> wry::WebView {
    #[cfg(target_os = "linux")]
    let _webview = {
        use gtk::prelude::*;
        use tao::platform::unix::WindowExtUnix;
        use wry::WebViewBuilderExtUnix;

        let vbox = window.default_vbox().expect("Failed to get default vbox");

        vbox.show_all();

        let webview = WebViewBuilder::new()
            .with_html(&html_content)
            .with_devtools(false)
            .build_gtk(vbox)
            .expect("Failed to build WebView for Linux");

        webview
    };

    #[cfg(not(target_os = "linux"))]
    let _webview = WebViewBuilder::new()
        .with_html(&html_content)
        .with_devtools(false)
        .build(&window)
        .expect("Failed to build WebView for non-Linux platforms");

    _webview
}
