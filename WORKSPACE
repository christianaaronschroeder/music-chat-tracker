workspace(name = "music-chat-tracker")

load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")

http_archive(
    name = "rules_rust",
    integrity = "sha256-mUV3N2A8ORVVZbrm3O9yepAe/Kv4MD2ob9YQhB8aOI8=",
    urls = ["https://github.com/bazelbuild/rules_rust/releases/download/0.41.1/rules_rust-v0.41.1.tar.gz"],
)

load("@rules_rust//rust:repositories.bzl", "rules_rust_dependencies", "rust_register_toolchains")

rules_rust_dependencies()

rust_register_toolchains(edition = "2021")