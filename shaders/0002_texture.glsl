#pragma @mag {range:[-10,10]}
uniform vec2 mag = vec2(0, 0);
uniform vec3 color = vec3(1);
#pragma @amp {range:[-0.1,0.1]}
uniform float amp = 0;
uniform sampler2D tex1;
uniform sampler2D tex2;

void mainImage(out vec4 fragColor, in vec2 fragCoord)
{
    vec2 uv = fragCoord.xy / iResolution.xy;
    vec2 c = vec2(uv.x, 1 - uv.y);
    c += sin((c.yx + iTime) * mag) * amp;
    vec4 bgColor = vec4(0.8, 0.5, 0.8, 1);
    vec4 texColor = texture(tex1, c);// + texture(tex2, c);
    // texColor = mix(bgColor, texColor, texColor.a); // <-- PSST, IT'S THIS
    float mouse_dis = distance(iMouse.xy, fragCoord);
    vec3 actual_color = mix(vec3(1), vec3(1, 0, 0), step(mouse_dis, 50 + iMouse.z * 50));
    fragColor = vec4(actual_color * texColor.rgb, 1.0);
}