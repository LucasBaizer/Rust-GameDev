#version 140

in vec2 position;
in vec2 uv;
out vec2 fragment_uv;

void main() {
    fragment_uv = uv;
    gl_Position = vec4(position, 1.0, 1.0);
}
