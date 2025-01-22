precision highp float;

uniform vec4 iViewport;
uniform vec3 iResolution;
uniform float iTime;
uniform int iFrame;
uniform vec4 iMouse;

out vec4 shadertoy_out_color;

void mainImage(out vec4 fragColor, in vec2 fragCoord);

void main() {
    shadertoy_out_color = vec4(1.0, 1.0, 1.0, 1.0);
    vec4 color = vec4(1e20);
    vec2 adjCoord = (gl_FragCoord.xy - iViewport.xy);
    mainImage(color, adjCoord.xy);
    if (shadertoy_out_color.x<0.0) color=vec4(1.0, 0.0, 0.0, 1.0);
    if (shadertoy_out_color.y<0.0) color=vec4(0.0, 1.0, 0.0, 1.0);
    if (shadertoy_out_color.z<0.0) color=vec4(0.0, 0.0, 1.0, 1.0);
    if (shadertoy_out_color.w<0.0) color=vec4(1.0, 1.0, 0.0, 1.0);
    shadertoy_out_color = vec4(color.xyz, 1.0);
}
