#shader vertex
#version 330 core


layout (location = 0) in vec3 aPos;
layout (location = 1) in vec3 aNormal;
layout (location = 2) in vec2 aTexCoords;

out vec2 uv;
out vec3 normal;

#include "shaders/tesselation/lorentz.glsl"

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;
void main()
{
    // gl_Position = model * vec4(aPos, 1.0);
    uv = aTexCoords;
    normal = aNormal;
    vec4 pos = vec4(aPos, 1.0f);
    gl_Position = projection * view * model * vec4(transformRelativistic(pos.xyz), 1.0f);

}
#shader fragment
#version 330 core
out vec4 FragColor;

in vec2 uv;
in vec3 normal;

uniform sampler2D diffuse_texture;

void main()
{
	// FragColor = vec4(normal / 2.0 + 1.0, 1.0);
	FragColor = texture(diffuse_texture, uv);
}