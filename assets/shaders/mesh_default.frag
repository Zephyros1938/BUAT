#version 330 core
out vec4 FragColor;
in vec3 FragPos;
in vec3 Normal;
uniform vec3 uColor;
uniform vec3 lightPos;
uniform vec3 viewPos;
uniform vec3 lightColor;

void main() {
    // Lighting constants
    const float AMBIENT_STRENGTH = 0.08;
    const float AMBIENT_COLOR_TINT = 0.1;
    
    const float WRAP_AMOUNT = 0.3;
    const float WRAP_SCALE = 1.3;
    
    const float SPECULAR_STRENGTH = 0.9;
    const float SHININESS = 16.0;
    const float FRESNEL_POWER = 3.0;
    const float RIM_STRENGTH = 0.15;
    
    const float ATTENUATION_LINEAR = 0.09;
    const float ATTENUATION_QUADRATIC = 0.032;
    
    const float SUBSURFACE_STRENGTH = 0.2;
    
    // Ambient with subtle color variation
    vec3 ambient = AMBIENT_STRENGTH * lightColor * (1.0 + AMBIENT_COLOR_TINT * uColor);
    
    // Diffuse with wrap lighting for softer shadows
    vec3 norm = normalize(Normal);
    vec3 lightDir = normalize(lightPos - FragPos);
    float NdotL = dot(norm, lightDir);
    float diffuseWrap = max((NdotL + WRAP_AMOUNT) / WRAP_SCALE, 0.0);
    vec3 diffuse = diffuseWrap * lightColor;
    
    // Enhanced Specular with Fresnel effect
    vec3 viewDir = normalize(viewPos - FragPos);
    vec3 halfwayDir = normalize(lightDir + viewDir);
    
    float NdotH = max(dot(norm, halfwayDir), 0.0);
    float spec = pow(NdotH, SHININESS);
    
    // Fresnel effect (Schlick approximation)
    float fresnel = pow(1.0 - max(dot(viewDir, norm), 0.0), FRESNEL_POWER);
    vec3 specular = SPECULAR_STRENGTH * spec * lightColor;
    specular += fresnel * RIM_STRENGTH * lightColor;
    
    // Improved attenuation
    float distance = length(lightPos - FragPos);
    float attenuation = 1.0 / (1.0 + ATTENUATION_LINEAR * distance + ATTENUATION_QUADRATIC * (distance * distance));
    
    // Subsurface scattering approximation
    float backlight = max(dot(norm, -lightDir), 0.0);
    vec3 subsurface = SUBSURFACE_STRENGTH * backlight * lightColor * uColor;
    
    // Final composition
    vec3 result = (ambient + (diffuse + specular) * attenuation + subsurface * attenuation) * uColor;
    
    // Tone mapping
    result = result / (result + vec3(1.0));
    
    // Gamma correction
    result = pow(result, vec3(1.0/2.2));
    
    FragColor = vec4(result, 1.0);
}