#version 140

uniform mat4 view_matrix;
uniform mat4 projection_matrix;

in vec3 position;
out vec3 fragment_uv;

void main() {
	gl_Position = projection_matrix * view_matrix * vec4(position * 200.0, 1.0);
    fragment_uv = position;
}
