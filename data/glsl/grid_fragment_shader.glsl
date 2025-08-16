#version 330 core

in vec2 v_tex_coords;
in vec3 v_normal;
in vec3 v_position;

out vec4 color;

uniform vec3 u_light_direction;
uniform vec3 u_light_color;

void main() {
    // Create procedural grid lines
    vec2 grid = abs(fract(v_tex_coords * 20.0) - 0.5) / fwidth(v_tex_coords * 20.0);
    float line = min(grid.x, grid.y);
    
    // Grid color - bright lines on dark background
    vec3 grid_color = vec3(0.1, 0.15, 0.2); // Dark background
    vec3 line_color = vec3(0.3, 0.4, 0.6); // Light blue lines
    
    // Make grid lines thinner and more subtle
    float grid_strength = 1.0 - min(line, 1.0);
    grid_strength = smoothstep(0.0, 0.1, grid_strength);
    
    vec3 final_color = mix(grid_color, line_color, grid_strength * 0.8);
    
    // Simple lighting
    float light_intensity = max(dot(normalize(v_normal), normalize(-u_light_direction)), 0.3);
    final_color *= light_intensity;
    
    // Make it semi-transparent so it doesn't interfere with objects above
    color = vec4(final_color, 0.6);
}