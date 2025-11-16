use egui::{Color32, RichText, Stroke, ColorImage, TextureHandle};
use crate::{
    simulation::{orbital_simulation::GalaxySimulation, timeutil::SimulationTime},
    ui::hover_system::HoverSystem,
};

/// Load the Inertia logo as an egui texture
fn load_logo(ctx: &egui::Context) -> Option<TextureHandle> {
    let image_data = image::open("inertiaicon.png").ok()?;
    let image_buffer = image_data.to_rgba8();
    let (width, height) = image_buffer.dimensions();
    let pixels = image_buffer.into_raw();
    
    let color_image = ColorImage::from_rgba_unmultiplied(
        [width as usize, height as usize],
        &pixels,
    );
    
    Some(ctx.load_texture(
        "inertia_logo",
        color_image,
        Default::default(),
    ))
}

/// Debug visualization flags
#[derive(Clone)]
pub struct DebugVisualization {
    pub show_velocity_vectors: bool,
    pub show_acceleration_vectors: bool,
    pub show_orbital_trails: bool,
    pub show_grid: bool,
}

impl Default for DebugVisualization {
    fn default() -> Self {
        Self {
            show_velocity_vectors: false,
            show_acceleration_vectors: false,
            show_orbital_trails: false,
            show_grid: true,
        }
    }
}

/// Renders the professional physics engine UI
pub struct PhysicsUI;

impl PhysicsUI {
    /// Configure the dark professional theme
    pub fn configure_theme(ctx: &egui::Context) {
        let mut style = (*ctx.style()).clone();
        style.visuals.window_fill = Color32::from_rgba_premultiplied(25, 25, 30, 240);
        style.visuals.panel_fill = Color32::from_rgba_premultiplied(20, 20, 25, 250);
        style.visuals.window_stroke = Stroke::new(1.0, Color32::from_rgb(60, 60, 70));
        ctx.set_style(style);
    }

