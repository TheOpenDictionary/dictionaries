load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")

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

http_archive(
    name = "odict",
    sha256 = "81bab611bcf93b6b8baac2e11fa6a5aa7ca907509bd5ad654fe58c2bfcafac1d",
    strip_prefix = "odict-1.4",
    url = "https://github.com/TheOpenDictionary/odict/archive/1.4/odict-1.4.tar.gz",
)

load("@odict//bazel:odict_deps.bzl", "odict_deps")

odict_deps()

load("@odict//bazel:odict_extra_deps.bzl", "odict_extra_deps")

odict_extra_deps()
