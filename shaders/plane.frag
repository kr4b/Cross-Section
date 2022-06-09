#version 330 core

#define THICKNESS 2

out vec4 frag_color;

in vec2 f_tex_coord;
uniform sampler2D frame;
uniform float height;
uniform float width;

void main() {
    vec2 f = min(f_tex_coord, 1.0 - f_tex_coord);

    if (min(f.x, f.y) < 0.01) {
        frag_color = vec4(1.0);
    } else {
        float c1 = texture(frame, f_tex_coord).r;
        for (int i = 1; i <= THICKNESS; i++) {
            float dx = float(i) / width;
            float dy = float(i) / height;
            float c2 = texture(frame, f_tex_coord + vec2(dx, 0.0)).r;
            float c3 = texture(frame, f_tex_coord + vec2(-dx, 0.0)).r;
            float c4 = texture(frame, f_tex_coord + vec2(0.0, dy)).r;
            float c5 = texture(frame, f_tex_coord + vec2(0.0, -dy)).r;
            if (c1 != c2 || c1 != c3 || c1 != c4 || c1 != c4) {
                frag_color = vec4(1.0);
                return;
            }
        }

        frag_color = vec4(0.0);
    }
}