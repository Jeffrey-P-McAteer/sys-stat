#!/usr/bin/env python3

# We have a long message string (url) that
# doesn't make sense to break up just to
# satisfy the linter.

# pylint: disable=E501

import sys
import subprocess


def cmd(*args):
    subprocess.check_call(list(args))


if __name__ == '__main__':

    cmd('cargo', 'build', '--release')

    # If the host is not running windows, build
    # a windows copy as well
    if sys.platform not in ['win32', 'cygwin']:
        print('> Attempting to cross-compile for x86_64-pc-windows-gnu systems...')
        print('> See https://wiki.archlinux.org/index.php/Rust#Cross_compiling for cross-compilation setup.')
        cmd('cargo', 'build', '--release', '--target=x86_64-pc-windows-gnu')

