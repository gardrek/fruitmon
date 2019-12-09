#version 330

in vec3 position;
in vec3 normal;

out vec3 v_color;

out vec3 v_normal;

uniform mat4 matrix;
uniform mat4 matrix_2;

void main() {
    v_normal = transpose(inverse(mat3(matrix))) * normal;
    v_color = normal; // set vertex color based on position for testing purposes
    gl_Position = matrix * vec4(position, 1.0);
}
