#version 460

uniform sampler2D FBO;

in vec2 vPos;

layout(location = 0) out vec4 Color;

void main(){
    Color.rgb = texture(FBO, vPos).rgb;
    Color.a = 1.0;
}
