#version 330

in vec3 position, normal;
in vec2 tex_coords;

out vec3 v_normal;
out vec2 v_tex_coords;

uniform mat4 translation, rotation, scale, self_rotation, view, perspective;

void main() {
    v_tex_coords = tex_coords;
    v_normal = transpose(inverse(mat3(rotation)))
        * transpose(inverse(mat3(translation)))
        * transpose(inverse(mat3(scale)))
        * transpose(inverse(mat3(self_rotation)))
        * normal;
    // Operations occur from right to left
    gl_Position =
    perspective *
    view *
    rotation *
    translation *
    scale *
    self_rotation *
    vec4(position, 1.0);
}
