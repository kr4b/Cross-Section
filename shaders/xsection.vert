#version 330 core
in vec3 position;
in vec3 tex_coord;

out vec3 g_tex_coord;

void main() {
    gl_Position = vec4(position, 1.0);
    g_tex_coord = tex_coord;
}