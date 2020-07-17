
uniform float beta;
uniform float gamma;

uniform int lorentzFlag;
// If 0, no relativity at all
// If 1, enable lorentz transform
// If 2, enable lorentz transform and light travel time considerations


uniform vec3 cameraPos;
uniform vec3 frustum;


uniform mat3 changeOfBasis;
uniform mat3 changeOfBasisInverse;


vec3 timeTransform(vec3 pos)
{
  vec3 refFramePos = changeOfBasis * (pos - cameraPos);
  // Transform into coords such that v = <v, 0, 0>
  // First do the y coords
  float x1 = refFramePos.x;
  float y1 = refFramePos.y;
  float z1 = refFramePos.z;
  float r = sqrt(y1*y1+z1*z1);
  float h2 = 1.0/beta;
  h2 = h2*h2;
  float x2 = (2.0*x1*h2 + sqrt((2.0*x1*h2)*(2.0*x1*h2)-4.0*(h2 - 1.0)*(x1*x1*h2-r*r)))/(2*(h2 - 1.0));
  vec3 transformed = vec3(x2, y1, z1);
  return changeOfBasisInverse * transformed + cameraPos;
}

vec3 lorentzTransform(vec3 pos)
{
  vec3 refFramePos = changeOfBasis * (pos - cameraPos);
  vec3 transformed = vec3(refFramePos.x/gamma, refFramePos.y, refFramePos.z);
  return changeOfBasisInverse * transformed + cameraPos;
}


vec3 transformRelativistic(vec3 pos)
{
  vec3 ret = pos;
  if (lorentzFlag != 0) {
    ret = lorentzTransform(ret);
    if (lorentzFlag == 2 && beta > 0.01) {
        ret = timeTransform(ret);
    }
  }
  return ret;
}