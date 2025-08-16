#version 330 core

in vec3 position;
in vec3 normal;
in vec2 tex_coords;

out vec2 v_tex_coords;
out vec3 v_normal;
out vec3 v_position;

uniform mat4 perspective;
uniform mat4 view;
uniform mat4 model;

void main() {
    v_tex_coords = tex_coords;
    v_normal = normalize((model * vec4(normal, 0.0)).xyz);
    v_position = (model * vec4(position, 1.0)).xyz;
    
    gl_Position = perspective * view * model * vec4(position, 1.0);
}