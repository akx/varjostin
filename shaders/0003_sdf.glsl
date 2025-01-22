// Based on https://www.shadertoy.com/view/4llXD7
// The MIT License, Copyright © 2015 Inigo Quilez

#pragma @radius {range:[0,1]}
uniform float radius = 0.5;
#pragma @offs {range:[-1,1]}
uniform vec2 offs = vec2(0, 0);

float sdRoundBox(in vec2 p, in vec2 b, in vec4 r)
{
    r.xy = (p.x>0.0)?r.xy : r.zw;
    r.x  = (p.y>0.0)?r.x  : r.y;
    vec2 q = abs(p)-b+r.x;
    return min(max(q.x, q.y), 0.0) + length(max(q, 0.0)) - r.x;
}

/*
float repeated( vec3 p )
{
    p.x = p.x - round(p.x);
    return sdf(p);
}
*/


void mainImage(out vec4 fragColor, in vec2 fragCoord)
{
    vec2 p = (2.0*fragCoord-iResolution.xy)/iResolution.y + offs;

    vec2 si = vec2(0.9, 0.6) + 0.3*cos(iTime+vec2(0, 2));
    vec4 ra = vec4(0, 1, 2, 3) * radius * (0.5 + cos(iTime) * 0.5);
    ra = min(ra, min(si.x, si.y));

    float d = sdRoundBox(p, si, ra);

    vec3 col = (d>0.0) ? vec3(0.1, 0.1, 0.1) : vec3(0.9, 0.7, 0.3);
    col *= 1.0 - exp(-6.0*abs(d));
    col *= 0.8 + 0.2*cos(150.0*d);
    col = mix(col, vec3(1.0), 1.0-smoothstep(0.0, 0.01, abs(d)));

    fragColor = vec4(col, 1.0);
}