uniform float speed = 2.0;
#pragma @zoom {range:[1, 50]}
uniform float zoom = 1;
#pragma @whoa {range:[-500, 500]}
uniform float whoa = 150;
#pragma @whoa2 {range:[-500, 500]}
uniform float whoa2 = 150;

#pragma @n {range:[1, 30]}
uniform int n = 8;

uniform sampler2D tex;

vec3 hsv2rgb(vec3 c) {
    vec4 K = vec4(1.0, 2.0 / 3.0, 1.0 / 3.0, 3.0);
    vec3 p = abs(fract(c.xxx + K.xyz) * 6.0 - K.www);
    return c.z * mix(K.xxx, clamp(p - K.xxx, 0.0, 1.0), c.y);
}


void mainImage(out vec4 out_color, in vec2 fragCoord) {
    vec3 color_rgb = vec3(0, 0, 0);
    for (int i = 0; i < n; i++) {
        float q = float(i) / float(n - 1);
        float ang = q * 6.282;
        float xd = cos(ang) * whoa2;
        float yd = sin(ang) * whoa2;
        vec2 m = iMouse.xy + vec2(xd, yd);

        float ds = distance(m, fragCoord) / max(iResolution.x, iResolution.y);
        ds = sqrt(ds);
        float mag = 0.5 + sin(ds * zoom * 6.282 - iTime * speed) * 0.5;
        vec3 hsv = vec3(q * 0.65 + iTime * 0.2, 1., 1.);
        color_rgb += vec3(hsv2rgb(hsv) * mag);
    }
    color_rgb /= n;
    //    out_color = vec4(color_rgb / n, 1);
    vec2 sc = fragCoord / iResolution.xy;
    sc = vec2(1, 1)-sc;
    vec3 tex_color = texture(tex, sc + (vec2(color_rgb.x, color_rgb.y) - 0.5) * 0.2).rgb;
    out_color = vec4(tex_color * color_rgb * 2, 1);
    //    out_color += color_rgb * 0.1;

    //    float distanceToCenter2 = distance(iMouse.xy - vec2(whoa * sin(iTime * 10), 0), fragCoord) / max(iResolution.x, iResolution.y);
    //    out_color += vec4(0, sin(distanceToCenter2 *zoom *  7.282 - iTime * speed), 0, 1);
}