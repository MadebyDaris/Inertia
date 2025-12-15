#version 450
layout(local_size_x = 16, local_size_y = 16) in;

layout(binding = 0) uniform sampler2D u_wavefunction;

// Output
layout(binding = 1, rgba8) uniform image2D u_output;

// Smooth HSV to RGB conversion
vec3 hsv2rgb(vec3 c) {
    vec4 K = vec4(1.0, 2.0 / 3.0, 1.0 / 3.0, 3.0);
    vec3 p = abs(fract(c.xxx + K.xyz) * 6.0 - K.www);
    return c.z * mix(K.xxx, clamp(p - K.xxx, 0.0, 1.0), c.y);
}

void main() {
    ivec2 pixel = ivec2(gl_GlobalInvocationID.xy);
    ivec2 size = imageSize(u_output);
    
    if (pixel.x >= size.x || pixel.y >= size.y) return;
    
    vec2 uv = (vec2(pixel) + 0.5) / vec2(size);
    
    // complex wavefunction
    vec2 psi = texture(u_wavefunction, uv).rg;
    
    // Calculate probability density
    float prob = dot(psi, psi);
    
    float phase = atan(psi.y, psi.x); // Range: -π to +π
    
    // Convert to HSV color
    float hue = (phase + 3.14159) / (2.0 * 3.14159); // Normalize to 0-1
    float saturation = 0.8;
    float value = sqrt(prob) * 2.0; // Brightness based on probability
    
    vec3 color = hsv2rgb(vec3(hue, saturation, value));
    
    imageStore(u_output, pixel, vec4(color, 1.0));
}