#version 140

in vec2 fragment_uv;
out vec4 color;

uniform sampler2D sampler;
uniform float value;
uniform vec2 screen_size;

void main() {
    color = texture(sampler, fragment_uv);
}