#shader vertex
#version 330 core
layout (location = 0) in vec3 aPos;
layout (location = 0) in vec2 aUv;


out vec2 uv;

uniform mat4 projection;
uniform mat4 view;
uniform mat4 

void main()
{
    uvw = aPos;
    mat4 myView = view;
    myView[0].w = 0.0;
    myView[1].w = 0.0;
    myView[2].w = 0.0;
    myView[3] = vec4(0.0, 0.0, 0.0, 0.0);
    vec4 pos = projection * myView * vec4(aPos, 1.0);
    gl_Position = pos.xyww;
}

#shader fragment
#version 330 core
out vec4 FragColor;

in vec3 uvw;

uniform samplerCube skybox;

void main()
{
  FragColor = texture(skybox, uvw);
}