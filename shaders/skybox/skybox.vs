#version 330 core
layout (location = 0) in vec3 aPos;

out vec3 TexCoords;

uniform mat4 projection;
uniform mat4 view;

void main()
{
    TexCoords = aPos;
    mat4 myView = view;
    myView[0].w = 0.0;
    myView[1].w = 0.0;
    myView[2].w = 0.0;
    myView[3] = vec4(0.0, 0.0, 0.0, 0.0);
    vec4 pos = projection * myView * vec4(aPos, 1.0);
    gl_Position = pos.xyww;
}
