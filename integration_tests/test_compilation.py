from pytest import raises
from subprocess import CalledProcessError
import ninja_syntax
import pytest

def test_fails_with_no_ninja_build_file(gbcli, tmp_path):
    with raises(CalledProcessError):
        gbcli(cwd=tmp_path)


def test_fails_with_empty_build_ninja_file(gbcli, tmp_path):
    ninja_build_file = tmp_path / "ninja.build"
    ninja_build_file.write_text("")
    with raises(CalledProcessError):
        gbcli(cwd=tmp_path)


@pytest.mark.xfail(reason="#1 Not implemented yet")
def test_fails_input_file_missing(gbcli, tmp_path):
    build_file_path = tmp_path / "build.ninja"
    with build_file_path.open("wt") as build_file:
        writer = ninja_syntax.Writer(output=build_file)
        writer.rule("capitalize", "dd if=$in of=$out conv=ucase")
        writer.build("loremipsum.txt.u", "capitalize", "loremipsum.txt")

    with raises(CalledProcessError) as exc_info:
        gbcli(cwd=tmp_path)

    error_message_expectation = (
        "error: 'loremipsum.txt', needed by 'loremipsum.txt.u', missing and no known rule to make it"
    )
    assert error_message_expectation in exc_info.value.output
