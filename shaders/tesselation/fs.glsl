#version 330 core
out vec4 FragColor;

#include "shaders/lighting/phong.glsl"

in vec2 TexCoord_FS_in;
in vec3 Normal_FS_in;
in vec3 WorldPos_FS_in;

uniform sampler2D texture_diffuse1;

void main()
{
    vec3 tex = texture(texture_diffuse1, TexCoord_FS_in).xyz;
    FragColor = vec4(
        ambientLighting(tex) +
        diffuseLighting(tex, Normal_FS_in) +
        specularLighting(tex, Normal_FS_in, WorldPos_FS_in)
        , 0.0);
    // FragColor = vec4(
    //     ambientLighting(tex) + diffuseLighting(tex, Normal_FS_in) + specularLighting(tex, Normal_FS_in, WorldPos_FS_in), 0.0);
}
