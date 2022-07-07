#version 330

uniform vec3 light;
uniform mat4 light_rotation;
uniform sampler2D tex;

in vec2 v_tex_coords;
in vec3 v_normal;
in vec3 v_position;
out vec4 color;

vec3 ambient_color = vec3(texture(tex, v_tex_coords)) * 0.45;
vec3 diffuse_color = ambient_color * 1.55;
vec3 specular_color = ambient_color * 4.0;

void main() {
    vec3 ulight = vec3(light_rotation * vec4(light, 1.0));
    float diffuse = max(dot(normalize(v_normal), normalize(ulight)), 0.0);
    vec3 camera_dir = normalize(-v_position);
    vec3 half_direction = normalize(normalize(ulight) + camera_dir);
    float specular = pow(max(dot(half_direction, normalize(v_normal)), 0.0), 16.0);
    color = vec4(ambient_color + diffuse * diffuse_color + specular * specular_color, 1.0);
}

