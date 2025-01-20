precision highp float;
in vec4 v_color;
out vec4 out_color;
void main() {
    out_color = vec4(1.0, gl_FragCoord.x / 512.0, 0.0, 1.0);
}