#version 450
layout(local_size_x = 16, local_size_y = 16) in;

// Input: Complex wavefunction (RG32F)
layout(binding = 0) uniform sampler2D u_wavefunction;

// Output: RGBA8 image
layout(binding = 1, rgba8) uniform image2D u_output;

void main() {
    ivec2 pixel = ivec2(gl_GlobalInvocationID.xy);
    ivec2 size = imageSize(u_output);
    
    if (pixel.x >= size.x || pixel.y >= size.y) return;
    
    vec2 uv = (vec2(pixel) + 0.5) / vec2(size);
    
    // Read complex wavefunction (Real, Imaginary)
    vec2 psi = texture(u_wavefunction, uv).rg;
    
    // Calculate amplitude |ψ| using same method as vertex shader
    float probability = dot(psi, psi);
    
    // Use logarithmic scale to see both low and high amplitude regions
    // This compresses the dynamic range so peaks are distinguishable
    float log_prob = log(1.0 + probability * 100.0);
    float intensity = log_prob / log(101.0);
    intensity = clamp(intensity, 0.0, 1.0);
    
    // Color map: blue (low) -> cyan -> green -> yellow -> red (high)
    vec3 color;
    if (intensity < 0.25) {
        // Blue to Cyan
        float t = intensity / 0.25;
        color = mix(vec3(0.0, 0.0, 1.0), vec3(0.0, 1.0, 1.0), t);
    } else if (intensity < 0.5) {
        // Cyan to Green
        float t = (intensity - 0.25) / 0.25;
        color = mix(vec3(0.0, 1.0, 1.0), vec3(0.0, 1.0, 0.0), t);
    } else if (intensity < 0.75) {
        // Green to Yellow
        float t = (intensity - 0.5) / 0.25;
        color = mix(vec3(0.0, 1.0, 0.0), vec3(1.0, 1.0, 0.0), t);
    } else {
        // Yellow to Red
        float t = (intensity - 0.75) / 0.25;
        color = mix(vec3(1.0, 1.0, 0.0), vec3(1.0, 0.0, 0.0), t);
    }
    
    imageStore(u_output, pixel, vec4(color, 1.0));
}
