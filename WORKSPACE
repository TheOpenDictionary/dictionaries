load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive", "http_file")

http_archive(
    name = "rules_python",
    sha256 = "b5668cde8bb6e3515057ef465a35ad712214962f0b3a314e551204266c7be90c",
    strip_prefix = "rules_python-0.0.2",
    url = "https://github.com/bazelbuild/rules_python/releases/download/0.0.2/rules_python-0.0.2.tar.gz",
)

load("@rules_python//python:repositories.bzl", "py_repositories")

py_repositories()

load("@rules_python//python:pip.bzl", "pip3_import", "pip_repositories")

pip_repositories()

pip3_import(
    name = "deps",
    requirements = "//:requirements.txt",
)

load("@deps//:requirements.bzl", "pip_install")

pip_install()

# load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_file")

http_file(
    name = "odict_so",
    urls = ["https://github.com/odict/odict/releases/download/v1.0/odict.so"],
)

http_file(
    name = "odict_h",
    urls = ["https://github.com/odict/odict/releases/download/v1.0/odict.h"],
)
