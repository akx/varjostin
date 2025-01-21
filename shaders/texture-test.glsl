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
    float dis = distance(uv, vec2(0.5, 0.5));
    c += sin((c.yx + iTime) * mag) * amp;
    vec4 texColor = texture(tex, c);
    fragColor = vec4(texColor.rgb * color, 1.0);
}