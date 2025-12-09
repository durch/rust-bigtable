fn main() {
    protobuf_codegen::Codegen::new()
        .pure()
        .includes(["protos"])
        .input("protos/google/api/annotations.proto")
        .input("protos/google/api/client.proto")
        .input("protos/google/api/field_behavior.proto")
        .input("protos/google/api/http.proto")
        .input("protos/google/api/resource.proto")
        .input("protos/google/bigtable/v2/bigtable.proto")
        .input("protos/google/bigtable/v2/data.proto")
        .input("protos/google/rpc/status.proto")
        .cargo_out_dir("protos")
        .run_from_script();
}
