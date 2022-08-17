load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")

http_archive(
    name = "rules_python",
    sha256 = "c03246c11efd49266e8e41e12931090b613e12a59e6f55ba2efd29a7cb8b4258",
    strip_prefix = "rules_python-0.11.0",
    url = "https://github.com/bazelbuild/rules_python/archive/refs/tags/0.11.0.tar.gz",
)

http_archive(
    name = "rules_rust",
    sha256 = "6bfe75125e74155955d8a9854a8811365e6c0f3d33ed700bc17f39e32522c822",
    urls = [
        "https://mirror.bazel.build/github.com/bazelbuild/rules_rust/releases/download/0.9.0/rules_rust-v0.9.0.tar.gz",
        "https://github.com/bazelbuild/rules_rust/releases/download/0.9.0/rules_rust-v0.9.0.tar.gz",
    ],
)

load("@rules_rust//rust:repositories.bzl", "rules_rust_dependencies", "rust_register_toolchains", "rust_repositories")

rules_rust_dependencies()

rust_register_toolchains(
    edition = "2021",
    include_rustc_srcs = True,
    version = "1.62.1",
)

load("@rules_rust//crate_universe:repositories.bzl", "crate_universe_dependencies")

crate_universe_dependencies()
## Install python

load("@rules_python//python:repositories.bzl", "python_register_toolchains")

python_register_toolchains(
    name = "python",
    # Available versions are listed in @rules_python//python:versions.bzl.
    # We recommend using the same version your team is already standardized on.
    python_version = "3.9",
)

load("@rules_rust//crate_universe:defs.bzl", "crate", "crates_repository")

crates_repository(
    name = "crate_index",
    annotations = {
        "pyo3": [crate.annotation(
            gen_build_script = False,
            rustc_flags = [
                "--cfg=Py_3_7",
                "--cfg=Py_3_8",
                "--cfg=Py_3_9",
                "--cfg=py_sys_config=\\\"WITH_THREAD\\\"",
            ],
            deps = ["@python//:libpython"],
        )],
    },
    cargo_lockfile = "//:Cargo.bazel.lock",
    lockfile = "//:cargo-bazel-lock.json",
    packages = {
        "pyo3": crate.spec(
            features = [
                "macros",
                "extension-module",
            ],
            version = "0.15",
        ),
    },
)

load("@crate_index//:defs.bzl", "crate_repositories")

crate_repositories()
