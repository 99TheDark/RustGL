#version 410
precision highp float;

uniform sampler2D surfaceTexture;

in vec3 pos;
in vec3 normal;
in vec2 uv;
out vec4 color;

void main() {
    float brightness = dot(normal, vec3(1.0, 0.0, 0.0));
    color = texture(surfaceTexture, uv) * (1.0 + brightness / 6.0) + vec4(pos.xy / 10.0, 0.0, 0.0);
}