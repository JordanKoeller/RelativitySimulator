#version 330 core
#define NR_POINT_LIGHTS 4

struct Material {
  sampler2D diffuse;
  sampler2D specular;
  float shininess;
};

struct Directionlight {
  vec3 direction;

  vec3 ambient;
  vec3 diffuse;
  vec3 specular;
};

struct Flashlight {
  vec3 position;
  vec3 direction;

  float radius;
  float outerRadius;

  vec3 diffuse;
  vec3 specular;

  vec3 attenuation;
};

struct Pointlight {
  vec3 position;

  vec3 diffuse;
  vec3 specular;

  vec3 attenuation;

};

in vec3 Normal;
in vec3 FragPos;
in vec2 TexCoords;
out vec4 FragColor;

uniform vec3 viewPos;
uniform Material material;
uniform Directionlight directionlight;
uniform Flashlight flashlight;
uniform Pointlight pointlights[NR_POINT_LIGHTS];


vec3 diffuseSpecular(vec3 diffuse, vec3 specular, float incidentAngle, vec3 lightDir, vec3 norm);

float attenuate(vec3 attenuationVector, float dist);

vec3 useDirectionalLight(Directionlight light, vec3 norm);

vec3 usePointlight(Pointlight light, vec3 norm);

vec3 useFlashlight(Flashlight light, vec3 norm);

void main()
{
  // Some crucial geometric computations
  vec3 norm = normalize(Normal);
  vec3 result = vec3(0.0f, 0.0f, 0.0f);
  result += useDirectionalLight(directionlight, norm);
  for (int i=0; i < NR_POINT_LIGHTS; ++i)
    result += usePointlight(pointlights[i], norm);
  result += useFlashlight(flashlight, norm);
  FragColor = vec4(result, 1.0f);
}

vec3 diffuseSpecular(vec3 diffuse, vec3 specular, float incidentAngle, vec3 lightDir, vec3 norm) {
    // ambient
  	
  // diffuse 
  vec3 dif = diffuse * incidentAngle * vec3(texture(material.diffuse, TexCoords));
    
  // specular
  vec3 viewDir = normalize(viewPos - FragPos);
  vec3 reflectDir = reflect(-lightDir, norm);  
  float spec = pow(max(dot(viewDir, reflectDir), 0.0), material.shininess);
  vec3 speclr = specular * (spec * vec3(texture(material.specular, TexCoords)));

  return dif + speclr;
}

float attenuate(vec3 attenuationVector, float dist) {
  //attenuationVector has layout of (x) constant (y) linear (z) quadratic
  return 1.0f / (attenuationVector.x + attenuationVector.y * dist + attenuationVector.z * dist * dist);
}


vec3 useDirectionalLight(Directionlight light, vec3 norm) {
  vec3 ambient  = light.ambient * texture(material.diffuse, TexCoords).rgb;
  vec3 lightDir = -normalize(light.direction);
  float incidentAngle = max(dot(norm, lightDir), 0.0);
  vec3 diffuseAndSpecular = diffuseSpecular(light.diffuse, light.specular, incidentAngle, lightDir, norm);
  return ambient + diffuseAndSpecular;
}

vec3 usePointlight(Pointlight light, vec3 norm) {
  vec3 lightDir = normalize(light.position - FragPos);
  float incidentAngle = max(dot(norm, lightDir), 0.0);
  vec3 diffuseAndSpecular = diffuseSpecular(light.diffuse, light.specular, incidentAngle, lightDir, norm);

  float dist = length(light.position - FragPos);
  float attenuation = attenuate(light.attenuation, dist);
  return diffuseAndSpecular * attenuation;
}

vec3 useFlashlight(Flashlight light, vec3 norm) {
    vec3 lightDir = normalize(light.position - FragPos);
    float theta = dot(lightDir, normalize(-light.direction)); 
    float epsilon = (light.radius - light.outerRadius);
    float intensity = clamp((theta - light.outerRadius) / epsilon, 0.0, 1.0);     
    float dist = length(light.position - FragPos);
    float incidentAngle = max(dot(norm, lightDir), 0.0);
    vec3 ret = diffuseSpecular(light.diffuse, light.specular, incidentAngle, lightDir, norm);
    ret *= intensity;
    ret *= attenuate(light.attenuation, dist);
    return ret;
}
