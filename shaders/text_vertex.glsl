#version 140

uniform mat4 transform_matrix;
uniform mat4 projection_matrix;
uniform int character;

in vec2 position;
in vec2 uv;
out vec2 fragment_uv;

void main() {
    float theX = (uv.x / 10.0 ) + ((character - 1) / 10.0);
	float theY = 1.0 - (1.0 - uv.y + face) / 6.0;

    fragment_uv = vec2(theX, theY);
    gl_Position = projection_matrix * transform_matrix * vec4(position, -1.0, 1.0);
}
