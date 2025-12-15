#version 450

layout(local_size_x = 16, local_size_y = 16) in;

// --- BINDINGS ---
layout(binding = 0) uniform sampler2D u_current_psi;
layout(binding = 1) uniform sampler2D u_potential;
layout(binding = 2, rg32f) uniform image2D u_next_psi;

// --- UNIFORMS ---
uniform float u_dt;
uniform float u_dx;
uniform float u_dy; // Assuming dy == dx usually, but kept for consistency
uniform float u_hbar;
uniform float u_mass;

// --- DERIVATIVE FUNCTION ---
// Calculates: dpsi/dt = -i/hbar * ( -h^2/2m * laplacian + V * psi )
vec2 compute_derivative(vec2 psi_val, vec2 kinetic_term, float V, float hbar) {
    // Potential Term: V * psi
    vec2 potential_term = V * psi_val;

    // Hamiltonian: H = K + V
    // Note: kinetic_term is pre-calculated based on the initial state
    // because we cannot recalculate the Laplacian for intermediate RK steps in a single pass.
    vec2 H_psi = kinetic_term + potential_term;

    // Schrödinger Equation: dpsi/dt = -i/hbar * H_psi
    // Complex multiplication by -i: (x + iy) * -i = (y - ix)
    return vec2(H_psi.y, -H_psi.x) / hbar;
}

void main() {
    ivec2 pixel = ivec2(gl_GlobalInvocationID.xy);
    ivec2 size = imageSize(u_next_psi);

    if (pixel.x >= size.x || pixel.y >= size.y) return;

    vec2 uv = (vec2(pixel) + 0.5) / vec2(size);
    vec2 stride = 1.0 / vec2(size);

    // 1. Sample Current State & Calculate Laplacian
    vec2 psi_C = texture(u_current_psi, uv).rg;
    vec2 psi_L = texture(u_current_psi, uv + vec2(-stride.x, 0.0)).rg;
    vec2 psi_R = texture(u_current_psi, uv + vec2( stride.x, 0.0)).rg;
    vec2 psi_D = texture(u_current_psi, uv + vec2(0.0, -stride.y)).rg;
    vec2 psi_U = texture(u_current_psi, uv + vec2(0.0,  stride.y)).rg;

    // Standard 5-point stencil Laplacian
    vec2 laplacian = (psi_L + psi_R + psi_D + psi_U - 4.0 * psi_C) / (u_dx * u_dx);

    // 2. Pre-calculate Environment Constants
    float V = texture(u_potential, uv).r;
    float kinetic_coeff = (u_hbar * u_hbar) / (2.0 * u_mass);
    
    // The kinetic contribution is treated as constant over the timestep 
    // due to the single-pass limitation.
    vec2 kinetic_term = -kinetic_coeff * laplacian;

    // 3. Runge-Kutta 4 Integration
    // k1 = f(t, y)
    vec2 k1 = compute_derivative(psi_C, kinetic_term, V, u_hbar);

    // k2 = f(t + dt/2, y + dt*k1/2)
    vec2 k2 = compute_derivative(psi_C + k1 * u_dt * 0.5, kinetic_term, V, u_hbar);

    // k3 = f(t + dt/2, y + dt*k2/2)
    vec2 k3 = compute_derivative(psi_C + k2 * u_dt * 0.5, kinetic_term, V, u_hbar);

    // k4 = f(t + dt, y + dt*k3)
    vec2 k4 = compute_derivative(psi_C + k3 * u_dt, kinetic_term, V, u_hbar);

    // 4. Final Combination
    // y_new = y + (dt/6) * (k1 + 2k2 + 2k3 + k4)
    vec2 next_psi = psi_C + (u_dt / 6.0) * (k1 + 2.0 * k2 + 2.0 * k3 + k4);

    // Texture Clamp mode prevents wrapping, but allows quantum tunneling through finite barriers
    // No infinite boundary - let the potential texture define all barriers
    
    imageStore(u_next_psi, pixel, vec4(next_psi, 0.0, 0.0));
}