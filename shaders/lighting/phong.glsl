

uniform vec3 ambientLight;
uniform vec3 diffuseLight;
uniform vec3 specularLight;

uniform vec3 directionalLight;
uniform vec3 cameraPos;
uniform mat4 normalMatrix;

vec3 ambientLighting(vec3 rgb) {
    return rgb * ambientLight;
}

vec3 diffuseLighting(vec3 rgb, vec3 norm) {
    vec3 transNorm = normalize(mat3(normalMatrix) * norm);
    float diff = max(dot(transNorm, -normalize(directionalLight)), 0.0);
    return diff * rgb;
}

vec3 specularLighting(vec3 rgb, vec3 norm, vec3 pos) {
    vec3 transNorm = normalize(mat3(normalMatrix) * norm);
    vec3 viewDir = normalize(cameraPos - pos);
    vec3 reflectDir = reflect(normalize(directionalLight), transNorm);
    float spec = pow(max(dot(viewDir, reflectDir), 0.0), 32);
    return specularLight * spec * rgb;
}