#shader vertex
#version 330 core


layout (location = 0) in vec3 aPos;
layout (location = 1) in vec2 aTexCoords;
layout (location = 2) in mat4 aModel;
layout (location = 6) in uint aDiffuseIndex;
// layout (location = 4) in vec3 ambient;

uniform mat4 view;
uniform mat4 projection;

out VS_OUT {
  vec2 TexCoords;
  flat uint DiffuseIndex;
} vs_out;

void main()
{
    gl_Position = gl_Position = projection * view * aModel * vec4(aPos, 1.0f);
    vs_out.TexCoords = aTexCoords;
    vs_out.DiffuseIndex = aDiffuseIndex;

}
#shader fragment
#version 430 core

uniform sampler2D diffuse_texture[32];

in VS_OUT {
  vec2 TexCoords;
  flat uint DiffuseIndex;
} fs_in;

out vec4 FragColor;


void main()
{
	FragColor =  texture(diffuse_texture[fs_in.DiffuseIndex], fs_in.TexCoords);
  if (FragColor.a < 0.5) discard;
}