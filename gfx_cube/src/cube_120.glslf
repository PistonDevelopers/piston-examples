#version 120
varying vec2 v_TexCoord;
uniform sampler2D t_color;
void main() {
    vec4 tex = texture2D(t_color, v_TexCoord);
    float blend = dot(v_TexCoord-vec2(0.5,0.5), v_TexCoord-vec2(0.5,0.5));
    gl_FragColor = mix(tex, vec4(0.0,0.0,0.0,0.0), blend*1.0);
}
