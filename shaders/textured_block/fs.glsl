#version 330 core
out vec4 FragColor;

in vec2 uv;

// texture samplers
uniform sampler2D diffuseMap;

void main()
{
	FragColor = texture(diffuseMap, uv);
}