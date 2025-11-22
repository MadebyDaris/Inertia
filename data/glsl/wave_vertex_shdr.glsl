#version 330 core
layout (location = 0) in vec3 position; // The flat grid positions
layout (location = 1) in vec2 tex_coords;

uniform sampler2D u_psi_texture; // Your simulation result
uniform float u_height_scale;    // How high the mountains are
uniform mat4 u_model;
uniform mat4 u_view;
uniform mat4 u_projection;

out vec2 vTexCoord;
out float vPhase; // Pass phase to fragment shader for color!
out float vProbability; // Pass probability for brightness
out vec3 vWorldPos; // World position for lighting

void main() {
    vTexCoord = tex_coords;
    
    // 1. Sample the complex number
    vec2 psi = texture(u_psi_texture, tex_coords).rg;
    
    // 2. Calculate Displacement (Let's use Probability Density)
    float probability = dot(psi, psi); // Real^2 + Imag^2
    vProbability = probability;
    
    // 3. Displace the Y position upward for wave peaks
    vec3 newPos = position;
    newPos.y -= probability * u_height_scale;
    
    // 4. Calculate Phase (Angle) for coloring later
    // atan(y, x) gives the angle in radians (-PI to PI)
    vPhase = atan(psi.y, psi.x); 

    // Transform to world space
    vec4 worldPos = u_model * vec4(newPos, 1.0);
    vWorldPos = worldPos.xyz;

    gl_Position = u_projection * u_view * worldPos;
}