#version 330 core
out vec4 FragColor;


in vec2 TexCoord_FS_in;
in vec3 Normal_FS_in;
in vec3 WorldPos_FS_in;


uniform sampler2D texture_diffuse1;


void main()
{
    FragColor = texture(texture_diffuse1, TexCoord_FS_in);
    if (FragColor.a < 0.99) {
        discard;
    }
    // FragColor = vec4((Normal_FS_in + 1.0) / 2.0, 1.0);
}