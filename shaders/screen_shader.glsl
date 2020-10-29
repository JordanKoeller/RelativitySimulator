#shader vertex
#version 330 core
layout (location = 0) in vec2 aPos;
layout (location = 1) in vec2 aUv;

out vec2 uv;

void main()
{
    gl_Position = vec4(aPos, 0.0, 1.0);
    uv = aUv;

}
#shader fragment
#version 330 core
in vec2 uv;

out vec4 FragColor;

uniform sampler2D tex;

void main()
{
	// FragColor = vec4(uv, 1.0, 1.0);
  FragColor = texture(tex, uv);
  // FragColor = vec4(vec3(1.0 - texture(tex, uv)), 1.0);

}