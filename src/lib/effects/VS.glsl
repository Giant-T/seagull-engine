#version 460

out gl_PerVertex {
    vec4 gl_Position;
};

out vec2 vPos;

layout (location = 0) in vec3 Position;

void main() {
    gl_Position = vec4(Position, 1.0);
    vPos = gl_Position.xy * 0.5 + 0.5;
}
