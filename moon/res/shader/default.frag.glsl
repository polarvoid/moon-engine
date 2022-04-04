#version 300 es
precision highp float;

uniform float uTime;
uniform mat4 uModel;
uniform mat4 uView;
uniform mat4 uProj;
uniform sampler2D uTex0;
uniform sampler2D uTex1;
uniform vec3 uCamPos;
uniform vec4 uColor;

in vec2 vPosition;
in vec2 vTexCoord;
in vec4 vColor;

out vec4 color;

void main() {
    color = texture(uTex0, vTexCoord) * vColor * uColor;
}
