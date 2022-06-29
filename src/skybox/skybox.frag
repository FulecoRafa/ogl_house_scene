#version 330
in vec3 ReflectDir;
out vec4 color;

uniform samplerCube cubetex;

void main() {
    color = texture(cubetex, ReflectDir);
}