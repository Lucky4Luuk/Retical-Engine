#version 330 core
layout (points) in;
layout (triangle_strip, max_vertices = 36) out;

in VS_OUT {
  vec2 v_TexCoord;
} gs_in[];

out vec2 v_UV;
out vec3 v_Normal;

uniform mat4 u_model_view_proj;

void main() {
  //v_TexCoord = gs_in[0].v_TexCoord;

  //Set up some variables to quickly use
  vec3 FRONT_NORMAL  = vec3( 0.0, 0.0, 1.0);
  vec3 BACK_NORMAL   = vec3( 0.0, 0.0,-1.0);
  vec3 LEFT_NORMAL   = vec3(-1.0, 0.0, 0.0);
  vec3 RIGHT_NORMAL  = vec3( 1.0, 0.0, 0.0);
  vec3 TOP_NORMAL    = vec3( 0.0, 1.0, 0.0);
  vec3 BOTTOM_NORMAL = vec3( 0.0,-1.0, 0.0);

  //Front
  gl_Position = gl_in[0].gl_Position + u_model_view_proj * vec4(-0.5,-0.5, 0.5, 1.0);
  v_Normal = FRONT_NORMAL;
  v_UV = vec2(1.0, 1.0);
  EmitVertex();
  gl_Position = gl_in[0].gl_Position + u_model_view_proj * vec4( 0.5,-0.5, 0.5, 1.0);
  v_Normal = FRONT_NORMAL;
  v_UV = vec2(0.0, 1.0);
  EmitVertex();
  gl_Position = gl_in[0].gl_Position + u_model_view_proj * vec4( 0.5, 0.5, 0.5, 1.0);
  v_Normal = FRONT_NORMAL;
  v_UV = vec2(0.0, 0.0);
  EmitVertex();
  EndPrimitive();

  gl_Position = gl_in[0].gl_Position + u_model_view_proj * vec4( 0.5, 0.5, 0.5, 1.0);
  v_Normal = FRONT_NORMAL;
  v_UV = vec2(0.0, 0.0);
  EmitVertex();
  gl_Position = gl_in[0].gl_Position + u_model_view_proj * vec4(-0.5, 0.5, 0.5, 1.0);
  v_Normal = FRONT_NORMAL;
  v_UV = vec2(1.0, 0.0);
  EmitVertex();
  gl_Position = gl_in[0].gl_Position + u_model_view_proj * vec4(-0.5,-0.5, 0.5, 1.0);
  v_Normal = FRONT_NORMAL;
  v_UV = vec2(1.0, 1.0);
  EmitVertex();
  EndPrimitive();

  //Back
  gl_Position = gl_in[0].gl_Position + u_model_view_proj * vec4( 0.5, 0.5,-0.5, 1.0);
  v_Normal = BACK_NORMAL;
  v_UV = vec2(0.0, 0.0);
  EmitVertex();
  gl_Position = gl_in[0].gl_Position + u_model_view_proj * vec4( 0.5,-0.5,-0.5, 1.0);
  v_Normal = BACK_NORMAL;
  v_UV = vec2(0.0, 1.0);
  EmitVertex();
  gl_Position = gl_in[0].gl_Position + u_model_view_proj * vec4(-0.5,-0.5,-0.5, 1.0);
  v_Normal = BACK_NORMAL;
  v_UV = vec2(1.0, 1.0);
  EmitVertex();
  EndPrimitive();

  gl_Position = gl_in[0].gl_Position + u_model_view_proj * vec4(-0.5,-0.5,-0.5, 1.0);
  v_Normal = BACK_NORMAL;
  v_UV = vec2(1.0, 1.0);
  EmitVertex();
  gl_Position = gl_in[0].gl_Position + u_model_view_proj * vec4(-0.5, 0.5,-0.5, 1.0);
  v_Normal = BACK_NORMAL;
  v_UV = vec2(1.0, 0.0);
  EmitVertex();
  gl_Position = gl_in[0].gl_Position + u_model_view_proj * vec4( 0.5, 0.5,-0.5, 1.0);
  v_Normal = BACK_NORMAL;
  v_UV = vec2(0.0, 0.0);
  EmitVertex();
  EndPrimitive();

  //Left
  gl_Position = gl_in[0].gl_Position + u_model_view_proj * vec4(-0.5,-0.5,-0.5, 1.0);
  v_Normal = LEFT_NORMAL;
  v_UV = vec2(0.0, 1.0);
  EmitVertex();
  gl_Position = gl_in[0].gl_Position + u_model_view_proj * vec4(-0.5,-0.5, 0.5, 1.0);
  v_Normal = LEFT_NORMAL;
  v_UV = vec2(1.0, 1.0);
  EmitVertex();
  gl_Position = gl_in[0].gl_Position + u_model_view_proj * vec4(-0.5, 0.5, 0.5, 1.0);
  v_Normal = LEFT_NORMAL;
  v_UV = vec2(1.0, 0.0);
  EmitVertex();
  EndPrimitive();

  gl_Position = gl_in[0].gl_Position + u_model_view_proj * vec4(-0.5, 0.5, 0.5, 1.0);
  v_Normal = LEFT_NORMAL;
  v_UV = vec2(1.0, 0.0);
  EmitVertex();
  gl_Position = gl_in[0].gl_Position + u_model_view_proj * vec4(-0.5, 0.5,-0.5, 1.0);
  v_Normal = LEFT_NORMAL;
  v_UV = vec2(0.0, 0.0);
  EmitVertex();
  gl_Position = gl_in[0].gl_Position + u_model_view_proj * vec4(-0.5,-0.5,-0.5, 1.0);
  v_Normal = LEFT_NORMAL;
  v_UV = vec2(0.0, 1.0);
  EmitVertex();
  EndPrimitive();

  //Right
  gl_Position = gl_in[0].gl_Position + u_model_view_proj * vec4( 0.5, 0.5, 0.5, 1.0);
  v_Normal = RIGHT_NORMAL;
  v_UV = vec2(1.0, 0.0);
  EmitVertex();
  gl_Position = gl_in[0].gl_Position + u_model_view_proj * vec4( 0.5,-0.5, 0.5, 1.0);
  v_Normal = RIGHT_NORMAL;
  v_UV = vec2(1.0, 1.0);
  EmitVertex();
  gl_Position = gl_in[0].gl_Position + u_model_view_proj * vec4( 0.5,-0.5,-0.5, 1.0);
  v_Normal = RIGHT_NORMAL;
  v_UV = vec2(0.0, 1.0);
  EmitVertex();
  EndPrimitive();

  gl_Position = gl_in[0].gl_Position + u_model_view_proj * vec4( 0.5,-0.5,-0.5, 1.0);
  v_Normal = RIGHT_NORMAL;
  v_UV = vec2(0.0, 1.0);
  EmitVertex();
  gl_Position = gl_in[0].gl_Position + u_model_view_proj * vec4( 0.5, 0.5,-0.5, 1.0);
  v_Normal = RIGHT_NORMAL;
  v_UV = vec2(0.0, 0.0);
  EmitVertex();
  gl_Position = gl_in[0].gl_Position + u_model_view_proj * vec4( 0.5, 0.5, 0.5, 1.0);
  v_Normal = RIGHT_NORMAL;
  v_UV = vec2(1.0, 0.0);
  EmitVertex();
  EndPrimitive();

  //Top
  gl_Position = gl_in[0].gl_Position + u_model_view_proj * vec4(-0.5, 0.5,-0.5, 1.0);
  v_Normal = TOP_NORMAL;
  v_UV = vec2(0.0, 0.0);
  EmitVertex();
  gl_Position = gl_in[0].gl_Position + u_model_view_proj * vec4(-0.5, 0.5, 0.5, 1.0);
  v_Normal = TOP_NORMAL;
  v_UV = vec2(0.0, 1.0);
  EmitVertex();
  gl_Position = gl_in[0].gl_Position + u_model_view_proj * vec4( 0.5, 0.5, 0.5, 1.0);
  v_Normal = TOP_NORMAL;
  v_UV = vec2(1.0, 1.0);
  EmitVertex();
  EndPrimitive();

  gl_Position = gl_in[0].gl_Position + u_model_view_proj * vec4( 0.5, 0.5, 0.5, 1.0);
  v_Normal = TOP_NORMAL;
  v_UV = vec2(1.0, 1.0);
  EmitVertex();
  gl_Position = gl_in[0].gl_Position + u_model_view_proj * vec4( 0.5, 0.5,-0.5, 1.0);
  v_Normal = TOP_NORMAL;
  v_UV = vec2(1.0, 0.0);
  EmitVertex();
  gl_Position = gl_in[0].gl_Position + u_model_view_proj * vec4(-0.5, 0.5,-0.5, 1.0);
  v_Normal = TOP_NORMAL;
  v_UV = vec2(0.0, 0.0);
  EmitVertex();
  EndPrimitive();

  //Bottom
  gl_Position = gl_in[0].gl_Position + u_model_view_proj * vec4( 0.5,-0.5, 0.5, 1.0);
  v_Normal = BOTTOM_NORMAL;
  v_UV = vec2(1.0, 1.0);
  EmitVertex();
  gl_Position = gl_in[0].gl_Position + u_model_view_proj * vec4(-0.5,-0.5, 0.5, 1.0);
  v_Normal = BOTTOM_NORMAL;
  v_UV = vec2(0.0, 1.0);
  EmitVertex();
  gl_Position = gl_in[0].gl_Position + u_model_view_proj * vec4(-0.5,-0.5,-0.5, 1.0);
  v_Normal = BOTTOM_NORMAL;
  v_UV = vec2(0.0, 0.0);
  EmitVertex();
  EndPrimitive();

  gl_Position = gl_in[0].gl_Position + u_model_view_proj * vec4(-0.5,-0.5,-0.5, 1.0);
  v_Normal = BOTTOM_NORMAL;
  v_UV = vec2(0.0, 0.0);
  EmitVertex();
  gl_Position = gl_in[0].gl_Position + u_model_view_proj * vec4( 0.5,-0.5,-0.5, 1.0);
  v_Normal = BOTTOM_NORMAL;
  v_UV = vec2(1.0, 0.0);
  EmitVertex();
  gl_Position = gl_in[0].gl_Position + u_model_view_proj * vec4( 0.5,-0.5, 0.5, 1.0);
  v_Normal = BOTTOM_NORMAL;
  v_UV = vec2(1.0, 1.0);
  EmitVertex();
  EndPrimitive();
}

