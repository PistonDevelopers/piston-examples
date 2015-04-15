#version 120
attribute vec3 a_pos;
attribute vec2 a_tex_coord;
varying vec2 v_TexCoord;
uniform mat4 u_model_view_proj;
void main() {
    v_TexCoord = a_tex_coord;
    gl_Position = u_model_view_proj * vec4(a_pos, 1.0);
}
