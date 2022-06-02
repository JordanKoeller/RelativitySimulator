#shader vertex
#version 330 core
layout (location = 0) in vec3 aPos;
layout (location = 1) in vec3 aNormal;

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;

out VS_OUT {
    vec4 normal;
} vs_out;

void main()
{
    vs_out.normal = (model * vec4(aNormal, 1.0));
    gl_Position = (model * vec4(aPos, 1.0));
}

#shader geometry
#version 330 core
layout (triangles) in;
layout (line_strip, max_vertices = 2) out;

uniform float debug_line_length;

in VS_OUT {
    vec4 normal;
} gs_in[];

out vec4 normal;

void main() {
    gl_Position = gl_in[0].gl_Position;
    normal = gs_in[0].normal;
    EmitVertex();

    gl_Position = gl_in[0].gl_Position + vec4(gs_in[0].normal.xyz * debug_line_length, 1.0);
    normal = gs_in[0].normal;
    EmitVertex();
    EndPrimitive();
}

#shader fragment
#version 330 core

in vec4 normal;

out vec4 FragColor;

void main() {
    FragColor = vec4(1.0, 1.0, 1.0, 1.0);
}