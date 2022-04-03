uniform float beta;
uniform float gamma;
uniform int lorentzFlag;
uniform vec3 cameraPos;
uniform vec3 frustum;
uniform mat3 changeOfBasis;
uniform mat3 changeOfBasisInverse;
// uniform mat3 commented_out_uniform;
vec3 lorentzTransform(vec3 pos)
{
vec3 refFramePos = changeOfBasis * (pos - cameraPos);
vec3 transformed = vec3(refFramePos.x/gamma, refFramePos.y, refFramePos.z);
return changeOfBasisInverse * transformed + cameraPos;
}