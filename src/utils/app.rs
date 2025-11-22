extern crate glium;
use std::{num::NonZeroU32, time::{Duration, Instant}};
use glium::{glutin::{display::GetGlDisplay, self, prelude::{GlDisplay, NotCurrentGlContext}, surface::WindowSurface}, winit::{dpi::LogicalSize, error::EventLoopError, event::{Event, StartCause}, event_loop::{ControlFlow, EventLoop}, raw_window_handle::HasWindowHandle, window::{Window, WindowAttributes, Icon}}};
use glutin_winit::DisplayBuilder;

#[derive(Clone,Copy)]
pub enum Action {
    Stop,
    Continue,
}

pub struct Inertia {}

impl Inertia {
    pub fn new() -> (
        glium::Display<WindowSurface>,
        EventLoop<()>,
        Window,
        glutin::config::Config) {

        let event_loop = EventLoop::new().expect("Eventloop failed to be created");

    // Load window icon
        let icon_image = image::open("inertia-app.png")
            .expect("Failed to load icon")
            .to_rgba8();
        let (icon_width, icon_height) = icon_image.dimensions();
        let icon_rgba = icon_image.into_raw();
        let icon = Icon::from_rgba(icon_rgba, icon_width, icon_height)
            .expect("Failed to create icon");

    // ATTRIBUTES
        let window_attributes = WindowAttributes::default()
            .with_resizable(true)
            .with_inner_size(LogicalSize::new(1024, 700))
            .with_title("Inertia")
            .with_window_icon(Some(icon));
        let template_builder = glutin::config::ConfigTemplateBuilder::new();
        let display_builder = DisplayBuilder::new().with_window_attributes(Some(window_attributes));

    // WINDOW AND GL CONFIG
        let (window, cfg) = display_builder.build(&event_loop, template_builder, |mut configs|{
            // Just use the first configuration since we don't have any special preferences here
            configs.next().unwrap()
        }).unwrap();

        let window = window.unwrap();

        let window_handle = window.window_handle().expect("couldn't obtain window handle");
        let context_attributes = glutin::context::ContextAttributesBuilder::new().build(Some(window_handle.into()));
        let fallback_context_attributes = glutin::context::ContextAttributesBuilder::new()
            .with_context_api(glutin::context::ContextApi::Gles(None))
            .build(Some(window_handle.into()));

        let not_current_gl_context = Some(unsafe {
            cfg.display().create_context(&cfg, &context_attributes).unwrap_or_else(|_| {
                cfg.display()
                    .create_context(&cfg, &fallback_context_attributes)
                    .expect("failed to create context")
            })
        });

        let (width, height): (u32, u32) = window.inner_size().into();
        let attrs = glutin::surface::SurfaceAttributesBuilder::<WindowSurface>::new().build(
            window_handle.into(),
            NonZeroU32::new(width).unwrap(),
            NonZeroU32::new(height).unwrap(),
        );

        let surface = unsafe { cfg.display().create_window_surface(&cfg, &attrs).unwrap() };
        let current_context = not_current_gl_context.unwrap().make_current(&surface).unwrap();
        let display = glium::Display::from_context_surface(current_context, surface).unwrap();

        return ( display, event_loop, window, cfg)
    }

    pub fn update<F>(event_loop: EventLoop<()>, mut callback: F) -> Result<(), EventLoopError>
    where F: 'static + FnMut(&Vec<Event<()>>) -> Action {
            let mut buffer = Vec::new();
            #[allow(deprecated)]
            event_loop.run(move |event: Event<()>, window_target| {
                let mut next_frame_time = std::time::Instant::now();
            
                let run_callback = match event {
                    Event::NewEvents(cause) => {
                        match cause {
                            StartCause::ResumeTimeReached { .. } | StartCause::Init => {
                                true
                            },
                            _ => false
                        }
                    },
                    event => {
                        buffer.push(event);
                        false
                    }
                    _ => {
                        false 
                    }
                };
        
                let action = if run_callback {
                    let action = callback(&buffer);
                    next_frame_time = Instant::now() + Duration::from_nanos(16666667);
        
                    buffer.clear();
                    action
                } else {
                    Action::Continue
                };
        
                match action {
                    Action::Continue => {
                        window_target.set_control_flow(ControlFlow::WaitUntil(next_frame_time));
                    },
                    Action::Stop => window_target.exit()
                }
            })
    }
}