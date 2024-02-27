#version 410
precision highp float;

uniform sampler2D surfaceTexture;

in vec3 pos;
in vec2 uv;
out vec4 color;

void main() {
    color = texture(surfaceTexture, uv) + vec4(0.0, uv, 0.0);
}