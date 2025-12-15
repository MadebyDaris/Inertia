#version 450

layout(local_size_x = 16, local_size_y = 16) in;

// --- BINDINGS ---
layout(binding = 0) uniform sampler2D u_current_psi;
layout(binding = 1) uniform sampler2D u_potential;
layout(binding = 2, rg32f) uniform image2D u_next_psi;

// --- UNIFORMS ---
uniform float u_dt;
uniform float u_dx;
uniform float u_hbar;
uniform float u_mass;

// 0 = Update Real Part (using Imaginary)
// 1 = Update Imaginary Part (using Real)
uniform int u_pass; 

// Helper to calculate H * psi_component
// Returns: (-h^2/2m * laplacian + V * val)
float apply_hamiltonian(float val, float laplacian, float V) {
    float kinetic_coeff = (u_hbar * u_hbar) / (2.0 * u_mass);
    float kinetic = -kinetic_coeff * laplacian;
    float potential = V * val;
    return kinetic + potential;
}

void main() {
    ivec2 pixel = ivec2(gl_GlobalInvocationID.xy);
    ivec2 size = imageSize(u_next_psi);

    if (pixel.x >= size.x || pixel.y >= size.y) return;

    vec2 uv = (vec2(pixel) + 0.5) / vec2(size);
    vec2 stride = 1.0 / vec2(size);

    // Load current full state
    vec4 current_psi = texture(u_current_psi, uv);
    float psi_real = current_psi.r;
    float psi_imag = current_psi.g;
    float V = texture(u_potential, uv).r;

    vec2 result_psi = vec2(psi_real, psi_imag);

    // --- LEAPFROG LOGIC ---
    // The equations are:
    // d(Real)/dt =  (1/h) * H * Imag
    // d(Imag)/dt = -(1/h) * H * Real
    
    // We only update ONE component per dispatch to ensure neighbor consistency.
    
    if (u_pass == 0) {
        // --- PASS 1: Update Real (using Imaginary) ---
        // We need the Laplacian of the IMAGINARY part
        float val_C = psi_imag;
        float val_L = texture(u_current_psi, uv + vec2(-stride.x, 0.0)).g;
        float val_R = texture(u_current_psi, uv + vec2( stride.x, 0.0)).g;
        float val_D = texture(u_current_psi, uv + vec2(0.0, -stride.y)).g;
        float val_U = texture(u_current_psi, uv + vec2(0.0,  stride.y)).g;
        
        float laplacian_imag = (val_L + val_R + val_D + val_U - 4.0 * val_C) / (u_dx * u_dx);

        // H * psi_imag
        float H_imag = apply_hamiltonian(psi_imag, laplacian_imag, V);

        // Real_new = Real_old + dt/hbar * (H * Imag)
        result_psi.x = psi_real + (u_dt / u_hbar) * H_imag;
        
        // Green channel (Imag) remains unchanged this pass
        result_psi.y = psi_imag;

    } else {
        // --- PASS 2: Update Imaginary (using Real) ---
        // We need the Laplacian of the REAL part
        float val_C = psi_real;
        float val_L = texture(u_current_psi, uv + vec2(-stride.x, 0.0)).r;
        float val_R = texture(u_current_psi, uv + vec2( stride.x, 0.0)).r;
        float val_D = texture(u_current_psi, uv + vec2(0.0, -stride.y)).r;
        float val_U = texture(u_current_psi, uv + vec2(0.0,  stride.y)).r;

        float laplacian_real = (val_L + val_R + val_D + val_U - 4.0 * val_C) / (u_dx * u_dx);

        // H * psi_real
        float H_real = apply_hamiltonian(psi_real, laplacian_real, V);

        // Imag_new = Imag_old - dt/hbar * (H * Real)
        result_psi.y = psi_imag - (u_dt / u_hbar) * H_real;
        
        // Red channel (Real) remains unchanged this pass
        result_psi.x = psi_real;
    }

    imageStore(u_next_psi, pixel, vec4(result_psi, 0.0, 0.0));
}