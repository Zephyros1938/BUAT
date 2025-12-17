#version 330 core

out vec4 FragColor;

in vec3 FragPos;
in vec3 Normal;

uniform vec3 uColor;
const vec3 lightPos = vec3(0,2,0);
uniform vec3 viewPos;
const vec3 lightColor = vec3(0.0,1.0,0.0);

void main() {
    // Ambient
    float ambientStrength = 0.05;
    vec3 ambient = ambientStrength * lightColor;
  	
    // Diffuse
    vec3 norm = normalize(Normal);
    vec3 lightDir = normalize(lightPos - FragPos);
    float diff = max(dot(norm, lightDir), 0.0);
    vec3 diffuse = diff * lightColor;
    
    // Specular
    float specularStrength = 0.8;
    vec3 viewDir = normalize(viewPos - FragPos);
    vec3 halfwayDir = normalize(lightDir + viewDir);  // Blinn-Phong
    float spec = pow(max(dot(norm, halfwayDir), 0.0), 32.0); // 32 is very shiny
    vec3 specular = specularStrength * spec * lightColor;
            
    // Attenuation
    float distance = length(lightPos - FragPos);
    float attenuation = 1.0 / (1.0 + 0.045 * distance + 0.0075 * (distance * distance));

    vec3 result = (ambient + (diffuse + specular) * attenuation) * uColor;
    
    result = pow(result, vec3(1.0/2.2));
    
    FragColor = vec4(result, 1.0);
}