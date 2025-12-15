#version 330 core

in vec3 ourColor;
in float fragDistance;

out vec4 FragColor;

/* Fog constants */
const vec3  FOG_COLOR = vec3(0.0, 0.0, 0.0);
const float FOG_START = 0.0;
const float FOG_END   = 5.0;

void main()
{
    float fogFactor = clamp(
        (FOG_END - fragDistance) / (FOG_END - FOG_START),
        0.0,
        1.0
    );

    vec3 color = mix(FOG_COLOR, ourColor, fogFactor);
    FragColor = vec4(color, 1.0);
}