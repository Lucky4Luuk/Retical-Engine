#version 330 core
layout (points) in;
layout (triangle_strip, max_vertices = 36) out;

in VS_OUT {
  vec2 v_TexCoord;
} gs_in[];

out vec2 v_TexCoord;

uniform mat4 u_model_view_proj;

void main() {
  v_TexCoord = gs_in[0].v_TexCoord;
  gl_Position = gl_in[0].gl_Position + u_model_view_proj * vec4(-0.5,-0.5, 0.5, 1.0);
  EmitVertex();
  gl_Position = gl_in[0].gl_Position + u_model_view_proj * vec4( 0.5,-0.5, 0.5, 1.0);
  EmitVertex();
  gl_Position = gl_in[0].gl_Position + u_model_view_proj * vec4( 0.5, 0.5, 0.5, 1.0);
  EmitVertex();
  EndPrimitive();

  gl_Position = gl_in[0].gl_Position + u_model_view_proj * vec4( 0.5, 0.5, 0.5, 1.0);
  EmitVertex();
  gl_Position = gl_in[0].gl_Position + u_model_view_proj * vec4(-0.5, 0.5, 0.5, 1.0);
  EmitVertex();
  gl_Position = gl_in[0].gl_Position + u_model_view_proj * vec4(-0.5,-0.5, 0.5, 1.0);
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
