#version 140

in vec2 fragment_uv;

out vec4 color;

uniform sampler2D sampler;

void main() {
    color = texture(sampler, fragment_uv);
}