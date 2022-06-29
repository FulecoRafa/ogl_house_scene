#version 330

uniform vec3 light;
uniform sampler2D tex;

in vec3 v_normal;
out vec4 color;

void main() {
    float brightness = dot(normalize(v_normal), normalize(light));
    vec3 dark_color = vec3(0.6, 0.0, 0.0);
    vec3 regular_color = vec3(1.0, 0.0, 0.0);
    color = vec4(mix(dark_color, regular_color, brightness), 1.0);
}

