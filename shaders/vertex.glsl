#version 140

uniform mat4 projection_matrix;
uniform mat4 view_matrix;

in mat4 matrix;
in vec3 position;
in vec2 uv;
in int id;
out vec2 fragment_uv;

void main() {
    fragment_uv = uv;
    gl_Position = projection_matrix * view_matrix * matrix * vec4(position, 1.0);
}