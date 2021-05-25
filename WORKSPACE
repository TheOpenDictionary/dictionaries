load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")

RULES_PYTHON_VERSION = "0.2.0"

http_archive(
    name = "rules_python",
    sha256 = "778197e26c5fbeb07ac2a2c5ae405b30f6cb7ad1f5510ea6fdac03bded96cc6f",
    url = "https://github.com/bazelbuild/rules_python/releases/download/%s/rules_python-%s.tar.gz" % (RULES_PYTHON_VERSION, RULES_PYTHON_VERSION),
)

load("@rules_python//python:pip.bzl", "pip_install")

pip_install(
    name = "deps",
    requirements = "//:requirements.txt",
)

http_archive(
    name = "odict",
    sha256 = "50ef737cce963025f4076bcb52f2c82fc64c1ab8b53747e80fa36a382c696a44",
    strip_prefix = "odict-1.4.5",
    url = "https://github.com/TheOpenDictionary/odict/archive/1.4.5/odict-1.4.5.tar.gz",
)

load("@odict//bazel:odict_deps.bzl", "odict_deps")

odict_deps()

load("@odict//bazel:odict_extra_deps.bzl", "odict_extra_deps")

odict_extra_deps()
