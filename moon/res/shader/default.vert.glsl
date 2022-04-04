#version 300 es

layout (location = 0) in vec2 aPosition;
layout (location = 1) in vec2 aTexCoord;
layout (location = 2) in vec4 aColor;

uniform float uTime;
uniform mat4 uModel;
uniform mat4 uView;
uniform mat4 uProj;
uniform vec4 uColor;

out vec2 vTexCoord;
out vec2 vPosition;
out vec4 vColor;

void main() {
    gl_Position = uProj * uView * vec4(aPosition, 0.0, 1.0);
    vTexCoord = aTexCoord;
    vColor = aColor;
}
