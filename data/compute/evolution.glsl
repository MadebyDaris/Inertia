#version 450

// Workgroup size (adjust based on your GPU, usually 8x8 or 16x16)
layout(local_size_x = 16, local_size_y = 16) in;


// BINDINGS
// The CURRENT state (Read-Only)
layout(binding = 0) uniform sampler2D u_current_psi;

// The Potential map
layout(binding = 1) uniform sampler2D u_potential;

// Binding 2: The NEXT state (Write-Only)
layout(binding = 2, rg32f) uniform image2D u_next_psi;


// CONSTANTS
uniform float u_dt;
uniform float u_dx;
uniform float u_dy;
uniform float u_hbar;
uniform float u_mass;

void main() {
    // Get integer coordinates for the current pixel
    ivec2 pixel_coords = ivec2(gl_GlobalInvocationID.xy);
    
    // texture dimensions
    ivec2 size = imageSize(u_next_psi);
    if (pixel_coords.x >= size.x || pixel_coords.y >= size.y) return;

    // Normalized UV coordinates for sampling
    vec2 uv = (vec2(pixel_coords) + 0.5) / vec2(size);
    vec2 texel_size = 1.0 / vec2(size);

    // Finite Difference Method - estimation of the laplace operator (look into notes)
    vec2 psi_C = texture(u_current_psi, uv).rg; // Center
    vec2 psi_L = texture(u_current_psi, uv + vec2(-texel_size.x, 0.0)).rg;
    vec2 psi_R = texture(u_current_psi, uv + vec2( texel_size.x, 0.0)).rg;
    vec2 psi_D = texture(u_current_psi, uv + vec2(0.0, -texel_size.y)).rg;
    vec2 psi_U = texture(u_current_psi, uv + vec2(0.0,  texel_size.y)).rg;

    vec2 laplacian_x = (psi_L + psi_R - 2.0 * psi_C) / (u_dx * u_dx);
    vec2 laplacian_y = (psi_D + psi_U - 2.0 * psi_C) / (u_dy * u_dy);
    vec2 laplacian = laplacian_x + laplacian_y;


// HAMILTONIAN APPLICATION (Ĥψ)
    // Get Potential V at this point (stored in Red channel)
    float V = texture(u_potential, uv).r;

    // Kinetic energy term
    float kinetic_coeff = (u_hbar * u_hbar) / (2.0 * u_mass);
    vec2 kinetic_term = -kinetic_coeff * laplacian;
    
    // Potential energy
    vec2 potential_term = V * psi_C;
    

    vec2 H_psi = kinetic_term + potential_term;


// SCHRODINGER EQUATION

    float inv_hbar = 1.0 / u_hbar;
    vec2 next_psi;
    next_psi.x = psi_C.x + ( H_psi.y * inv_hbar) * u_dt; // Real part update
    next_psi.y = psi_C.y + (-H_psi.x * inv_hbar) * u_dt; // Imag part update

    // Apply edge dampening to prevent boundary reflections
    // UV is [0,1], so center is 0.5
    float dist_from_center = distance(uv, vec2(0.5));
    // Only dampen very near the edges (starts at 95% of radius, fully dampened at edge)
    float dampening = smoothstep(0.5, 0.475, dist_from_center);
    next_psi *= dampening;

    // Write to the output image
    imageStore(u_next_psi, pixel_coords, vec4(next_psi, 0.0, 0.0));
}