    /// Render the top toolbar
    pub fn render_toolbar(
        ctx: &egui::Context,
        time: &SimulationTime,
        hover_system: &HoverSystem,
        simulation: &GalaxySimulation,
    ) {
        egui::TopBottomPanel::top("toolbar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.add_space(25.0);
                
                // Display logo if available
                if let Some(logo) = load_logo(ctx) {
                    ui.add(egui::Image::new(&logo).max_size(egui::vec2(48.0, 48.0)));
                    ui.add_space(10.0);
                }
                
                ui.separator();
                ui.label(
                    RichText::new("Orbital Mechanics Simulator")
                        .size(12.0)
                        .color(Color32::GRAY),
                );

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.add_space(8.0);
                    let fps = 1.0 / time.delta_time.max(0.001);
                    ui.label(
                        RichText::new(format!("FPS: {:.0}", fps))
                            .size(11.0)
                            .color(Color32::LIGHT_GRAY),
                    );

                    // Show hovered planet info
                    if let Some(name) = hover_system.get_hovered_name(&simulation.object_names) {
                        ui.separator();
                        ui.label(
                            RichText::new(format!("🎯 {}", name))
                                .size(12.0)
                                .color(Color32::from_rgb(255, 220, 100)),
                        );
                    }
                });
            });
        });
    }

    /// Render the left control panel
    pub fn render_control_panel(
        ctx: &egui::Context,
        time: &SimulationTime,
        simulation: &GalaxySimulation,
        hover_system: &HoverSystem,
        debug_viz: &mut DebugVisualization,
    ) -> bool {
        let mut shoot_ray = false;
        egui::SidePanel::left("controls")
            .default_width(320.0)
            .min_width(280.0)
            .max_width(500.0)
            .resizable(true)
            .show(ctx, |ui| {
                // Main scroll area for the entire panel
                egui::ScrollArea::vertical()
                    .auto_shrink([false; 2])
                    .show(ui, |ui| {
                        ui.add_space(8.0);

                        Self::render_simulation_status(ui, time, simulation);
                        ui.add_space(12.0);

                        Self::render_body_list(ui, simulation, hover_system);
                        ui.add_space(12.0);
                        
                        shoot_ray = Self::render_debug_tools(ui, hover_system, debug_viz);
                        ui.add_space(12.0);

                        Self::render_controls_help(ui);
                        
                        ui.add_space(20.0); // Bottom padding
                    });
            });
        shoot_ray
    }

    /// Render simulation status section
    fn render_simulation_status(
        ui: &mut egui::Ui,
        time: &SimulationTime,
        simulation: &GalaxySimulation,
    ) {
        ui.heading(RichText::new("⚙ Simulation Status").color(Color32::WHITE));
        ui.add_space(4.0);

        egui::Frame::none()
            .fill(Color32::from_rgb(30, 30, 35))
            .inner_margin(12.0)
            .rounding(4.0)
            .show(ui, |ui| {
                // Status indicator
                let status_color = if time.paused {
                    Color32::from_rgb(255, 180, 0)
                } else {
                    Color32::from_rgb(0, 255, 100)
                };
                let status_text = if time.paused { "⏸ PAUSED" } else { "▶ RUNNING" };
                ui.label(
                    RichText::new(status_text)
                        .color(status_color)
                        .size(14.0)
                        .strong(),
                );

                ui.add_space(8.0);
                ui.separator();
                ui.add_space(6.0);
                
                // Basic info
                ui.horizontal(|ui| {
                    ui.label(RichText::new("Bodies:").color(Color32::LIGHT_GRAY));
                    ui.label(RichText::new(format!("{}", simulation.owned_objects.len())).strong());
                });
                
                ui.horizontal(|ui| {
                    ui.label(RichText::new("Time Scale:").color(Color32::LIGHT_GRAY));
                    ui.label(RichText::new(format!("{:.2}x", time.accelerated_time_factor)).strong());
                });
                
                ui.horizontal(|ui| {
                    ui.label(RichText::new("Gravity (G):").color(Color32::LIGHT_GRAY));
                    ui.label(RichText::new(format!("{:.2}", simulation.gravity_constant)).strong());
                }).response.on_hover_text("Universal gravitational constant");

                ui.add_space(6.0);
                ui.separator();
                ui.add_space(6.0);

                // Energy calculations
                let total_ke: f32 = simulation
                    .owned_objects
                    .iter()
                    .map(|b| 0.5 * b.mass * b.velocity.magnitude().powi(2))
                    .sum();
                    
                let total_mass: f32 = simulation
                    .owned_objects
                    .iter()
                    .map(|b| b.mass)
                    .sum();
                
                let avg_velocity: f32 = if !simulation.owned_objects.is_empty() {
                    simulation.owned_objects
                        .iter()
                        .map(|b| b.velocity.magnitude())
                        .sum::<f32>() / simulation.owned_objects.len() as f32
                } else {
                    0.0
                };

                ui.label(RichText::new("Physics:").color(Color32::from_rgb(100, 180, 255)).strong());
                ui.add_space(2.0);
                
                ui.horizontal(|ui| {
                    ui.label(RichText::new("  Total KE:").color(Color32::LIGHT_GRAY).size(11.0));
                    ui.label(RichText::new(format!("{:.2e} J", total_ke)).size(11.0).monospace());
                }).response.on_hover_text("Total kinetic energy of the system");
                
                ui.horizontal(|ui| {
                    ui.label(RichText::new("  Total Mass:").color(Color32::LIGHT_GRAY).size(11.0));
                    ui.label(RichText::new(format!("{:.2e} kg", total_mass)).size(11.0).monospace());
                }).response.on_hover_text("Sum of all body masses");
                
                ui.horizontal(|ui| {
                    ui.label(RichText::new("  Avg Speed:").color(Color32::LIGHT_GRAY).size(11.0));
                    ui.label(RichText::new(format!("{:.2} m/s", avg_velocity)).size(11.0).monospace());
                }).response.on_hover_text("Average velocity magnitude");

                // Center of mass
                if !simulation.owned_objects.is_empty() {
                    let mut com = crate::utils::vector::Vector(0.0, 0.0, 0.0);
                    for body in &simulation.owned_objects {
                        com = com + body.position() * body.mass;
                    }
                    com = com / total_mass;
                    
                    ui.add_space(4.0);
                    ui.horizontal(|ui| {
                        ui.label(RichText::new("  COM:").color(Color32::LIGHT_GRAY).size(11.0));
                        ui.label(RichText::new(format!("({:.1}, {:.1}, {:.1})", com.0, com.1, com.2)).size(11.0).monospace());
                    }).response.on_hover_text("Center of mass coordinates");
                }
            });
    }

    /// Render celestial bodies list
    fn render_body_list(
        ui: &mut egui::Ui,
        simulation: &GalaxySimulation,
        hover_system: &HoverSystem,
    ) {
        ui.heading(RichText::new("🪐 Celestial Bodies").color(Color32::WHITE));
        ui.add_space(4.0);

        // Bodies list with its own scroll area (nested scroll for long lists)
        egui::ScrollArea::vertical()
            .max_height(280.0) // Increased height
            .auto_shrink([false; 2])
            .show(ui, |ui| {
                for (idx, body) in simulation.owned_objects.iter().enumerate() {
                    let is_hovered = Some(idx) == hover_system.hovered_index;

                    egui::Frame::none()
                        .fill(if is_hovered {
                            Color32::from_rgb(50, 50, 60)
                        } else {
                            Color32::from_rgb(35, 35, 40)
                        })
                        .inner_margin(12.0)
                        .rounding(4.0)
                        .stroke(if is_hovered {
                            Stroke::new(2.0, Color32::from_rgb(255, 220, 100))
                        } else {
                            Stroke::NONE
                        })
                        .show(ui, |ui| {
                            let color = Self::get_body_color(idx);

                            ui.horizontal(|ui| {
                                ui.label(RichText::new("●").color(color).size(16.0));
                                ui.label(
                                    RichText::new(&simulation.object_names[idx])
                                        .strong()
                                        .size(16.0),
                                );
                                if is_hovered {
                                    ui.label(RichText::new("🎯").size(14.0));
                                }
                            });

                            ui.add_space(6.0);
                            
                            // Use grid layout for better alignment
                            egui::Grid::new(format!("body_grid_{}", idx))
                                .num_columns(1)
                                .spacing([10.0, 5.0])
                                .show(ui, |ui| {
                                    ui.label(RichText::new("Mass:").color(Color32::GRAY).size(13.0));
                                    ui.label(RichText::new(format!("{:.2e} kg", body.mass)).size(13.0));
                                    ui.end_row();
                                    
                                    ui.label(RichText::new("Speed:").color(Color32::GRAY).size(13.0));
                                    ui.label(RichText::new(format!("{:.2} m/s", body.velocity.magnitude())).size(13.0));
                                    ui.end_row();
                                    
                                    ui.label(RichText::new("Radius:").color(Color32::GRAY).size(13.0));
                                    ui.label(RichText::new(format!("{:.2} m", body.r)).size(13.0));
                                    ui.end_row();

                                    if idx > 0 {
                                        let distance = body.position().magnitude();
                                        ui.label(RichText::new("Distance:").color(Color32::GRAY).size(13.0));
                                        ui.label(RichText::new(format!("{:.2} m", distance)).size(13.0));
                                        ui.end_row();
                                    }
                                });
                        });
                    ui.add_space(6.0); // Increased spacing between bodies
                }
            });
    }

    /// Render debug tools section
    fn render_debug_tools(
        ui: &mut egui::Ui,
        hover_system: &HoverSystem,
        debug_viz: &mut DebugVisualization,
    ) -> bool {
        ui.heading(RichText::new("🔧 Debug Tools").color(Color32::WHITE));
        ui.add_space(4.0);

        let mut shoot_ray = false;

        egui::Frame::none()
            .fill(Color32::from_rgb(30, 30, 35))
            .inner_margin(12.0)
            .rounding(4.0)
            .show(ui, |ui| {
                // Ray Visualization
                ui.label(
                    RichText::new("Ray Casting")
                        .color(Color32::from_rgb(100, 180, 255))
                        .size(12.0)
                        .strong(),
                );
                ui.add_space(4.0);

                if ui.button(
                    RichText::new("🎯 Shoot Debug Ray")
                        .color(Color32::WHITE)
                        .size(13.0)
                ).on_hover_text("Fire a raycast from camera through mouse cursor").clicked() {
                    shoot_ray = true;
                }

                ui.add_space(2.0);
                
                let ray_status = if hover_system.show_debug_ray {
                    RichText::new("✓ Ray Visible").color(Color32::from_rgb(0, 255, 100))
                } else {
                    RichText::new("○ Ray Hidden").color(Color32::GRAY)
                };
                ui.label(ray_status.size(11.0));

                if let Some(ray) = &hover_system.debug_ray {
                    ui.add_space(6.0);
                    ui.separator();
                    ui.add_space(4.0);
                    ui.label(
                        RichText::new("Ray Info:")
                            .color(Color32::LIGHT_GRAY)
                            .size(10.0),
                    );
                    ui.label(
                        RichText::new(format!(
                            "Origin: ({:.1}, {:.1}, {:.1})",
                            ray.origin.0, ray.origin.1, ray.origin.2
                        ))
                        .color(Color32::LIGHT_GRAY)
                        .size(9.0)
                        .monospace(),
                    );
                    ui.label(
                        RichText::new(format!(
                            "Direction: ({:.2}, {:.2}, {:.2})",
                            ray.direction.0, ray.direction.1, ray.direction.2
                        ))
                        .color(Color32::LIGHT_GRAY)
                        .size(9.0)
                        .monospace(),
                    );
                }

                ui.add_space(12.0);
                ui.separator();
                ui.add_space(8.0);

                // Vector Visualization
                ui.label(
                    RichText::new("Vector Visualization")
                        .color(Color32::from_rgb(100, 180, 255))
                        .size(12.0)
                        .strong(),
                );
                ui.add_space(6.0);

                ui.checkbox(&mut debug_viz.show_velocity_vectors, 
                    RichText::new("🏹 Velocity Vectors").color(Color32::WHITE))
                    .on_hover_text("Show velocity direction and magnitude for each body (green)");

                ui.checkbox(&mut debug_viz.show_acceleration_vectors, 
                    RichText::new("⚡ Acceleration Vectors").color(Color32::WHITE))
                    .on_hover_text("Show gravitational acceleration for each body (red)");

                ui.add_space(8.0);
                ui.separator();
                ui.add_space(8.0);

                // Orbital Visualization
                ui.label(
                    RichText::new("Orbital Paths")
                        .color(Color32::from_rgb(100, 180, 255))
                        .size(12.0)
                        .strong(),
                );
                ui.add_space(6.0);

                ui.checkbox(&mut debug_viz.show_orbital_trails, 
                    RichText::new("🌀 Orbital Trails").color(Color32::WHITE))
                    .on_hover_text("Show predicted circular orbital paths");

                ui.checkbox(&mut debug_viz.show_grid, 
                    RichText::new("📐 Grid Floor").color(Color32::WHITE))
                    .on_hover_text("Toggle grid floor visibility");
            });

        shoot_ray
    }

    /// Render controls help section
    fn render_controls_help(ui: &mut egui::Ui) {
        ui.heading(RichText::new("Controls").color(Color32::WHITE));
        ui.add_space(4.0);

        egui::Frame::none()
            .fill(Color32::from_rgb(30, 30, 35))
            .inner_margin(8.0)
            .rounding(4.0)
            .show(ui, |ui| {
                ui.label("WASD - Move camera");
                ui.label("Mouse - Look around");
                ui.label("SPACE - Pause/Play");
                ui.label("↑/↓ - Time scale");
                ui.label("R - Reverse time");
                ui.add_space(4.0);
                ui.label(
                    RichText::new("💡 Hover over planets to highlight")
                        .color(Color32::from_rgb(150, 150, 255))
                        .size(11.0),
                );
            });
    }

    /// Get color for a body based on its index
    pub fn get_body_color(idx: usize) -> Color32 {
        match idx {
            0 => Color32::YELLOW,
            1 => Color32::from_rgb(100, 200, 255),
            2 => Color32::from_rgb(255, 120, 100),
            _ => Color32::from_rgb(200, 180, 150),
        }
    }
}
