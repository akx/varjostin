#pragma @bpm {range:[100,180]}
uniform float bpm = 140;

vec3 hsv2rgb(vec3 c) {
    vec4 K = vec4(1.0, 2.0 / 3.0, 1.0 / 3.0, 3.0);
    vec3 p = abs(fract(c.xxx + K.xyz) * 6.0 - K.www);
    return c.z * mix(K.xxx, clamp(p - K.xxx, 0.0, 1.0), c.y);
}

void mainImage(out vec4 out_color, in vec2 fragCoord) {
    vec2 uv = fragCoord.xy / iResolution.xy;
    float beats = iTime * bpm / 60.0;
    float beat = pow(1.0 - fract(beats), 1.2);

    vec3 col = hsv2rgb(vec3(uv.x + iTime, 1, 1));
    out_color = mix(vec4(col, 1), vec4(0), step(beat, uv.y));
}