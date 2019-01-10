#version 150 core

in vec2 v_UV;
in vec3 v_Normal;

out vec4 o_Color;

uniform vec3 u_view_dir;
uniform sampler2D t_color;

void main() {
  //gl_FrontFacing = (floor(dot(gs_out.v_Normal, u_view_dir)) < 0) ? true : false;

  //vec4 tex = texture(t_color, gs_out.v_UV);
  //float blend = dot(gs_in.v_UV-vec2(0.5,0.5), gs_out.v_UV-vec2(0.5,0.5));
  //o_Color = mix(tex, vec4(0.0,0.0,0.0,0.0), blend*1.0);
  o_Color = vec4(gs_out.v_UV, 0.0, 1.0);
  //o_Color = vec4(gs_out.v_Normal, 1.0);
}
