#version 410
precision highp float;

uniform mat3 translationMatrix;
uniform mat3 rotationMatrix;
uniform mat3 scaleMatrix;

uniform sampler2D surfaceTexture;

in vec2 position;
in vec2 tex_coords;

out vec2 pos;
out vec2 uv;

void main() {
    vec3 transformedPosition = translationMatrix * rotationMatrix * scaleMatrix * vec3(position, 1.0);
    pos = transformedPosition.xy;
    uv = tex_coords;

    gl_Position = vec4(transformedPosition.xy, 0.0, 1.0);
}