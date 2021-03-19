#shader vertex
#version 330 core


layout (location = 0) in vec3 aPos;
layout (location = 1) in vec3 aNormal;
layout (location = 2) in vec2 aTexCoords;

out vec2 uv;
out vec3 normal;
out vec3 worldPos;


uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;

void main()
{
    uv = aTexCoords;
    normal = (model * vec4(aNormal, 1.0)).xyz;
    worldPos = (model * vec4(aPos, 1.0)).xyz;
    // gl_Position = projection * view * model * vec4(aPos, 1.0);
}

#include "shaders/tessellation.glsl"

#shader geometry
#version 330 core
layout (triangles) in;
layout (triangle_strip, max_vertices = 4) out;

void build_house(vec4 position)
{    
    gl_Position = position + vec4(-0.2, -0.2, -0.01, 0.0);    // 1:bottom-left
    EmitVertex();   
    gl_Position = position + vec4( 0.2, -0.2, -0.01, 0.0);    // 2:bottom-right
    EmitVertex();
    gl_Position = position + vec4(-0.2,  0.2, -0.01, 0.0);    // 3:top-left
    EmitVertex();
    gl_Position = position + vec4( 0.2,  0.2, -0.01, 0.0);    // 4:top-right
    EmitVertex();
    EndPrimitive();
}

void main() {    
    build_house(gl_in[0].gl_Position);
    build_house(gl_in[1].gl_Position);
    build_house(gl_in[2].gl_Position);
}  

#shader fragment
#version 330 core
out vec4 FragColor;

void main()
{
	  FragColor = vec4(1.0, 1.0, 1.0, 1.0);
}