use tao::event::{Event, WindowEvent};
use tao::event_loop::{ControlFlow, EventLoop};
use tao::window::WindowBuilder;
use wry::WebViewBuilder;

use crate::views::builder::Builder;

mod views;

fn main() {
    let event_loop = EventLoop::new();
    
    let window = WindowBuilder::new()
        .with_title("ScraPiP")
        .with_inner_size(tao::dpi::LogicalSize::new(900.0, 650.0))
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


    #[cfg(target_os = "linux")]
    let _webview = {
        use gtk::prelude::*;
        use tao::platform::unix::WindowExtUnix;
        use wry::WebViewBuilderExtUnix;

        let vbox = window.default_vbox().unwrap();
        let fixed = gtk::Fixed::new();
        fixed.show_all();
        vbox.pack_start(&fixed, true, true, 0);
        
        WebViewBuilder::new()
            .with_html(&home_view)
            .with_devtools(false)
            .build_gtk(&fixed)
            .unwrap()
    };

    #[cfg(not(target_os = "linux"))]
    let _webview = WebViewBuilder::new()
        .with_html(&home_view)
        .with_devtools(false)
        .build(&window)
        .unwrap();

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
