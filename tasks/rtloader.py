"""
RtLoader namespaced tasks
"""

import errno
import os
import shutil
import sys

from invoke import task
from invoke.exceptions import Exit


def get_rtloader_path():
    here = os.path.abspath(os.path.dirname(__file__))
    return os.path.abspath(os.path.join(here, '..', 'rtloader'))


def get_rtloader_build_path():
    return os.path.join(get_rtloader_path(), 'build')


def get_dev_path():
    here = os.path.abspath(os.path.dirname(__file__))
    return os.path.abspath(os.path.join(here, '..', 'dev'))


def run_make_command(ctx, command=""):
    ctx.run(f"make -C {get_rtloader_build_path()} {command}")


def get_cmake_cache_path(rtloader_path):
    return os.path.join(rtloader_path, "CMakeCache.txt")


def clear_cmake_cache(rtloader_path, settings):
    """
    CMake is not regenerated when we change an option. This function detect the
    current cmake settings and remove the cache if they have change to retrigger
    a cmake build.
    """
    cmake_cache = get_cmake_cache_path(rtloader_path)
    if not os.path.exists(cmake_cache):
        return

    settings_not_found = settings.copy()
    with open(cmake_cache) as cache:
        for line in cache.readlines():
            for key, value in settings.items():
                if line.strip() == key + "=" + value:
                    settings_not_found.pop(key)

    if settings_not_found:
        os.remove(cmake_cache)


@task
def make(ctx, install_prefix=None, python_runtimes='3', cmake_options='', arch="x64"):
    dev_path = get_dev_path()

    if cmake_options.find("-G") == -1:
        cmake_options += " -G \"Unix Makefiles\""

    cmake_args = cmake_options + f" -DBUILD_DEMO:BOOL=OFF -DCMAKE_INSTALL_PREFIX:PATH={install_prefix or dev_path}"

    python_runtimes = python_runtimes.split(',')

    settings = {
        "DISABLE_PYTHON2:BOOL": "OFF",
        "DISABLE_PYTHON3:BOOL": "OFF",
    }
    if '2' not in python_runtimes:
        settings["DISABLE_PYTHON2:BOOL"] = "ON"
    if '3' not in python_runtimes:
        settings["DISABLE_PYTHON3:BOOL"] = "ON"

    rtloader_build_path = get_rtloader_build_path()

    # clear cmake cache if settings have changed since the last build
    clear_cmake_cache(rtloader_build_path, settings)

    for option, value in settings.items():
        cmake_args += f" -D{option}={value} "

    if arch == "x86":
        cmake_args += " -DARCH_I386=ON"

    if sys.platform == 'darwin':
        cmake_args += " -DCMAKE_OSX_DEPLOYMENT_TARGET=10.13"

    # Perform "out of the source build" in `rtloader_build_path` folder.
    try:
        os.makedirs(rtloader_build_path)
    except OSError as e:
        if e.errno == errno.EEXIST:
            pass
        else:
            raise

    ctx.run(f"cd {rtloader_build_path} && cmake {cmake_args} {get_rtloader_path()}")
    run_make_command(ctx)


@task
def clean(_):
    """
    Clean up CMake's cache.
    Necessary when the paths to some libraries found by CMake (for example Python) have changed on the system.
    """
    dev_path = get_dev_path()
    include_path = os.path.join(dev_path, "include")
    lib_path = os.path.join(dev_path, "lib")
    rtloader_build_path = get_rtloader_build_path()

    for p in [include_path, lib_path, rtloader_build_path]:
        try:
            shutil.rmtree(p)
            print(f"Successfully cleaned '{p}'")
        except FileNotFoundError:
            print(f"Nothing to clean up '{p}'")


@task
def install(ctx):
    run_make_command(ctx, "install")


@task
def test(ctx):
    ctx.run(f"make -C {get_rtloader_build_path()}/test run")


@task
def format(ctx, raise_if_changed=False):
    run_make_command(ctx, "clang-format")

    if raise_if_changed:
        changed_files = [line for line in ctx.run("git ls-files -m rtloader").stdout.strip().split("\n") if line]
        if len(changed_files) != 0:
            print("Following files were not correctly formated:")
            for f in changed_files:
                print(f"  - {f}")
            raise Exit(code=1)


@task
def generate_doc(ctx):
    """
    Generates the doxygen documentation, puts it in rtloader/doc, and logs doc errors/warnings.
    (rtloader/doc is hardcoded right now in the Doxyfile, as doxygen cannot take the output directory as argument)
    Logs all errors and warnings to <rtloader_path>/doxygen/errors.log and to the standard output.
    Returns 1 if errors were found (by default, doxygen returns 0 even if errors are present).
    """
    rtloader_path = get_rtloader_path()

    # Clean up Doxyfile options that are not supported on the version of Doxygen used
    result = ctx.run(f"doxygen -u '{rtloader_path}/doxygen/Doxyfile'", warn=True)
    if result.exited != 0:
        print("Fatal error encountered while trying to clean up the Doxyfile.")
        raise Exit(code=result.exited)

    # doxygen puts both errors and warnings in stderr
    result = ctx.run(
        "doxygen '{0}/doxygen/Doxyfile' 2>'{0}/doxygen/errors.log'".format(rtloader_path),  # noqa: UP032
        warn=True,
    )

    if result.exited != 0:
        print("Fatal error encountered while trying to generate documentation.")
        print(f"See {rtloader_path}/doxygen/errors.log for details.")
        raise Exit(code=result.exited)

    errors, warnings = [], []

    def flushentry(entry):
        if 'error:' in entry:
            errors.append(entry)
        elif 'warning:' in entry:
            warnings.append(entry)

    # Separate warnings from errors
    with open(f"{rtloader_path}/doxygen/errors.log") as errfile:
        currententry = ""
        for line in errfile.readlines():
            if 'error:' in line or 'warning:' in line:  # We get to a new entry, flush current one
                flushentry(currententry)
                currententry = ""

            currententry += line

        flushentry(currententry)  # Flush last entry

        print("\033[93m{}\033[0m".format("\n".join(warnings)))  # noqa: FS002
        print("\033[91m{}\033[0m".format("\n".join(errors)))  # noqa: FS002
        print(f"Found {len(errors)} error(s) and {len(warnings)} warning(s) while generating documentation.")
        print(f"The full list is available in {rtloader_path}/doxygen/errors.log.")

    # Exit with non-zero code if an error has been found
    if len(errors) > 0:
        raise Exit(code=1)
