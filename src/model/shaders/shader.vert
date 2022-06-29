#version 330

in vec3 position;
in vec2 tex_coords;
in vec3 normal;
out vec2 v_tex_coords;
out vec3 v_normal;

uniform mat4 translation;
uniform mat4 rotation;
uniform mat4 scale;
uniform mat4 self_rotation;
uniform mat4 view;

void main() {
    v_tex_coords = tex_coords;
    v_normal = transpose(inverse(mat3(rotation)))
        * transpose(inverse(mat3(translation)))
        * transpose(inverse(mat3(scale)))
        * transpose(inverse(mat3(self_rotation)))
        * normal;
    // Operations occur from right to left
    gl_Position =
    view *
    rotation *
    translation *
    scale *
    self_rotation *
    vec4(position, 1.0);
}
