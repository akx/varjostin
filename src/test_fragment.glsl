void mainImage(out vec4 out_color, in vec2 fragCoord) {
    out_color = mix(vec4(0, 0, 0, 1), vec4(1), (fragCoord.y / iResolution.y));
}