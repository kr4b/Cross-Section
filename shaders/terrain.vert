#version 330 core
in vec3 position;
in vec3 normal;
in vec3 tex_coord;

uniform mat4 perspective;
uniform mat4 view;

out float brightness;
out vec3 f_tex_coord;

void main() {
    gl_Position = perspective * view * vec4(position, 1.0);
    brightness = clamp(0.0, 1.0, dot(normal, normalize(vec3(-1.0, 4.0, -1.0))));
    f_tex_coord = tex_coord;
}