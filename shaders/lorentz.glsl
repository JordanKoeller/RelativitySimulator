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

#shader fragment
#version 330 core
out vec4 FragColor;

in vec2 finalUv;
in vec3 finalNormal;
in vec3 finalWorldPos;
// in vec2 uv;
// in vec3 normal;

uniform sampler2D diffuse_texture;
uniform vec3 diffuse;
void main()
{
  // if (distance(finalWorldPos.xyz, gl_FragCoord.xyz) < 0.5) {
    // FragColor = vec4(gl_PointCoord, 0.0,1.0);
  // } else {
	  FragColor = texture(diffuse_texture, finalUv) * vec4(diffuse, 1.0);
  // }
	// FragColor = texture(diffuse_texture, uv);
  if (FragColor.a < 0.5) discard;
}