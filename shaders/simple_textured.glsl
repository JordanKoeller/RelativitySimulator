#shader vertex
#version 330 core

layout (location = 0) in vec3 aPos;
layout (location = 1) in vec3 aNormal;
layout (location = 2) in vec3 aTangent;
layout (location = 3) in vec3 aBitangent;
layout (location = 4) in vec2 aTexCoords;

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;
uniform vec3 light_position;
uniform vec3 camera_position;

out vec2 uv;
out vec3 frag_pos;
out vec3 frag_normal;
out vec3 tangent_light_position;
out vec3 tangent_camera_position;
out vec3 tangent_frag_position;


void main()
{
    gl_Position = projection * view * model * vec4(aPos, 1.0);
    uv = aTexCoords;
    frag_pos = vec3(model * vec4(aPos, 1.0));
    frag_normal = aNormal;

    mat3 normalMatrix = transpose(inverse(mat3(model)));
    vec3 T = normalize(normalMatrix * aTangent);
    vec3 N = normalize(normalMatrix * aNormal);
    T = normalize(T - dot(T, N) * N);
    vec3 B = cross(N, T);

    mat3 TBN = transpose(mat3(T, B, N));
    
    tangent_light_position = TBN * light_position;
    tangent_camera_position  = TBN * camera_position;
    tangent_frag_position  = TBN * frag_pos;

}

#shader fragment
#version 330 core

in vec2 uv;
in vec3 frag_position;
in vec3 frag_normal;
in vec3 tangent_light_position;
in vec3 tangent_camera_position;
in vec3 tangent_frag_position;
in vec3 light_position;
in vec3 camera_position;

out vec4 FragColor;

// Environment Uniforms
uniform vec3 light_ambient;
uniform vec3 light_diffuse;
uniform vec3 light_specular;
uniform float ambient_strength;
uniform float diffuse_strength;
uniform float specular_power;
uniform float specular_strength;

// Material Uniforms
uniform sampler2D diffuse_texture;
uniform sampler2D specular_texture;
uniform sampler2D normal_texture;
uniform vec3 ambient;
uniform vec3 diffuse;
uniform vec3 specular;

void main()
{

    vec3 normal = normalize(texture(normal_texture, uv).rgb * 2.0 - 1.0);

    // ambient
    vec3 ambient_lighting = ambient_strength * light_ambient;

    // diffuse
    vec3 light_direction = normalize(tangent_light_position - tangent_frag_position);
    float diff = max(dot(light_direction, normal), 0.0);
    vec3 diffuse_lighting = diff * light_diffuse * diffuse_strength;

    // specular
    vec3 view_direction = normalize(tangent_camera_position - tangent_frag_position);
    vec3 reflect_direction = reflect(-light_direction, normal);
    vec3 halfway_direction = normalize(light_direction + view_direction);  
    float spec = pow(max(dot(normal, halfway_direction), 0.0), specular_power);
  	vec3 specular_lighting = spec * light_specular;
    float spec_mag = texture(specular_texture, uv).x * specular_strength;

    vec3 ambient_contrib =  (ambient  + texture(diffuse_texture, uv).xyz) * ambient_lighting;
    vec3 diffuse_contrib =  (diffuse  + texture(diffuse_texture, uv).xyz) * diffuse_lighting;
    vec3 specular_contrib = (specular + vec3(spec_mag, spec_mag, spec_mag)) * specular_lighting;

    // FragColor = vec4(normal, 1.0); // vec4(ambient_contrib + diffuse_contrib + specular_contrib, 1.0);
    FragColor = vec4(ambient_contrib + diffuse_contrib + specular_contrib, 1.0);
}