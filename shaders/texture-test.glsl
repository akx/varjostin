#pragma @center {range:[-2,2]}
uniform vec2 center = vec2(0, 0);
uniform vec3 color = vec3(1);
uniform sampler2D tex;

void mainImage(out vec4 fragColor, in vec2 fragCoord)
{
    vec2 uv = fragCoord.xy / iResolution.xy;
    vec4 texColor = texture(tex, vec2(uv.x, 1 - uv.y));
    fragColor = vec4(texColor.rgb * color, 1.0);
}