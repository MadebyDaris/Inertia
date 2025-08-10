# Inertia User Documentation

## Table of Contents
1. [Getting Started](#getting-started)
2. [Basic Usage](#basic-usage)
3. [Creating Custom Simulations](#creating-custom-simulations)
4. [Physics Configuration](#physics-configuration)
5. [Visual Customization](#visual-customization)
6. [Advanced Features](#advanced-features)
7. [Troubleshooting](#troubleshooting)

## Getting Started

### System Requirements
- **Operating System**: Windows 10+, macOS 10.15+, or Linux (Ubuntu 18.04+)
- **Graphics**: OpenGL 3.3+ compatible GPU
- **Memory**: 4GB RAM minimum (8GB recommended)
- **Storage**: 100MB free space

### Installation Guide

#### Method 1: Binary Release (Recommended for Users)
1. Download the latest release from the [releases page](https://github.com/MadebyDaris/Inertia/releases)
2. Extract the archive to your desired location
3. Run the executable:
   - **Windows**: Double-click `Inertia.exe` or run `./Inertia.exe` in terminal
   - **macOS/Linux**: Run `./Inertia` in terminal

#### Method 2: Build from Source (For Developers)
```bash
# Prerequisites: Install Rust from https://rustup.rs/
git clone https://github.com/MadebyDaris/Inertia.git
cd Inertia
cargo build --release
cargo run --release
```

## Basic Usage

### Understanding the Interface

When you first launch Inertia, you'll see:

1. **3D Viewport**: Main simulation area showing celestial bodies
2. **Information Panels**: Real-time physics data for each body
3. **Simulation Controls**: Time and physics parameters

### Navigation Controls

| Input | Action | Description |
|-------|--------|-------------|
| `W` | Move Forward | Camera moves in the direction you're looking |
| `S` | Move Backward | Camera moves opposite to view direction |
| `A` | Strafe Left | Camera moves left relative to view direction |
| `D` | Strafe Right | Camera moves right relative to view direction |
| `↑` | Move Up | Camera rises vertically |
| `↓` | Move Down | Camera descends vertically |
| `Mouse Movement` | Look Around | First-person camera rotation |
| `ESC` | Exit | Close the application |

### Reading the UI Panels

Each celestial body has an information panel displaying:

- **Mass**: Object's mass in kilograms
- **Position**: Current 3D coordinates (x, y, z)
- **Velocity**: Current velocity vector in m/s
- **Angular Velocity**: Rotational speed in rad/s
- **Acceleration**: Current acceleration due to forces
- **Moment of Inertia**: Resistance to rotational changes
- **Euler Angles**: Current orientation (pitch, yaw, roll)
- **Radius**: Physical size of the object

## Creating Custom Simulations

### Basic Simulation Setup

Create a new simulation by modifying `src/simulation/example.rs`:

```rust
use crate::mesh::sphere::SphereConstructor;
use crate::utils::vector::Vector;

// Create a sphere constructor
let sphere_constructor = SphereConstructor {
    radius: 2.0,        // Object radius
    longitude: 32,      // Sphere detail (horizontal)
    latitude: 16        // Sphere detail (vertical)
};

// Create a celestial body
let mut central_star = sphere_constructor.sphere_physics_object(
    Vector(0.0, 0.0, 0.0),  // Initial velocity (vx, vy, vz)
    1000.0,                 // Mass in kg
    &display,
    star_shader             // Visual appearance
);

// Position the object in space
central_star.mesh.translate(0.0, 0.0, 0.0);  // (x, y, z) position
central_star.mesh.scale(3.0, 3.0, 3.0);      // Scale factor
```

### Multi-Body Systems

Create complex systems with multiple interacting bodies:

```rust
// Central massive body (star)
let mut star = sphere_constructor.sphere_physics_object(
    Vector(0.0, 0.0, 0.0), 1000.0, &display, star_shader
);

// Orbiting body (planet)
let mut planet = sphere_constructor.sphere_physics_object(
    Vector(0.0, 0.0, 8.0), 10.0, &display, planet_shader
);
planet.mesh.translate(50.0, 0.0, 0.0);

// Moon orbiting the planet
let mut moon = sphere_constructor.sphere_physics_object(
    Vector(0.0, 0.0, 12.0), 1.0, &display, moon_shader
);
moon.mesh.translate(60.0, 0.0, 0.0);
```

### Setting Up Forces and Physics

In your simulation loop, apply gravitational forces:

```rust
// Apply gravitational forces between all bodies
calculate_g_forces!(star, &planet, &moon);
calculate_g_forces!(planet, &star, &moon);
calculate_g_forces!(moon, &star, &planet);

// Update physics for each body
update_astral_body_physics!(star, delta_time);
update_astral_body_physics!(planet, delta_time);
update_astral_body_physics!(moon, delta_time);
```

## Physics Configuration

### Gravitational Constant

Modify the gravitational constant to change the strength of gravitational attraction:

```rust
const G: f32 = 5.0;  // Default value
// Increase for stronger gravity: const G: f32 = 10.0;
// Decrease for weaker gravity: const G: f32 = 1.0;
```

### Damping Forces

Add realistic energy dissipation to simulate orbital decay:

```rust
// Apply damping to a specific body
let damping = planet.damping_force(0.01);  // Damping coefficient
planet.add_force(damping);
```

### Time Step Control

Adjust simulation accuracy vs. performance:

```rust
// In your physics update loop
let delta_time = now.duration_since(last_frame).as_secs_f32();

// For higher accuracy (slower):
let delta_time = delta_time * 0.5;

// For faster simulation (less accurate):
let delta_time = delta_time * 2.0;
```

### Custom Forces

Add your own force implementations:

```rust
impl AstralBody {
    pub fn custom_force(&self, other: &AstralBody) -> Force {
        // Your custom force calculation
        let direction = other.position() - self.position();
        Force {
            direction: direction.normalized(),
            magnitude: 100.0  // Your force magnitude
        }
    }
}
```

## Visual Customization

### Textures and Shaders

Create custom visual appearances:

```rust
let custom_shader = ShaderData {
    tex_filename: "./data/tex/your_texture.jpg".to_string(),
    vertex_shader: "data/glsl/vertex_shader.glsl".to_string(),
    fragment_shader: "data/glsl/fragment_shader.glsl".to_string(),
};
```

### Supported Texture Formats
- JPEG (.jpg, .jpeg)
- PNG (.png)
- BMP (.bmp)
- TIFF (.tiff)

### Lighting Configuration

Adjust the lighting system:

```rust
let light = DiffuseLight {
    u_light_color: (1.0, 1.0, 1.0),      // RGB color (0.0-1.0)
    u_light_direction: (-1.0, -1.0, -1.0) // Direction vector
};
```

### Camera Settings

Customize camera behavior:

```rust
let mut camera = Camera::new(&display);
// In Camera::new(), modify these values:
// translation_sensitivity: 0.05,  // Movement speed
// rotation_sensitivity: 0.005,    // Mouse sensitivity
```

## Advanced Features

### Orbital Prediction

Implement trajectory prediction:

```rust
// Calculate future positions (simplified example)
fn predict_orbit(&self, time_steps: usize, dt: f32) -> Vec<Vector> {
    let mut positions = Vec::new();
    let mut temp_body = self.clone();
    
    for _ in 0..time_steps {
        // Apply forces and update position
        temp_body.law_of_momentum();
        temp_body.update_velocity(dt);
        temp_body.update_geometry(dt);
        positions.push(temp_body.position());
    }
    
    positions
}
```

### Performance Optimization

For large simulations:

```rust
// Use spatial partitioning for O(n log n) instead of O(n²)
// Implement Barnes-Hut algorithm for distant body approximation
// Reduce sphere resolution for distant objects
let low_detail_sphere = SphereConstructor {
    radius: 2.0,
    longitude: 16,  // Reduced from 32
    latitude: 8     // Reduced from 16
};
```

### Data Export

Save simulation data for analysis:

```rust
use std::fs::File;
use std::io::Write;

fn export_trajectory(&self, filename: &str) {
    let mut file = File::create(filename).unwrap();
    writeln!(file, "time,x,y,z,vx,vy,vz").unwrap();
    
    // Write position and velocity data
    writeln!(file, "{},{},{},{},{},{},{}", 
        time, pos.0, pos.1, pos.2, vel.0, vel.1, vel.2).unwrap();
}
```

## Troubleshooting

### Common Issues

#### Performance Problems
- **Solution**: Reduce sphere resolution or limit number of bodies
- **Check**: GPU compatibility and driver updates
- **Monitor**: Frame rate in window title

#### Visual Artifacts
- **Cause**: Usually texture loading issues or shader compilation errors
- **Solution**: Verify texture file paths and formats
- **Debug**: Check console output for OpenGL errors

#### Physics Instability
- **Symptoms**: Bodies flying apart or oscillating wildly
- **Solutions**:
  - Reduce time step: `delta_time * 0.1`
  - Check initial conditions (avoid bodies too close together)
  - Verify mass and gravitational constant values

#### Camera Controls Not Working
- **Check**: Window has focus
- **Verify**: Event handling is properly implemented
- **Solution**: Restart application or check input device

### Debug Mode

Enable additional debugging information:

```rust
// Add debug prints in physics loop
println!("Body position: {:?}", body.position());
println!("Applied forces: {:?}", body.forces.len());
println!("Acceleration: {:?}", body.acceleration);
```

### Performance Monitoring

Monitor simulation performance:

```rust
use std::time::Instant;

let frame_start = Instant::now();
// ... simulation code ...
let frame_time = frame_start.elapsed();
println!("Frame time: {:.2}ms", frame_time.as_millis());
```

### Getting Help

- **Documentation**: Check the API documentation with `cargo doc --open`
- **Community**: Join discussions on GitHub Issues
- **Examples**: Study the provided example simulations
- **Physics References**: Consult orbital mechanics textbooks for theoretical background

## Best Practices

1. **Start Simple**: Begin with 2-3 body systems before creating complex scenarios
2. **Verify Physics**: Compare results with known orbital mechanics solutions
3. **Optimize Gradually**: Profile performance before optimizing
4. **Document Changes**: Keep track of parameter modifications
5. **Version Control**: Use git to save working configurations
6. **Test Thoroughly**: Verify stability over extended simulation times

---

Ready to create your own cosmic simulations? Start with the basic examples and gradually build complexity as you become familiar with the system! 🚀