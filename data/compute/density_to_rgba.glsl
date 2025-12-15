#version 450
layout(local_size_x = 16, local_size_y = 16) in;

// Input RG32F
layout(binding = 0) uniform sampler2D u_wavefunction;

// Output RGBA8 image
layout(binding = 1, rgba8) uniform image2D u_output;

void main() {
    ivec2 pixel = ivec2(gl_GlobalInvocationID.xy);
    ivec2 size = imageSize(u_output);
    
    if (pixel.x >= size.x || pixel.y >= size.y) return;
    
    vec2 uv = (vec2(pixel) + 0.5) / vec2(size);
    
    vec2 psi = texture(u_wavefunction, uv).rg;
    
    // Calculate probability density |ψ|²
    float prob = dot(psi, psi);
    
    // Scale for visibility
    float intensity = prob * 50.0;
    intensity = clamp(intensity, 0.0, 1.0);
    
    vec3 gray = vec3(intensity);
    
    imageStore(u_output, pixel, vec4(gray, 1.0));
}
