uniform float speed = 8.0;

void mainImage(out vec4 out_color, in vec2 fragCoord) {
    float mp = iMouse.y / iResolution.y;
    float distanceToCenter = distance(iMouse.xy, fragCoord) / iResolution.y;
    out_color = vec4(sin(distanceToCenter * 6.282 - iTime * speed), 0, 0, 1);
    out_color += vec4(0, sin(distanceToCenter * 7.282 - iTime * speed), 0, 1);
}