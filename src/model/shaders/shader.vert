#version 330

in vec3 position, normal;
in vec2 tex_coords;

out vec3 v_normal, v_position;
out vec2 v_tex_coords;

uniform mat4 translation, rotation, scale, self_rotation, view, perspective;

void main() {
    v_tex_coords = tex_coords;

    // Operations occur from right to left
    mat4 matrix =
    perspective *
    view *
    rotation *
    translation *
    scale *
    self_rotation;

    gl_Position =
    matrix *
    vec4(position, 1.0);

    v_normal =
    inverse(transpose(mat3(matrix))) *
    normal;

    v_position = gl_Position.xyz / gl_Position.w;
}
