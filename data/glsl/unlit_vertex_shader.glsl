#version 330 core
in vec3 position;
in vec2 tex_coords;

uniform mat4 model;
uniform mat4 view;
uniform mat4 perspective;

out vec2 v_tex_coords;

void main() {
    gl_Position = perspective * view * model * vec4(position, 1.0);
    v_tex_coords = tex_coords;
}
