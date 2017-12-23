#version 140

uniform mat4 projection_matrix;
uniform mat4 view_matrix;
uniform float total_blocks;

in mat4 matrix;
in vec3 position;
in vec2 uv;
in int face;
in int id;

out vec2 fragment_uv;

void main() {
	//float theX = (uv.x / total_blocks ) + ((id - 1) / total_blocks);
	//float theY = 1.0 - (1.0 - uv.y + face) / 6.0;

	float p = 2.0 / 16.0;
	float theX = float(id) / total_blocks + ((uv.x * (1 - 2 * p) + p) / total_blocks);
	float theY = face / 6.0 + ((uv.y * (1 - 2 * p) + p) / 6.0);

    fragment_uv = vec2(theX, theY);
    gl_Position = projection_matrix * view_matrix * matrix * vec4(position, 1.0);
}