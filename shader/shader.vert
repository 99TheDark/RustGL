#version 410
precision highp float;

uniform mat3 translationMatrix;

in vec2 position;
in vec3 color;
out vec3 fragColor;

void main() {
    fragColor = color;

    vec3 transformedPosition = translationMatrix * vec3(position, 1.0);
    gl_Position = vec4(transformedPosition.xy, 0.0, 1.0);
}