#version 330 core
in vec3 position;
in vec3 normal;
in vec2 tex_coords;

uniform mat4 model;
uniform mat4 view;
uniform mat4 perspective;

out vec3 v_position;
out vec3 v_normal;
out vec2 v_tex_coords;

void main() {
    vec4 world_pos = model * vec4(position, 1.0);
    gl_Position = perspective * view * world_pos;
    
    v_position = world_pos.xyz;
    v_normal = mat3(transpose(inverse(model))) * normal;
    v_tex_coords = tex_coords;
}
