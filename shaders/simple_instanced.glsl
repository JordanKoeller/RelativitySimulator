#shader vertex
#version 330 core


layout (location = 0) in vec3 aPos;
layout (location = 1) in vec2 aTexCoords;
layout (location = 2) in mat4 model;
// layout (location = 4) in vec3 ambient;

out vec2 uv;
out vec3 ambien;
// out vec3 normal;

uniform mat4 view;
uniform mat4 projection;
void main()
{
    uv = aTexCoords;
    gl_Position = projection * view * model * vec4(aPos, 1.0f);
    // ambien = ambient;

}
#shader fragment
#version 330 core
out vec4 FragColor;

in vec2 uv;
// in vec3 ambien;

uniform sampler2D diffuse_texture;


void main()
{
	FragColor = vec4(1.0f, 1.0f, 1.0f, 1.0f); // texture(diffuse_texture, uv) * vec4(ambien, 1.0f);
  if (FragColor.a < 0.5) discard;
}