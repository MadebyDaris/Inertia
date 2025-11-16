#version 330 core

in vec3 v_position;
in vec3 v_normal;
in vec2 v_tex_coords;

uniform sampler2D tex;
uniform vec3 u_light_direction;
uniform vec3 u_light_color;
uniform vec3 material_color;
uniform float ambient_occlusion;

out vec4 frag_color;

void main() {
    vec3 ambient = 0.1 * u_light_color;
    
    vec3 norm = normalize(v_normal);
    float diff = max(dot(norm, -u_light_direction), 0.0);
    vec3 diffuse = diff * u_light_color;
    
    vec4 tex_color = texture(tex, v_tex_coords);
    vec3 result = (ambient + diffuse) * tex_color.rgb * material_color * ambient_occlusion;
    
    frag_color = vec4(result, tex_color.a);
}
