from langchain_contrib.utils.tests import current_directory

from zamm.actions.use_terminal import ZTerminal


def get_terminal() -> ZTerminal:
    return ZTerminal(safe_mode=False)


def test_terminal_simple_bash():
    t = get_terminal()
    assert t.run_bash_command("ls Makefile") == "Makefile\n"


def test_directory_change():
    with current_directory():  # reset to present cwd after test
        t = get_terminal()
        assert t.run_bash_command("pwd").strip().endswith("zamm")
        t.run_bash_command("cd tests")
        assert t.run_bash_command("pwd").strip().endswith("tests")


def test_tabbed_script():
    """Check whether we can capture tabbed output from the terminal."""
    t = get_terminal()
    assert t.run_bash_command("tests/resources/tabbed.sh") == "\\ta\n"


def test_tabbed_file():
    """Check whether we can capture tabbed output from files."""
    t = get_terminal()
    assert t.run_bash_command("cat tests/resources/tabbed.txt") == "\ta\n"


def test_no_ansi_color():
    t = get_terminal()
    assert t.run_bash_command("tests/resources/colored.sh") == "Hello\nWorld\n"


def test_no_ansi_color_py():
    t = get_terminal()
    assert t.run_bash_command("tests/resources/colored.py") == "bye world...\n"


def test_no_ansi_erase():
    t = get_terminal()
    assert (
        t.run_bash_command("tests/resources/erase_line.sh")
        == "Resolving dependencies...\n"
    )


def test_no_r():
    t = get_terminal()
    assert (
        t.run_bash_command("tests/resources/erase_line_r.sh")
        == "Resolving dependencies...\n"
    )


def test_no_previous_line():
    t = get_terminal()
    assert (
        t.run_bash_command("tests/resources/erase_previous_line.py")
        == "All that\nRemains\n"
    )
