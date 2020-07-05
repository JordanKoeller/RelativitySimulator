#version 330 core
layout (location = 0) in vec3 aPos;
layout (location = 1) in vec3 aNormal;
layout (location = 2) in vec2 aTexCoords;

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;

uniform vec3 cameraPos;

uniform mat3 changeOfBasis;
uniform mat3 changeOfBasisInverse;

uniform float gamma;
uniform float beta;
uniform float t;
uniform float c;

out vec2 TexCoord_CS_in;
out vec3 WorldPos_CS_in;
out vec3 Normal_CS_in;

vec3 lorentzTransform(vec3 pos)
{
  vec3 refFramePos = changeOfBasis * (pos - cameraPos);
  vec3 transformed = vec3(refFramePos.x/gamma, refFramePos.y, refFramePos.z);
  return changeOfBasisInverse * transformed + cameraPos;
}

void main()
{
    TexCoord_CS_in = aTexCoords;
    WorldPos_CS_in =(model * vec4(aPos, 1.0)).xyz;
    vec3 worldCoords = (model * vec4(aNormal, 0.0)).xyz;
    Normal_CS_in = lorentzTransform(worldCoords);
}
