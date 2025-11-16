#version 330 core

in vec2 v_tex_coords;

uniform sampler2D tex;
uniform vec3 material_color;

out vec4 frag_color;

void main() {
    vec4 tex_color = texture(tex, v_tex_coords);
    frag_color = vec4(tex_color.rgb * material_color, tex_color.a);
}