/*
let vertex_data = vec![
      //top (0, 0, 1)
      Vertex::new([-1, -1,  1], [0, 0]),
      Vertex::new([ 1, -1,  1], [1, 0]),
      Vertex::new([ 1,  1,  1], [1, 1]),
      Vertex::new([-1,  1,  1], [0, 1]),
      //bottom (0, 0, -1)
      Vertex::new([ 1,  1, -1], [0, 0]),
      Vertex::new([-1,  1, -1], [1, 0]),
      Vertex::new([-1, -1, -1], [1, 1]),
      Vertex::new([ 1, -1, -1], [0, 1]),
      //right (1, 0, 0)
      Vertex::new([ 1, -1, -1], [0, 0]),
      Vertex::new([ 1,  1, -1], [1, 0]),
      Vertex::new([ 1,  1,  1], [1, 1]),
      Vertex::new([ 1, -1,  1], [0, 1]),
      //left (-1, 0, 0)
      Vertex::new([-1,  1,  1], [0, 0]),
      Vertex::new([-1, -1,  1], [1, 0]),
      Vertex::new([-1, -1, -1], [1, 1]),
      Vertex::new([-1,  1, -1], [0, 1]),
      //front (0, 1, 0)
      Vertex::new([-1,  1, -1], [0, 0]),
      Vertex::new([ 1,  1, -1], [1, 0]),
      Vertex::new([ 1,  1,  1], [1, 1]),
      Vertex::new([-1,  1,  1], [0, 1]),
      //back (0, -1, 0)
      Vertex::new([ 1, -1,  1], [0, 0]),
      Vertex::new([-1, -1,  1], [1, 0]),
      Vertex::new([-1, -1, -1], [1, 1]),
      Vertex::new([ 1, -1, -1], [0, 1]),
  ];
*/
