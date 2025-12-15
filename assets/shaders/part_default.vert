#version 330 core

layout (location = 0) in vec3 aPos;
layout (location = 1) in vec3 aColor;

out vec3 ourColor;
out float fragDistance;

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;

void main()
{
    vec4 worldPos = model * vec4(aPos, 1.0);
    vec4 viewPos  = view * worldPos;
    fragDistance = length(viewPos.xyz);
    ourColor = aColor;
    gl_Position = projection * viewPos;
}