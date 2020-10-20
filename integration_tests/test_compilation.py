from pathlib import Path
from pytest import raises
from subprocess import CalledProcessError
import shutil

def test_fails_with_no_ninja_build_file(gbcli, tmp_path):
    with raises(CalledProcessError):
        gbcli(cwd=tmp_path)


def test_fails_with_empty_build_ninja_file(gbcli, tmp_path):
    ninja_build_file = tmp_path / "ninja.build"
    ninja_build_file.write_text("")
    with raises(CalledProcessError):
        gbcli(cwd=tmp_path)


def test_simple_transform(gbcli, tmp_path):
    shutil.copy(Path(__file__).parent / "build.ninja", tmp_path)
    gbcli(cwd=tmp_path)