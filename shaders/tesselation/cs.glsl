#version 410 core

// define the number of CPs in the output patch
layout (vertices = 3) out;

// attributes of the input CPs
in vec3 WorldPos_CS_in[];
in vec2 TexCoord_CS_in[];
in vec3 Normal_CS_in[];

// attributes of the output CPs
out vec3 WorldPos_ES_in[];
out vec2 TexCoord_ES_in[];
out vec3 Normal_ES_in[];


void main()
{
    // Set the control points of the output patch
    TexCoord_ES_in[gl_InvocationID] = TexCoord_CS_in[gl_InvocationID];
    Normal_ES_in[gl_InvocationID] = Normal_CS_in[gl_InvocationID];
    WorldPos_ES_in[gl_InvocationID] = WorldPos_CS_in[gl_InvocationID];

    // Calculate the tessellation levels
    gl_TessLevelOuter[0] = 16;
    gl_TessLevelOuter[1] = 16;
    gl_TessLevelOuter[2] = 16;
    gl_TessLevelInner[0] = 16;
}