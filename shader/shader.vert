#version 410
precision highp float;

uniform mat4 modelMatrix;
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
    pos = (projectionMatrix * modelMatrix * vec4(position, 1.0)).xyz;
    normal = transpose(inverse(mat3(modelMatrix))) * surface_normal;
    uv = tex_coords;

    // Divide by 1000 because of the 1x1x1 clipping cube, eventually to be changed with model-view matrices
    gl_Position = vec4(pos.x, pos.y, pos.z / 1000.0, 1.0);
}