import subprocess
from mlib.command import run_command

def gnome_compile_resources(input, source_dir):
    run_command("glib-compile-resources", f"--sourcedir={source_dir}", input)