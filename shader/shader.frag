#version 410
precision highp float;

uniform sampler2D surfaceTexture;

in vec3 pos;
in vec3 normal;
in vec2 uv;
out vec4 color;

void main() {
    color = texture(surfaceTexture, uv);
}