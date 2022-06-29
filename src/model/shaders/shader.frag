#version 330

uniform vec3 light;
uniform sampler2D tex;

//in vec2 v_tex_coords;
//in vec3 v_normal;
//out vec4 frag_texture;
//out vec4 tex_color;
out vec4 color;

void main() {
//    float brightness = dot(normalize(v_normal), normalize(light));
//    tex_color = texture(tex, v_tex_coords);
//    vec3 darkest = 0.4 * tex_color.rgb;
//    vec3 lightest = 1.0 * tex_color.rgb;
//    frag_texture = vec4(mix(darkest, lightest, brightness), 1.0);
    color = vec4(1.0, 0.0, 0.0, 1.0);
}

