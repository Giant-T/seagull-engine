#version 460

in vec2 vPos;

vec3 posToRGB(vec2 pos) {
    // Wrap position to [0,1] range just in case
    vec2 p = fract(pos);

    // Example: hue based on angle, brightness based on distance from center
    float angle = atan(p.y - 0.5, p.x - 0.5);
    float dist = length(p - 0.5);

    // Normalize angle to [0,1]
    float hue = (angle / (2.0 * 3.1415926)) + 0.5;

    // Convert hue to RGB (simple rainbow gradient)
    vec3 color = vec3(0.0);
    color.r = abs(hue * 6.0 - 3.0) - 1.0;
    color.g = 2.0 - abs(hue * 6.0 - 2.0);
    color.b = 2.0 - abs(hue * 6.0 - 4.0);

    // Clamp and normalize
    color = clamp(color, 0.0, 1.0);

    // Optional: fade out near edges
    color *= smoothstep(0.5, 0.0, dist);

    return color;
}

vec3 uvToColor(vec2 uv) {
    return vec3(uv, 0.5 + 0.5 * sin(6.2831 * (uv.x + uv.y)));
}

layout(location = 0) out vec4 Color;

// https://lospec.com/palette-list/fading-16
uniform vec3 ColorPalette[16] = vec3[](
    vec3(0.87, 0.81, 0.60), // #ddcf99
    vec3(0.80, 0.66, 0.48), // #cca87b
    vec3(0.73, 0.48, 0.38), // #b97a60
    vec3(0.61, 0.32, 0.30), // #9c524e
    vec3(0.47, 0.26, 0.32), // #774251
    vec3(0.29, 0.24, 0.27), // #4b3d44
    vec3(0.31, 0.33, 0.39), // #4e5463
    vec3(0.36, 0.57, 0.45), // #5b7d73
    vec3(0.56, 0.73, 0.49), // #8e9f7d
    vec3(0.39, 0.33, 0.33), // #645355
    vec3(0.55, 0.49, 0.47), // #8c7c79
    vec3(0.66, 0.62, 0.55), // #a99c8d
    vec3(0.49, 0.49, 0.38), // #7d7b62
    vec3(0.67, 0.64, 0.29), // #aaa25d
    vec3(0.52, 0.42, 0.35), // #846d59
    vec3(0.66, 0.54, 0.42)  // #a88a5e
);

uniform int NColors = 16;

//https://en.wikipedia.org/wiki/Ordered_dithering
const mat4 bayerMatrix = mat4(
    vec4( 0.0, 8.0, 2.0, 10.0),
    vec4(12.0, 4.0, 14.0, 6.0),
    vec4( 3.0, 11.0, 1.0, 9.0),
    vec4(15.0, 7.0, 13.0, 5.0)
) / 16.0;

vec3 quantize(vec3 actualColor) {
    vec3 nearestColor = ColorPalette[0];

    for (int i = 1; i < NColors; i++) {
        vec3 color = ColorPalette[i];
        bool isNearer = distance(color, actualColor) < distance(nearestColor, actualColor);
        nearestColor = float(isNearer) * color + float(!isNearer) * nearestColor;
    }

    return nearestColor;
}

void main() {
    vec3 c = uvToColor(vPos);

    ivec2 coords = ivec2(mod(gl_FragCoord.xy, 4.0));
    
    float threshold = bayerMatrix[coords.y][coords.x];
    const float r = 1.0 / 16.0;
    c = quantize(c + r * threshold);

    Color = vec4(c, 1.0);
}

