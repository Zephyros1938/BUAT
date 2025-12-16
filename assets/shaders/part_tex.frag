#version 330 core

in vec2 TexCoord;

out vec4 FragColor;

uniform sampler2D uTexture;
uniform vec3 uColor;

void main()
{
    vec3 texColor = texture(uTexture, TexCoord).rgb;
    
    vec3 n = mix(texColor, uColor, 0.5);

    FragColor = vec4(n, 1.0);
}
