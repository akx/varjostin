uniform float speed = 2.0;

void mainImage(out vec4 out_color, in vec2 fragCoord) {
    float distanceToCenter = distance(iMouse.xy, fragCoord) / max(iResolution.x, iResolution.y);
    out_color = vec4(sin(distanceToCenter * 6.282 - iTime * speed), 0, 0, 1);
    //out_color += vec4(0, sin(distanceToCenter * 7.282 - iTime * speed), 0, 1);
}