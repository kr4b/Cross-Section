#version 330 core
in vec2 position;
in vec2 tex_coord;

out vec2 f_tex_coord;

uniform mat4 view;
uniform mat4 perspective;
uniform mat4 scale;
uniform mat4 transform;

void main() {
    gl_Position = perspective * view * transform * scale * vec4(position, 0.0, 1.0);
    f_tex_coord = tex_coord;
}