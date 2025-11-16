#version 330 core

in vec2 v_tex_coords;

uniform sampler2D tex;
uniform vec3 material_color;
uniform float emission_strength;

out vec4 frag_color;

void main() {
    vec4 tex_color = texture(tex, v_tex_coords);
    vec3 emission = tex_color.rgb * material_color * emission_strength;
    frag_color = vec4(emission, tex_color.a);
}
