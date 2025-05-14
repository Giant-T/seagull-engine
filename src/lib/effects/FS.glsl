#version 460

uniform float Elapsed = 0.0;

in vec2 vPos;

layout(location = 0) out vec4 Color;

// https://lospec.com/palette-list/fading-16
// uniform vec3 ColorPalette[16] = vec3[](
//     vec3(0.87, 0.81, 0.60), // #ddcf99
//     vec3(0.80, 0.66, 0.48), // #cca87b
//     vec3(0.73, 0.48, 0.38), // #b97a60
//     vec3(0.61, 0.32, 0.30), // #9c524e
//     vec3(0.47, 0.26, 0.32), // #774251
//     vec3(0.29, 0.24, 0.27), // #4b3d44
//     vec3(0.31, 0.33, 0.39), // #4e5463
//     vec3(0.36, 0.57, 0.45), // #5b7d73
//     vec3(0.56, 0.73, 0.49), // #8e9f7d
//     vec3(0.39, 0.33, 0.33), // #645355
//     vec3(0.55, 0.49, 0.47), // #8c7c79
//     vec3(0.66, 0.62, 0.55), // #a99c8d
//     vec3(0.49, 0.49, 0.38), // #7d7b62
//     vec3(0.67, 0.64, 0.29), // #aaa25d
//     vec3(0.52, 0.42, 0.35), // #846d59
//     vec3(0.66, 0.54, 0.42)  // #a88a5e
// );
uniform vec3 ColorPalette[32] = vec3[](
    vec3(0.357, 0.651, 0.459),  // #5ba675
    vec3(0.424, 0.788, 0.424),  // #6bc96c
    vec3(0.671, 0.867, 0.392),  // #abdd64
    vec3(0.988, 0.910, 0.553),  // #fcef8d
    vec3(1.000, 0.722, 0.475),  // #ffb879
    vec3(0.918, 0.384, 0.306),  // #ea6262
    vec3(0.800, 0.259, 0.212),  // #cc425e
    vec3(0.639, 0.220, 0.345),  // #a32858
    vec3(0.459, 0.090, 0.318),  // #751756
    vec3(0.224, 0.063, 0.278),  // #390947
    vec3(0.380, 0.090, 0.318),  // #611851
    vec3(0.533, 0.333, 0.333),  // #873555
    vec3(0.651, 0.396, 0.396),  // #a6555f
    vec3(0.788, 0.451, 0.451),  // #c97373
    vec3(0.949, 0.682, 0.600),  // #f2ae99
    vec3(1.000, 0.765, 0.949),  // #ffc3f2
    vec3(0.933, 0.890, 0.925),  // #ee8fcb
    vec3(0.831, 0.431, 0.702),  // #d46eb3
    vec3(0.529, 0.243, 0.518),  // #873e84
    vec3(0.122, 0.063, 0.165),  // #1f102a
    vec3(0.290, 0.188, 0.322),  // #4a3052
    vec3(0.482, 0.329, 0.502),  // #7b5480
    vec3(0.659, 0.627, 0.627),  // #a6859f
    vec3(0.851, 0.741, 0.769),  // #d9bdc8
    vec3(1.000, 1.000, 1.000),  // #ffffff
    vec3(0.682, 0.898, 1.000),  // #aee2ff
    vec3(0.553, 0.729, 1.000),  // #8db7ff
    vec3(0.427, 0.631, 0.980),  // #6d80fa
    vec3(0.518, 0.537, 0.800),  // #8465ec
    vec3(0.514, 0.329, 0.769),  // #834dc4
    vec3(0.490, 0.176, 0.627),  // #7d2da0
    vec3(0.306, 0.094, 0.486)   // #4e187c
);

uniform int NColors = 32;

// https://en.wikipedia.org/wiki/Ordered_dithering
const mat4 bayerMatrix = mat4(
    vec4( 0.0, 8.0, 2.0, 10.0),
    vec4(12.0, 4.0, 14.0, 6.0),
    vec4( 3.0, 11.0, 1.0, 9.0),
    vec4(15.0, 7.0, 13.0, 5.0)
) / 16.0;

float swirl(vec2 uv) {
    vec2 p = uv - 0.5;
    float angle = atan(p.y, p.x);
    float radius = length(p);
    float t = Elapsed * 0.0001;
    return sin(6.0 * radius - t + angle);
}

vec3 uvToColor(vec2 uv) {
    float f = swirl(uv);

    float r = 0.5 + 0.5 * sin(6.2831 * (uv.x + f));
    float g = 0.5 + 0.5 * sin(6.2831 * (uv.y + f));
    float b = 0.5 + 0.5 * sin(6.2831 * (uv.x + uv.y + f));
    return vec3(r, g, b);
}

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
