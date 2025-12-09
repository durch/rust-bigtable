fn main() {
    protobuf_codegen::Codegen::new()
        .pure()
        .includes(["protos"])
        // Google API protos
        .input("protos/google/api/annotations.proto")
        .input("protos/google/api/client.proto")
        .input("protos/google/api/field_behavior.proto")
        .input("protos/google/api/http.proto")
        .input("protos/google/api/resource.proto")
        .input("protos/google/api/routing.proto")
        // Google protobuf well-known types
        .input("protos/google/protobuf/any.proto")
        .input("protos/google/protobuf/duration.proto")
        .input("protos/google/protobuf/timestamp.proto")
        .input("protos/google/protobuf/wrappers.proto")
        // Google RPC
        .input("protos/google/rpc/status.proto")
        // Google type
        .input("protos/google/type/date.proto")
        // Bigtable v2 protos
        .input("protos/google/bigtable/v2/bigtable.proto")
        .input("protos/google/bigtable/v2/data.proto")
        .input("protos/google/bigtable/v2/request_stats.proto")
        .input("protos/google/bigtable/v2/types.proto")
        .cargo_out_dir("protos")
        .run_from_script();
}
