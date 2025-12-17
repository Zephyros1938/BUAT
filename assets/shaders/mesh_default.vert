#version 330 core

// Layouts match your Rust attribute pointers
layout (location = 0) in vec3 aPos;       // Position
layout (location = 2) in vec2 aTexCoord;  // UVs (from vt in your OBJ)
layout (location = 3) in vec3 aNormal;    // Normals (from vn in your OBJ)

out vec3 FragPos;
out vec3 Normal;
out vec2 TexCoord;

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;

void main() {
    // Calculate world position for lighting
    FragPos = vec3(model * vec4(aPos, 1.0));
    
    // Transform normals to world space
    Normal = mat3(transpose(inverse(model))) * aNormal;
    
    // Pass UVs (optional for now, but good to have)
    TexCoord = aTexCoord;

    gl_Position = projection * view * model * vec4(aPos, 1.0);
}