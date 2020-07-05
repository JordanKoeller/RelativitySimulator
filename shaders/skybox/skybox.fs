#version 330 core
out vec4 FragColor;

in vec3 TexCoords;

uniform samplerCube skybox;

uniform mat3 changeOfBasis;
uniform mat3 changeOfBasisInverse;


uniform float gamma;
uniform float beta;
uniform float t;
uniform float c;

void main()
{
  vec3 based = changeOfBasis * TexCoords;
  vec3 transformed = vec3(gamma*(based.x - beta * c * t), based.y, based.z);
  vec3 res = changeOfBasisInverse * transformed;
  FragColor = texture(skybox, res);
  // FragColor = vec4(gl_FraagCoord.xyz - 600.0, 1.0);//vec4(TexCoords.x + 1, 0.0, 0.0, 1.0);
}
