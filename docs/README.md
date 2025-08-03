# Project Inertia
**A Real-Time Physics Engine**

Inertia is a high-performance, Rust-based physics engine built with OpenGL that simulates gravitational interactions between celestial bodies in real-time in this fork but more coming soon. Designed for educational exploration and visualization of orbital mechanics, this engine provides an interactive 3D sandbox environment where users can experiment with astronomical phenomena and observe the elegant dance of celestial objects under Newton's laws of motion.

![Space Simulation](https://img.shields.io/badge/Physics-Orbital%20Mechanics-blue)
![Language](https://img.shields.io/badge/Language-Rust-orange)
![Graphics](https://img.shields.io/badge/Graphics-OpenGL-green)

## Features

### Current Capabilities
- **N-Body Gravitational Simulation**: Accurate gravitational force calculations between multiple celestial bodies
- **Real-Time Physics**: Smooth integration using Euler's method with customizable time steps
- **3D Visualization**: OpenGL-powered rendering with textured spherical bodies
- **Interactive Camera**: First-person camera with smooth movement and mouse look controls
- **Live Debugging UI**: Real-time display of physics parameters including:
  - Position, velocity, and acceleration vectors
  - Angular momentum and rotational dynamics
  - Force magnitudes and directions
  - Orbital characteristics
- **Procedural Sphere Generation**: Customizable resolution spheres with proper UV mapping
- **Lighting System**: Diffuse lighting model for realistic celestial illumination
- **Damping Forces**: Configurable energy dissipation for realistic orbital decay

### Educational Focus
Perfect for:
- **Students** learning orbital mechanics and physics
- **Educators** demonstrating gravitational concepts
- **Developers** interested in physics simulation and game engine architecture
- **Astronomy enthusiasts** exploring celestial dynamics

## Architecture

Inertia is built with a modular design that separates concerns across several key components:

```
Inertia Engine
├── Simulation Layer      # Physics world management
├── Physics Engine        # Force calculations & integration  
├── Rendering System      # OpenGL graphics pipeline
├── Mesh Management       # 3D geometry and texturing
├── UI System            # Real-time parameter visualization
└── Utilities            # Math, vectors, and app framework
```

### Core Systems

**Physics Engine** (`src/physics/`)
- **AstralBody**: Complete celestial object with mass, velocity, acceleration, and rotational dynamics
- **Force Calculations**: Universal gravitation, damping forces, and custom force applications
- **Integration**: Numerical methods for updating position and velocity over time

**Rendering Pipeline** (`src/render/`)
- **Camera System**: 3D navigation with pitch/yaw controls and perspective projection
- **Mesh Rendering**: Efficient vertex buffer management and texture mapping
- **Lighting**: Configurable diffuse lighting for realistic appearance

**User Interface** (`src/utils/ui.rs`)
- **Real-time Monitoring**: Live physics parameter display
- **Interactive Controls**: Camera movement and simulation parameters
- **Educational Widgets**: Clear visualization of complex physics concepts

## Quick Start

### Prerequisites
- Rust 1.70+ with Cargo
- OpenGL 3.3+ compatible graphics driver
- Modern GPU (integrated graphics sufficient for basic simulations)

### Installation

1. **Clone the repository**
   ```bash
   git clone https://github.com/MadebyDaris/Inertia.git
   cd Inertia
   ```

2. **Build the project**
   ```bash
   cargo build --release
   ```

3. **Run the simulation**
   ```bash
   cargo run --release
   ```

### First Steps
1. **Observe the Default System**: The engine starts with a 3-body system (Earth-like planet with two moons)
2. **Navigate**: Use WASD keys to move, mouse to look around
3. **Monitor Physics**: Check the UI panels for real-time physics data
4. **Experiment**: Modify parameters in `src/simulation/example.rs` to create different scenarios

## Controls

| Control | Action |
|---------|--------|
| `W/A/S/D` | Move camera forward/left/backward/right |
| `Arrow Up/Down` | Move camera up/down |
| `Mouse` | Look around (first-person view) |
| `ESC` | Exit simulation |

## Customization

### Creating New Scenarios

```rust
// Example: Create a custom celestial body
let mut planet = sphere_constructor.sphere_physics_object(
    Vector(0.0, 0.0, 0.0),  // Initial velocity
    100.0,                   // Mass
    &display, 
    earth_shader
);
planet.mesh.translate(50.0, 0.0, 0.0);  // Position in space
```

### Adjustable Parameters
- **Gravitational Constant**: Modify `G` in physics calculations
- **Time Step**: Adjust `delta_time` for simulation speed/accuracy
- **Damping Coefficient**: Control orbital decay rates
- **Camera Sensitivity**: Customize movement and look speeds
- **Sphere Resolution**: Balance visual quality vs. performance

## Technical Details

### Dependencies
- **glium**: Modern OpenGL wrapper for Rust
- **nalgebra**: Linear algebra and matrix operations
- **egui**: Immediate-mode GUI for debugging interface
- **image**: Texture loading and processing
- **winit**: Cross-platform window management

### Performance Considerations
- Optimized for real-time simulation (60+ FPS)
- Efficient O(n²) gravitational calculations
- GPU-accelerated rendering pipeline
- Memory-efficient mesh management

## Educational Applications

### Physics Concepts Demonstrated
- **Newton's Law of Universal Gravitation**: F = G(m₁m₂)/r²
- **Newton's Second Law**: F = ma
- **Conservation of Energy**: Kinetic + potential energy
- **Angular Momentum**: Rotational dynamics and precession
- **Orbital Mechanics**: Elliptical orbits, escape velocity, tidal forces

### Suggested Experiments
1. **Binary Star Systems**: Create two massive bodies orbiting each other
2. **Slingshot Maneuvers**: Launch a small body past a large one
3. **Three-Body Problems**: Explore chaotic orbital dynamics
4. **Tidal Locking**: Observe rotational synchronization
5. **Orbital Decay**: Study the effects of damping forces

## Future Development

We're continuously expanding Inertia's capabilities. See our [roadmap](docs/ROADMAP.md) for upcoming features including:

- **Advanced Integration Methods** (Runge-Kutta, Verlet)
- **Collision Detection & Merging**
- **Procedural Planet Generation**
- **Realistic Texturing & Atmospheres** 
- **Time Acceleration Controls**
- **Orbital Prediction Visualization**
- **Binary/Multiple Star Systems**
- **Spacecraft Trajectory Planning**

## Contributing

We welcome contributions! Whether you're fixing bugs, adding features, or improving documentation:

1. Fork the repository
2. Create a feature branch: `git checkout -b feature/amazing-feature`
3. Commit your changes: `git commit -m 'Add amazing feature'`
4. Push to the branch: `git push origin feature/amazing-feature`
5. Open a Pull Request

### Development Setup
```bash
cargo fmt      # Format code
cargo clippy   # Lint code
cargo test     # Run tests
cargo doc --open  # Generate and view documentation
```

## Resources

- **Documentation**: [API Reference](docs/)
- **Examples**: [Tutorial Scenarios](examples/)
- **Physics Background**: [Orbital Mechanics Primer](docs/physics-primer.md)
- **OpenGL Learning**: [LearnOpenGL](https://learnopengl.com/)

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- The Rust community for excellent graphics and math libraries
- OpenGL contributors for the robust graphics standard
- Physics educators and textbooks that inspired accurate implementations
- The open-source community for continuous feedback and improvements

---

**Ready to explore the cosmos?**