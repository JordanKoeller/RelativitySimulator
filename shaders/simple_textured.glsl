#shader vertex
#version 330 core

layout (location = 0) in vec3 aPos;
layout (location = 1) in vec3 aNormal;
layout (location = 2) in vec2 aTexCoords;

out vec2 uv;
out vec3 normal;
out vec3 frag_pos;

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;

void main()
{
    gl_Position = projection * view * model * vec4(aPos, 1.0);
    uv = aTexCoords;
    normal = normalize(mat3(transpose(inverse(model))) * aNormal);
    frag_pos = vec3(model * vec4(aPos, 1.0));


}

#shader fragment
#version 330 core

in vec2 uv;
in vec3 normal;
in vec3 frag_pos;

out vec4 FragColor;

// Environment Uniforms
uniform vec3 light_position;
uniform vec3 light_ambient;
uniform vec3 light_diffuse;
uniform vec3 light_specular;
uniform vec3 camera_position;

// Material Uniforms
uniform sampler2D ambient_texture;
uniform sampler2D diffuse_texture;
uniform sampler2D specular_texture;
uniform vec3 ambient;
uniform vec3 diffuse;
uniform vec3 specular;

void main()
{
    // ambient
    float ambient_strength = 0.2;
    vec3 ambient_lighting = ambient_strength * light_ambient;
  	
    // diffuse 
    vec3 light_direction = normalize(light_position - frag_pos);
    float diff = max(dot(normal, light_direction), 0.0);
    vec3 diffuse_lighting = diff * light_diffuse;
    
    // specular
    float specular_strength = 0.5;
    vec3 view_direction = normalize(camera_position - frag_pos);
    vec3 reflect_direction = reflect(-light_direction, normal);  
    float spec = pow(max(dot(view_direction, reflect_direction), 0.0), 32);
    vec3 specular_lighting = specular_strength * spec * light_specular;  
        
    vec3 ambient_contrib =  (ambient + texture(ambient_texture, uv).xyz) * ambient_lighting;
    vec3 diffuse_contrib =  (diffuse + texture(diffuse_texture, uv).xyz) * diffuse_lighting;
    vec3 specular_contrib = (specular + texture(specular_texture, uv).xyz) * specular_lighting;
    FragColor = vec4(ambient_contrib + diffuse_contrib + specular_contrib, 1.0);
}