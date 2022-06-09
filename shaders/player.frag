#version 330 core
out vec4 frag_color;

in vec2 f_tex_coord;
uniform sampler2D tex;

void main() {
    frag_color = vec4(texture(tex, f_tex_coord));
}