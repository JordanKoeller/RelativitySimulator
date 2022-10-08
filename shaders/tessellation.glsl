
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
out vec3 tessColor[];

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;

float lerp_clip(float iMin, float iMax, float fMin, float fMax, float v) {
  float dI = iMax - iMin;
  float dF = fMax - fMin;
  float frac = (v - iMin) / dI;
  return min(max(frac * dF + fMin, fMin), fMax);
}




float tess_level(vec3 v1, vec3 v2) {
  // I want to know curvature. That's what decides how fine of a tessellation is needed.
  // The easiest way I can think of is to compute the midpt of each side
  // and transform all into a relativistic frame.

  // Then I compute the angle from the midpt to the two end segments to approx. The
  // curvature. The larger the angle, the larger the degree of tessellation.
  float PI = 3.14159265;
  if (lorentzFlag == 2 && beta > 0.01) {
    mat4 screenspace = projection * view;
    vec3 midpt = (v1 + v2) / 2.0;
    vec3 relV1 =  vec4(timeTransform(lorentzTransform(v1)), 1.0).xyz;
    vec3 relV2 =  vec4(timeTransform(lorentzTransform(v2)), 1.0).xyz;
    vec3 relMid = vec4(timeTransform(lorentzTransform(midpt)), 1.0).xyz;
    vec3 vect1 = normalize(relV1 - midpt);
    vec3 vect2 = normalize(relV2 - midpt);
    float angle = acos(dot(vect1, vect2));
    float tess_float = lerp_clip(-PI, PI, 0, 1, PI - angle);
    tessColor[gl_InvocationID] = vec3(tess_float, tess_float, tess_float);
    float tess_level_result = lerp_clip(0, 1, 1, 10, tess_float);
    return tess_level_result;
    // float angle = dot(normalize(relV2 - relMid), normalize(relMid - relV1));
    // return max((1.0 - abs(angle)) * 50.0, 1.0);
  } else {
    tessColor[gl_InvocationID] = vec3(0, 0, 0);
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
    gl_TessLevelOuter[0] = tess_level(newWorldPos[0], newWorldPos[1]);
    gl_TessLevelOuter[1] = tess_level(newWorldPos[1], newWorldPos[2]);
    gl_TessLevelOuter[2] = tess_level(newWorldPos[2], newWorldPos[0]);
    tessColor[gl_InvocationID] = vec3(
      lerp_clip(1, 10, 0, 1, gl_TessLevelOuter[0]),
      lerp_clip(1, 10, 0, 1, gl_TessLevelOuter[1]),
      lerp_clip(1, 10, 0, 1, gl_TessLevelOuter[2])
    );
    // gl_TessLevelOuter[0] = 10;
    // gl_TessLevelOuter[1] = 10;
    // gl_TessLevelOuter[2] = 10;
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
in vec3 tessColor[];

out vec3 finalWorldPos;
out vec2 finalUv;
out vec3 finalNormal;
// out vec3 finalTessColor;

out TS_OUT {
    vec3 tessColor;
} ts_out;

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
    ts_out.tessColor = interpolate3D(tessColor[0], tessColor[1], tessColor[2]);

    finalWorldPos = transformRelativistic(finalWorldPos);
    gl_Position = projection * view * vec4(finalWorldPos, 1.0);
}
