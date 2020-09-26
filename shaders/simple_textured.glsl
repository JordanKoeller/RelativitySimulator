#shader vertex
#version 330 core


layout (location = 0) in vec3 aPos;
layout (location = 1) in vec2 aTexCoords;

out vec2 uv;

#include "shaders/tesselation/lorentz.glsl"

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;
void main()
{
    // gl_Position = model * vec4(aPos, 1.0);
    uv = aTexCoords;
    vec4 pos = model * vec4(aPos, 1.0f);
    gl_Position = projection * view * vec4(transformRelativistic(pos.xyz), 1.0f);

}
#shader fragment
#version 330 core
out vec4 FragColor;

in vec2 uv;

uniform sampler2D diffuseMap;

void main()
{
	FragColor = texture(diffuseMap, uv);
}