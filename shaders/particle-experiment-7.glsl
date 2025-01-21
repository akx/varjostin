// Via https://www.shadertoy.com/view/MddGWN
// Created by Stephane Cuillerdier - Aiekick/2015
// License Creative Commons Attribution-NonCommercial-ShareAlike 3.0 Unported License.
// Tuned via XShade (http://www.funparadigm.com/xshade/)

#pragma @n {range:[0,100]}
uniform int n = 100;
#pragma @center {range:[-2,2]}
uniform vec2 center = vec2(0, 0);
uniform vec3 startColor = vec3(0, 0.64, 0.2);
uniform vec4 endColor = vec4(0.06, 0.35, 0.85, 0.0);
const int xyzy = 1351;

void mainImage(out vec4 fragColor, in vec2 fragCoord)
{
    float t = iTime+5.;
    float z = 6.;

    float startRadius = 0.84;
    float endRadius = 1.6;

    float power = 0.51;
    float duration = 3.;

    vec2 s = iResolution.xy;
    vec2 v = z*(2.*gl_FragCoord.xy-s)/s.y - center;

    // Mouse axis y => zoom
    if (iMouse.z>0.) v *= iMouse.y/s.y * 20.;

    // Mouse axis x => duration
    if (iMouse.z>0.) duration = iMouse.x/s.x * 10.;

    vec4 col = vec4(0.);

    vec2 pm = v.yx*2.8;

    float dMax = duration;


    float evo = (sin(iTime*.01+400.)*.5+.5)*99.+1.;

    float mb = 0.;
    float mbRadius = 0.;
    float sum = 0.;
    for (int i=0;i<n;i++)
    {
        float d = fract(t*power+48934.4238*sin(float(i/int(evo))*692.7398));

        float tt = 0.;

        float a = 6.28*float(i)/float(n);

        float x = d*cos(a)*duration;

        float y = d*sin(a)*duration;

        float distRatio = d/dMax;

        mbRadius = mix(startRadius, endRadius, distRatio);

        vec2 p = v - vec2(x, y);//*vec2(1,sin(a+3.14159/2.));

        mb = mbRadius/dot(p, p);

        sum += mb;

        col = mix(col, mix(vec4(startColor, 1.0), endColor, distRatio), mb/sum);
    }

    sum /= float(n);

    col = normalize(col) * sum;

    sum = clamp(sum, 0., .8);

    fragColor.rgb = col.rgb;
}