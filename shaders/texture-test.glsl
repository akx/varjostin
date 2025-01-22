#pragma @mag {range:[-10,10]}
uniform vec2 mag = vec2(0, 0);
uniform vec3 color = vec3(1);
#pragma @amp {range:[-0.1,0.1]}
uniform float amp = 0;
uniform sampler2D tex;

void mainImage(out vec4 fragColor, in vec2 fragCoord)
{
    vec2 uv = fragCoord.xy / iResolution.xy;
    vec2 c = vec2(uv.x, 1 - uv.y);
    c += sin((c.yx + iTime) * mag) * amp;
    vec4 texColor = texture(tex, c);
    float mouse_dis = distance(iMouse.xy, fragCoord);
    vec3 actual_color = mix(vec3(1), vec3(1, 0, 0), step(mouse_dis, 50));
    fragColor = vec4(actual_color * texColor.rgb, 1.0);
}