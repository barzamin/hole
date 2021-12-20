struct VertexOutput {
    [[builtin(position)]] clip_pos: vec4<f32>;
};

[[stage(vertex)]]
fn vtx_main(
    [[builtin(vertex_index)]] in_vtx_idx: u32,
) -> VertexOutput {
    var out: VertexOutput;
    let x = f32(1 - i32(in_vtx_idx)) * 0.5;
    let y = f32(i32(in_vtx_idx & 1u) * 2 - 1) * 0.5;
    out.clip_pos = vec4<f32>(x, y, 0.0, 1.0);
    return out;
}

[[stage(fragment)]]
fn frag_main(
    in: VertexOutput,
) -> [[location(0)]] vec4<f32> {
    return vec4<f32>(0.3, 0.2, 0.1, 1.0);
}