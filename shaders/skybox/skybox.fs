#version 330 core
out vec4 FragColor;

in vec3 TexCoords;

uniform samplerCube skybox;

uniform mat3 changeOfBasis;
uniform mat3 changeOfBasisInverse;


uniform float gamma;

void main()
{
  vec3 based = changeOfBasis * TexCoords;
  vec3 transformed = vec3(gamma*based.x, based.y, based.z);
  vec3 res = changeOfBasisInverse * transformed;
  // FragColor = vec4(1.0, 1.0, 1.0, 1.0);//texture(skybox, res);
  FragColor = texture(skybox, res);
  // FragColor = vec4(gl_FraagCoord.xyz - 600.0, 1.0);//vec4(TexCoords.x + 1, 0.0, 0.0, 1.0);
}
