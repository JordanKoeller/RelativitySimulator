#version 410 core
#include "test_resources/import_shader.fs"
uniform mat4 view;
uniform mat4 projection;
vec3 interpolate3D(vec3 v0, vec3 v1, vec3 v2)
{
return vec3(gl_TessCoord.x) * v0 + vec3(gl_TessCoord.y) * v1 + vec3(gl_TessCoord.z) * v2;
}