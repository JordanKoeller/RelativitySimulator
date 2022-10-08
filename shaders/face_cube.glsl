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
    normal = (vec4(aNormal, 1.0)).xyz;
    worldPos = (model * vec4(aPos, 1.0)).xyz;
    // gl_Position = projection * view * model * vec4(aPos, 1.0);
}

#shader tesscontrol
#version 410 core

// define the number of CPs in the output patch
layout (vertices = 3) out;

#include "shaders/lorentz_helper.glsl"

// attributes of the input CPs
in vec3 worldPos[];
in vec2 uv[];
in vec3 normal[];

// attributes of the output CPs
out vec3 newWorldPos[];
out vec2 newUv[];
out vec3 newNormal[];


uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;

float tess_level(vec3 v1, vec3 v2) {
  // I want to know curvature. That's what decides how fine of a tessellation is needed.
  // The easiest way I can think of is to compute the midpt of each side
  // and transform all into a relativistic frame.

  // Then I compute the angle from the midpt to the two end segments to approx. The
  // curvature. The larger the angle, the larger the degree of tessellation.
  if (lorentzFlag == 2 && beta > 0.01) {
    mat4 screenspace = projection * view;
    vec3 midpt = (v1 + v2) / 2.0;
    vec2 relV1 =  (screenspace * vec4(timeTransform(lorentzTransform(v1)), 1.0)).xy;
    vec2 relV2 =  (screenspace * vec4(timeTransform(lorentzTransform(v2)), 1.0)).xy;
    vec2 relMid = (screenspace * vec4(timeTransform(lorentzTransform(midpt)), 1.0)).xy;
    float angle = dot(normalize(relV2 - relMid), normalize(relMid - relV1));
    return max((1.0 - abs(angle)) * 50.0, 1.0);
  } else {
    return 1.0;
  }
}

void main()
{
    // Set the control points of the output patch
    newUv[gl_InvocationID] = uv[gl_InvocationID];
    newNormal[gl_InvocationID] = normal[gl_InvocationID];
    newWorldPos[gl_InvocationID] = worldPos[gl_InvocationID];

    // Calculate the tessellation levels
    // gl_TessLevelOuter[0] = tess_level(newWorldPos[0], newWorldPos[1]);
    // gl_TessLevelOuter[1] = tess_level(newWorldPos[1], newWorldPos[2]);
    // gl_TessLevelOuter[2] = tess_level(newWorldPos[2], newWorldPos[0]);
    gl_TessLevelOuter[0] = 10;
    gl_TessLevelOuter[1] = 10;
    gl_TessLevelOuter[2] = 10;
    gl_TessLevelInner[0] = (gl_TessLevelOuter[0] + gl_TessLevelOuter[1] + gl_TessLevelOuter[2]) / 3.0;
}



#shader tesseval
#version 410 core

layout(triangles, equal_spacing, ccw) in;

#include "shaders/lorentz_helper.glsl"


uniform mat4 view;
uniform mat4 projection;

in vec3 newWorldPos[];
in vec2 newUv[];
in vec3 newNormal[];

out vec3 finalWorldPos;
out vec2 finalUv;
out vec3 finalNormal;

vec2 interpolate2D(vec2 v0, vec2 v1, vec2 v2)
{
    return vec2(gl_TessCoord.x) * v0 + vec2(gl_TessCoord.y) * v1 + vec2(gl_TessCoord.z) * v2;
}

vec3 interpolate3D(vec3 v0, vec3 v1, vec3 v2)
{
    return vec3(gl_TessCoord.x) * v0 + vec3(gl_TessCoord.y) * v1 + vec3(gl_TessCoord.z) * v2;
}

void main()
{
    // Interpolate the attributes of the output vertex using the barycentric coordinates
    finalUv = interpolate2D(newUv[0], newUv[1], newUv[2]);
    finalNormal = interpolate3D(newNormal[0], newNormal[1], newNormal[2]);
    finalNormal = normalize(finalNormal);
    finalWorldPos = interpolate3D(newWorldPos[0], newWorldPos[1], newWorldPos[2]);

    finalWorldPos = transformRelativistic(finalWorldPos);
    gl_Position = projection * view * vec4(finalWorldPos, 1.0);
    // gl_Position = vec4(abberration(gl_Position.xyz), 1.0);
}

#shader fragment
#version 330 core
out vec4 FragColor;

in vec2 finalUv;
in vec3 finalNormal;

// in vec2 uv;
// in vec3 normal;

uniform sampler2D diffuse_texture;
uniform vec3 diffuse;
void main()
{
  FragColor = vec4((finalNormal + 1.0) / 2.0, 1.0);
}