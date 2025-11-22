# Quantum Simulation Visualization System

## Overview
A GPU-accelerated quantum mechanics simulation with 3D wavefunction visualization using high-resolution mesh displacement.

## Architecture

### 1. **QuantumGrid** (`quantum_grid.rs`)
Manages GPU textures for quantum state:
- **Wavefunction Textures** (RG32F): Store complex numbers (Real + Imaginary)
  - `wave_packet`: Current state
  - `wave_packet_mem`: Temporary buffer for ping-pong rendering
- **Potential Texture** (R32F): Energy landscape V(x,y) for barriers/slits
- **Probability Density** (R32F): |ψ|² for quick visualization

### 2. **QuantumMesh** (`quantum_mesh.rs`)
High-resolution 3D mesh for visualization:
- **Vertex Grid**: Each vertex maps 1:1 with a simulation texel
  - For 256×256 simulation → 256×256 vertices (65,536 total)
  - Position: Flat grid in XZ plane
  - Tex Coords: Map to simulation texture
- **Index Buffer**: Triangulates the grid (2 triangles per cell)
- **Shader Integration**: Loads wave visualization shaders

### 3. **QuantumObject** (`quantum_object.rs`)
High-level rendering interface:
- Wraps QuantumMesh with transform and parameters
- Height scale: Controls vertical displacement amount
- Transform: Position/rotation/scale in world space
- Visibility toggle

### 4. **QuantumSimulation** (`quantum_sim.rs`)
Main simulation coordinator:
- Manages QuantumGrid and QuantumObject
- Loads compute shaders (evolution, probability)
- Time evolution parameters (ℏ, mass, dt, dx)
- Integration with engine's Simulation trait

## Shader Pipeline

### Compute Shaders (GLSL 450)

#### **evolution.glsl**
Solves time-dependent Schrödinger equation:
```
iℏ ∂ψ/∂t = Ĥψ = (-ℏ²/2m ∇² + V)ψ
```
- **Input**: Current wavefunction, potential
- **Output**: Next wavefunction state
- **Method**: Finite difference (5-point stencil for Laplacian)
- **Integration**: Euler method (requires small dt for stability)

#### **probability.glsl**
Calculates probability density:
```
P(x,y) = |ψ(x,y)|² = Real² + Imaginary²
```
- **Input**: Wavefunction texture (RG channels)
- **Output**: Probability density (R channel)

### Visualization Shaders (GLSL 330)

#### **wave_vertex_shdr.glsl**
Vertex displacement:
1. Samples wavefunction texture at vertex tex coords
2. Calculates |ψ|² for displacement
3. Displaces Y position: `y += |ψ|² * height_scale`
4. Computes phase angle: `atan(imaginary, real)`
5. Transforms to clip space

#### **wave_fragment_shdr.glsl**
Phase-based coloring:
1. Receives phase angle from vertex shader
2. Maps phase [-π, π] to hue [0, 1]
3. Converts HSV to RGB
4. Outputs colored fragment

## Usage Example

```rust
// 1. Create simulation (256×256 resolution)
let mut quantum_sim = QuantumSimulation::new(&display, 256);

// 2. Initialize Gaussian wave packet
quantum_sim.setup_gaussian_wave_packet(
    &display,
    -0.5,  // Center X
    0.0,   // Center Y  
    0.1,   // Sigma (width)
    5.0,   // Momentum X
    0.0,   // Momentum Y
);

// 3. Add double-slit barrier
quantum_sim.setup_double_slit(
    &display,
    0.0,     // Barrier X position
    0.05,    // Thickness
    0.15,    // Slit width
    0.4,     // Slit separation
    1000.0,  // Potential height
);

// 4. Configure visualization
quantum_sim.quantum_object.set_height_scale(2.0);

// 5. In render loop
quantum_sim.update(delta_time);
quantum_sim.render(&display, &mut frame, view_matrix, proj_matrix);
```

## Performance Considerations

### Vertex Count vs Resolution
| Resolution | Vertices | Triangles | Use Case |
|------------|----------|-----------|----------|
| 64×64      | 4,096    | 7,938     | Testing, mobile |
| 128×128    | 16,384   | 32,258    | Medium quality |
| 256×256    | 65,536   | 130,050   | **Recommended** |
| 512×512    | 262,144  | 522,242   | High-end GPUs |

### Optimization Tips
1. **Match mesh resolution to texture**: Avoid mismatch artifacts
2. **LOD system**: Use lower resolution mesh when far from camera
3. **Frustum culling**: Don't render off-screen portions
4. **Ping-pong buffers**: Efficient GPU memory usage
5. **Small time steps**: dt ≤ 0.0001 for numerical stability

## Visualization Features

### Height Displacement
- Vertical displacement based on probability density
- Controllable via `height_scale` parameter
- Creates "mountain" effect where particle is likely to be

### Phase Coloring
- Hue represents phase angle of complex wavefunction
- Red/Orange: Phase ≈ 0
- Yellow/Green: Phase ≈ π/2
- Blue/Purple: Phase ≈ π
- Shows wave interference patterns

### What You'll See
1. **Gaussian packet**: Smooth "hill" moving across grid
2. **At barrier**: Wave splits at slits
3. **After barrier**: Interference pattern forms
4. **Colors**: Rainbow fringes show phase relationships

## Technical Notes

### Texture Formats
- **GL_RG32F**: High precision complex numbers (2×32-bit float)
- **GL_R32F**: Single channel for potential/probability
- **MipmapsOption::NoMipmap**: Simulation data shouldn't be filtered

### Coordinate Systems
- **Normalized [-1, 1]**: Used for physics calculations
- **UV [0, 1]**: Texture coordinates
- **World space**: Arbitrary positioning via transform

### Compute Shader Limitation
Glium 0.36 doesn't natively support compute shaders. For production:
- Use `glium::backend::glutin` with raw OpenGL calls
- Or migrate to `wgpu` for full compute support
- Current code has placeholders for future compute integration

## Future Enhancements
1. **Interactive potential editing**: Draw barriers in real-time
2. **Multiple particles**: Entanglement visualization
3. **3D simulation**: Extend to volumetric rendering
4. **Performance profiling**: GPU timing queries
5. **Export**: Save wavefunction states/animations
