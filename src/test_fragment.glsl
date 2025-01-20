void mainImage(out vec4 out_color, in vec2 fragCoord) {
    float mp = iMouse.y / iResolution.y;
    float c = gl_FragCoord.x / iResolution.x;
    out_color = mix(
    vec4(1.0, c, 0.0, 1.0),
    vec4(0.0, c, 1.0, 1.0),
    sin(iTime) * 0.5 + 0.5
    ) * (1.0 - mp);
}