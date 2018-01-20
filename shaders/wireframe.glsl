#version 140

uniform mat4 projection_matrix;
uniform mat4 view_matrix;
uniform vec3 cube_position;

in vec3 position;

void main() {
    gl_Position = projection_matrix * view_matrix * vec4(cube_position + position, 1.0);
}
