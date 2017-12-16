#version 140

uniform mat4 projection_matrix;
uniform mat4 view_matrix;
uniform float total_blocks;

in mat4 matrix;
in vec3 position;
in vec2 uv;
in int id;

out vec2 fragment_uv;

void main() {
	float theX = uv.x / total_blocks;
    fragment_uv = vec2(theX + ((id - 1) / total_blocks), uv.y);
    gl_Position = projection_matrix * view_matrix * matrix * vec4(position, 1.0);
}