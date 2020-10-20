import pytest
import subprocess

def pytest_addoption(parser):
    parser.addoption(
        "--cli-exec", action="store", default="gb", help="path to the graph_build command line executable"
    )


@pytest.fixture
def gbcli(request):
    cli = request.config.getoption("--cli-exec")
    def _run_gbcli(args=[], cwd=None):
        args.insert(0, cli)
        completion = subprocess.run(args, cwd=cwd)
        completion.check_returncode()

    return _run_gbcli