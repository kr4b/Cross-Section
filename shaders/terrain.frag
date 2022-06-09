#version 330 core
out vec4 frag_color;

in float brightness;
in vec3 f_tex_coord;

uniform sampler3D volume;

void main() {
    frag_color = vec4(brightness * texture(volume, f_tex_coord).xyz, 1.0);
}