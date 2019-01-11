#version 150 core

in vec2 v_UV;
in vec3 v_Normal;
in vec4 v_Atlas_UV;

out vec4 o_Color;

uniform vec3 u_view_dir;
uniform sampler2D t_color;

vec3 light_dir = normalize(vec3(-0.5, -1.0, -0.5));

void main() {
  //gl_FrontFacing = (floor(dot(gs_out.v_Normal, u_view_dir)) < 0) ? true : false;

  //vec4 tex = texture(t_color, gs_out.v_UV);
  //float blend = dot(gs_in.v_UV-vec2(0.5,0.5), gs_out.v_UV-vec2(0.5,0.5));
  //o_Color = mix(tex, vec4(0.0,0.0,0.0,0.0), blend*1.0);
  //o_Color = vec4(v_UV, 0.0, 1.0);
  //o_Color = vec4(gs_out.v_Normal, 1.0);
  vec4 tex_col = texture(t_color, v_Atlas_UV.xy + v_Atlas_UV.zw * v_UV);
  float attenuation = 1.0 - clamp(dot(light_dir, v_Normal), 0.0, 1.0);
  o_Color = tex_col * attenuation;
}
