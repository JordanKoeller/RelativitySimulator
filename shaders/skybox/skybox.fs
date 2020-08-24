#version 330 core
out vec4 FragColor;

in vec3 TexCoords;

uniform samplerCube cubeMap;

// uniform mat3 changeOfBasis;
// uniform mat3 changeOfBasisInverse;


// uniform float gamma;

void main()
{
  FragColor = texture(cubeMap, TexCoords);
}
