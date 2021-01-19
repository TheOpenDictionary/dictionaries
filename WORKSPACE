load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")

RULES_PYTHON_VERSION = "0.1.0"

http_archive(
    name = "rules_python",
    sha256 = "b6d46438523a3ec0f3cead544190ee13223a52f6a6765a29eae7b7cc24cc83a0",
    url = "https://github.com/bazelbuild/rules_python/releases/download/%s/rules_python-%s.tar.gz" % (RULES_PYTHON_VERSION, RULES_PYTHON_VERSION),
)

load("@rules_python//python:pip.bzl", "pip_install")

pip_install(
    name = "deps",
    requirements = "//:requirements.txt",
)

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
