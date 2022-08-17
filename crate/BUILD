load("@rules_rust//rust:defs.bzl", "rust_shared_library")

rust_shared_library(
    name = "wheel",
    srcs = ["lib.rs"],
    crate_name = "test_wheel",
    edition = "2021",
    visibility = ["//visibility:public"],
    deps = [
        "@crate_index//:pyo3",
        "@python//:libpython",
    ],
)
