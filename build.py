#!/usr/bin/env python3

import os, sys, subprocess

def cmd(*args):
  subprocess.check_call([x for x in args])

if __name__ == '__main__':
  cmd('cargo', 'build', '--release')
  # If the host is not running windows, build
  # a windows copy as well
  if not sys.platform in ['win32', 'cygwin']:
    cmd('cargo', 'build', '--release', '--target=x86_64-pc-windows-gnu')
