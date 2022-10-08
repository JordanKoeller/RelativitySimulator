#shader vertex
#version 330 core


layout (location = 0) in vec3 aPos;
layout (location = 1) in vec3 aNormal;
layout (location = 2) in vec2 aTexCoords;

out vec2 uv;
out vec3 normal;
out vec3 worldPos;


uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;

void main()
{
    uv = aTexCoords;
    normal = (model * vec4(aNormal, 1.0)).xyz;
    worldPos = (model * vec4(aPos, 1.0)).xyz;
    // gl_Position = projection * view * model * vec4(aPos, 1.0);
}

#include "shaders/tessellation.glsl"

#shader geometry
#version 330 core
layout (triangles) in;
layout (triangle_strip, max_vertices = 4) out;

in TS_OUT {
    vec3 tessColor;
} gs_in[]; 

out vec4 ScreenPos;
out vec3 tessColor;
out vec2 billboard_uv;

void build_house(vec4 position)
{    
    ScreenPos = vec4(position.x, position.y, position.z, position.w);
    gl_Position = position + vec4(-0.2, -0.2, -0.01, 0.0);    // 1:bottom-left
    billboard_uv = vec2(0.0, 0.0);
    EmitVertex();   
    gl_Position = position + vec4( 0.2, -0.2, -0.01, 0.0);    // 2:bottom-right
    billboard_uv = vec2(1.0, 0.0);
    EmitVertex();
    gl_Position = position + vec4(-0.2,  0.2, -0.01, 0.0);    // 3:top-left
    billboard_uv = vec2(0.0, 1.0);
    EmitVertex();
    gl_Position = position + vec4( 0.2,  0.2, -0.01, 0.0);    // 4:top-right
    billboard_uv = vec2(1.0, 1.0);
    EmitVertex();
    EndPrimitive();
}

void main() {
    tessColor = gs_in[0].tessColor;
    build_house(gl_in[0].gl_Position);
    tessColor = gs_in[1].tessColor;
    build_house(gl_in[1].gl_Position);
    tessColor = gs_in[2].tessColor;
    build_house(gl_in[2].gl_Position);
}  

#shader fragment
#version 330 core
in vec4 ScreenPos;
in vec3 tessColor;
in vec2 billboard_uv;
out vec4 FragColor;
uniform vec2 WindowDimensions;
uniform sampler2D debug_texture;

float lerp(float iMin, float iMax, float fMin, float fMax, float v) {
  float dI = iMax - iMin;
  float dF = fMax - fMin;
  float frac = (v - iMin) / dI;
  return frac * dF + fMin;
}

void main()
{
    float avg_interp = (tessColor.x + tessColor.y + tessColor.z) / 3.0;
    float number_fragment = floor(avg_interp * 10.0) / 10.0;
    vec2 uv = vec2(number_fragment + billboard_uv.x / 10.0, billboard_uv.y);
    FragColor = texture(debug_texture, uv);
}