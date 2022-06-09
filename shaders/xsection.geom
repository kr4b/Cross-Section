#version 330 core

#define EPSILON 10e-6

layout(lines) in;
layout(triangle_strip, max_vertices = 4) out;

in vec3 g_tex_coord[];

out vec3 f_tex_coord;

uniform mat4 projection;
uniform mat4 transform;

struct Vertex {
    vec3 position;
    vec3 tex_coord;
};

void main() {
    vec3 vertices[4] = vec3[4](
        gl_in[0].gl_Position.xyz,
        vec3(gl_in[0].gl_Position.xy, gl_in[1].gl_Position.z),
        vec3(gl_in[1].gl_Position.x, gl_in[0].gl_Position.yz),
        vec3(gl_in[1].gl_Position.x, gl_in[0].gl_Position.y, gl_in[1].gl_Position.z)
    );
    vec3 tex_coords[4] = vec3[4](
        g_tex_coord[0],
        vec3(g_tex_coord[0].xy, g_tex_coord[1].z),
        vec3(g_tex_coord[1].x, g_tex_coord[0].yz),
        vec3(g_tex_coord[1].x, g_tex_coord[0].y, g_tex_coord[1].z)
    );
    int edges[8] = int[8](
        0, 1,
        0, 2,
        1, 3,
        2, 3
    );

    float y_pos = (transform * vec4(0.0, gl_in[1].gl_Position.y, 0.0, 1.0)).y;
    float y_tex = g_tex_coord[1].y;

    Vertex intersections[4];
    int j = 0;

    for (int i = 0; i < 4; i++) {
        vertices[i] = (transform * vec4(vertices[i], 1.0)).xyz;
        if (abs(vertices[i].z) < EPSILON) {
            intersections[j].position = vertices[i];
            intersections[j].position.z = 0.0;
            intersections[j].tex_coord = tex_coords[i];
            j += 1;
        }
    }

    for (int i = 0; i < 4; i++) {
        if (j == 2) {
            break;
        }

        int e0 = edges[i * 2 + 0];
        int e1 = edges[i * 2 + 1];
        vec3 v0 = vertices[e0];
        vec3 v1 = vertices[e1];

        if ((v0.z < EPSILON && v1.z < EPSILON) || (v0.z > -EPSILON && v1.z > -EPSILON)) {
            continue;
        }

        float t = 0.0;
        float d = v0.z - v1.z;
        if (abs(d) > EPSILON) {
            t = v0.z / d;
        }

        vec3 t0 = tex_coords[e0];
        vec3 t1 = tex_coords[e1];
        intersections[j].position = mix(v0, v1, t);
        intersections[j].position.z = 0.0;
        intersections[j].tex_coord = mix(t0, t1, t);

        j += 1;
    }

    if (j == 2) {
        for (int i = 0; i < 2; i++) {
            vec3 position = intersections[i].position;
            intersections[i + 2].position = (projection * vec4(position.x, y_pos, 0.0, 1.0)).xyz;
            vec3 tex_coord = intersections[i].tex_coord;
            intersections[i + 2].tex_coord = vec3(tex_coord.x, y_tex, tex_coord.z);
            intersections[i].position = (projection * vec4(position, 1.0)).xyz;
        }

        gl_Position = vec4(intersections[0].position, 1.0);
        f_tex_coord = intersections[0].tex_coord;
        EmitVertex();
        gl_Position = vec4(intersections[1].position, 1.0);
        f_tex_coord = intersections[1].tex_coord;
        EmitVertex();
        gl_Position = vec4(intersections[2].position, 1.0);
        f_tex_coord = intersections[2].tex_coord;
        EmitVertex();
        gl_Position = vec4(intersections[3].position, 1.0);
        f_tex_coord = intersections[3].tex_coord;
        EmitVertex();
        EndPrimitive();
    }
}