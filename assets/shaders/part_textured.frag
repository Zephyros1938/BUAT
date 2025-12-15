#version 330 core

in vec2 TexCoord;

out vec4 FragColor;

uniform sampler2D uTexture;

void main()
{

    vec3 texColor = texture(uTexture, TexCoord).rgb;

    FragColor = vec4(texColor, 1.0);
}
