#version 410
precision highp float;

uniform mat4 translationMatrix;
uniform mat4 xRotationMatrix;
uniform mat4 yRotationMatrix;
uniform mat4 zRotationMatrix;
uniform mat4 scaleMatrix;

uniform sampler2D surfaceTexture;

in vec3 position;
in vec3 surface_normal;
in vec2 tex_coords;

out vec3 pos;
out vec3 normal;
out vec2 uv;

void main() {
    mat4 transformationMatrix = xRotationMatrix * zRotationMatrix * scaleMatrix * yRotationMatrix * translationMatrix;
    
    vec4 transformedPosition = transformationMatrix * vec4(position, 1.0);
    pos = transformedPosition.xyz;
    normal = surface_normal;
    uv = tex_coords;

    gl_Position = vec4(transformedPosition.xyz, 1.0);
}