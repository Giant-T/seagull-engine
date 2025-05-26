#version 460

in vec2 vPos;

layout(location = 0) out vec4 Color;

uniform vec2 Points[16];

vec3 getColorFromPos(vec2 pos) {
    vec2 nearestPoint = Points[0];

    for (int i = 1; i < 16; i++) {
        vec2 point = Points[i];
        bool isNearer = distance(pos, point) < distance(nearestPoint, point);
        nearestPoint = float(isNearer) * point + float(!isNearer) * nearestPoint;
    }

    return vec3(nearestPoint, 1.0);
}

void main(){
    Color = vec4(getColorFromPos(vPos), 1.0);
}
