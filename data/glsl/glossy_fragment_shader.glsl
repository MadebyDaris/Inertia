#version 330 core

in vec3 v_position;
in vec3 v_normal;
in vec2 v_tex_coords;
in vec3 v_view_dir;

uniform sampler2D tex;
uniform vec3 u_light_direction;
uniform vec3 u_light_color;
uniform vec3 material_color;
uniform float specular_intensity;
uniform float shininess;
uniform float ambient_occlusion;

out vec4 frag_color;

void main() {
    vec3 ambient = 0.1 * u_light_color;
    
    vec3 norm = normalize(v_normal);
    vec3 light_dir = normalize(-u_light_direction);
    
    // Diffuse
    float diff = max(dot(norm, light_dir), 0.0);
    vec3 diffuse = diff * u_light_color;
    
    // Specular (Blinn-Phong)
    vec3 halfway_dir = normalize(light_dir + v_view_dir);
    float spec = pow(max(dot(norm, halfway_dir), 0.0), shininess);
    vec3 specular = specular_intensity * spec * u_light_color;
    
    vec4 tex_color = texture(tex, v_tex_coords);
    vec3 result = (ambient + diffuse + specular) * tex_color.rgb * material_color * ambient_occlusion;
    
    frag_color = vec4(result, tex_color.a);
}
