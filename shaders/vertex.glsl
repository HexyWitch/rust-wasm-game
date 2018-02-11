#version 100
attribute highp vec2 position;
attribute highp vec2 tex_coord;
attribute mediump vec4 color;

uniform vec2 screen_size;

varying vec2 v_tex_coord;
varying mediump vec4 v_color;

void main()
{
    gl_Position = vec4((position / screen_size * 2.0) - vec2(1.0, 1.0), 0.0, 1.0);
    v_color = color;
    v_tex_coord = tex_coord;
}
