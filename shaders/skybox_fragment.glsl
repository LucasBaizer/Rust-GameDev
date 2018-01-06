#version 140

uniform samplerCube cubemap;

in vec3 fragment_uv;
out vec4 color;

void main() {
    color = texture(cubemap, fragment_uv);
}