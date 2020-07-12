#version 330 core
layout (location = 0) in vec3 aPos;
layout (location = 1) in vec3 aNormal;
layout (location = 2) in vec2 aTexCoords;

uniform mat4 model;


out vec2 TexCoord_CS_in;
out vec3 WorldPos_CS_in;
out vec3 Normal_CS_in;

void main()
{
    TexCoord_CS_in = aTexCoords;
    WorldPos_CS_in =(model * vec4(aPos, 1.0)).xyz;
    vec3 worldCoords = (model * vec4(aNormal, 0.0)).xyz;
    Normal_CS_in = worldCoords;
}
