#version 450
layout(local_size_x = 16, local_size_y = 16) in;

layout(binding = 0) uniform sampler2D u_psi_texture;
// We write to a single channel float texture (R32F) or standard RGBA
layout(binding = 1, r32f) uniform image2D u_prob_map; 

void main() {
    ivec2 coords = ivec2(gl_GlobalInvocationID.xy);
    vec2 size = vec2(imageSize(u_prob_map));
    if (coords.x >= int(size.x) || coords.y >= int(size.y)) return;

    vec2 uv = (vec2(coords) + 0.5) / size;

    // Read Wavefunction
    vec2 psi = texture(u_psi_texture, uv).rg;

    // Calculate Probability: Real^2 + Imag^2
    float probability = dot(psi, psi);

    // Write to output
    imageStore(u_prob_map, coords, vec4(probability, 0.0, 0.0, 1.0));
}