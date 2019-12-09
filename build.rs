extern crate protobuf_codegen_pure;

fn main() {
  protobuf_codegen_pure::run(protobuf_codegen_pure::Args {
    out_dir: "src/protos",
    input: &[
      "protos/google/bigtable/v2/bigtable.proto",
      "protos/google/bigtable/v2/data.proto",
      "protos/google/rpc/status.proto"
    ],
    includes: &["protos"],
    customize: protobuf_codegen_pure::Customize {
      ..Default::default()
    },
  })
  .expect("protoc");
}
