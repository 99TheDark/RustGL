#version 410
precision highp float;

uniform mat4 modelMatrix;
uniform mat4 viewMatrix;
uniform mat4 projectionMatrix;

uniform float aspect;

uniform sampler2D surfaceTexture;

in vec3 position;
in vec3 surface_normal;
in vec2 tex_coords;

out vec3 pos;
out vec3 normal;
out vec2 uv;

void main() {
    mat4 modelViewMatrix = viewMatrix * modelMatrix;

    gl_Position = projectionMatrix * modelViewMatrix * vec4(position, 1.0);

    pos = gl_Position.xyz / gl_Position.w;
    normal = transpose(inverse(mat3(modelViewMatrix))) * surface_normal;
    uv = tex_coords;
}