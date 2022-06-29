#version 330
in vec3 position;
out vec3 ReflectDir;

uniform mat4 view;
uniform mat4 perspective;

void main() {
    ReflectDir = position;
    vec4 matrix = /*perspective **/ view * vec4(position, 1.0);
    gl_Position = vec4(matrix.xy, 1.0, 1.0);
}