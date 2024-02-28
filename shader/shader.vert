#version 410
precision highp float;

uniform mat4 translationMatrix;
uniform mat4 xRotationMatrix;
uniform mat4 yRotationMatrix;
uniform mat4 zRotationMatrix;
uniform mat4 scaleMatrix;
uniform float aspect;

uniform sampler2D surfaceTexture;

in vec3 position;
in vec3 surface_normal;
in vec2 tex_coords;

out vec3 pos;
out vec3 normal;
out vec2 uv;

void main() {
    mat4 transformationMatrix = 
        translationMatrix * scaleMatrix * yRotationMatrix * xRotationMatrix * zRotationMatrix;
    
    pos = (transformationMatrix * vec4(position, 1.0)).xyz;
    normal = transpose(inverse(mat3(transformationMatrix))) * surface_normal;
    uv = tex_coords;

    gl_Position = vec4(pos.x, pos.y * aspect, pos.z, 1.0);
}