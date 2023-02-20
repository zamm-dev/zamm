import re

from zamm.utils import ansi_escape_regex, remove_ansi_escapes


def test_remove_rs():
    line = (
        "\r.gitignore            0%[                   ]       0  --.-KB/s               "  # noqa
        "\r.gitignore          100%[===================>]  3.01K  --.-KB/s    in 0s      "  # noqa
    )
    assert (
        remove_ansi_escapes(line)
        == ".gitignore          100%[===================>]  3.01K  --.-KB/s    in 0s      "  # noqa
    )


def test_escape_extraction():
    """Detect different escape codes and their commands."""
    regex = ansi_escape_regex()
    assert re.search(regex, "Detect\rthis").group() == "\r"
    assert re.search(regex, "Detect\\rthis").group() == "\\r"
    assert re.search(regex, "Detect\\rthis").groups() == (None, None)

    assert re.search(regex, "Detect\033[31;1;4mthis").group() == "\033[31;1;4m"
    assert re.search(regex, "Detect\033[31;1;4mthis").groups() == ("\033", "31;1;4m")

    assert re.search(regex, "Detect\\e[31;1;4mthis").group() == "\\e[31;1;4m"
    assert re.search(regex, "Detect\\e[31;1;4mthis").groups() == ("\\e", "31;1;4m")

    assert re.search(regex, "Detect\x1b[31;1;4mthis").group() == "\x1b[31;1;4m"
    assert re.search(regex, "Detect\x1b[31;1;4mthis").groups() == ("\x1b", "31;1;4m")

    assert re.search(regex, "Detect\u001b[31;1;4mthis").group() == "\u001b[31;1;4m"
    assert re.search(regex, "Detect\u001b[31;1;4mthis").groups() == (
        "\u001b",
        "31;1;4m",
    )

    assert re.search(regex, "Detect\x1b[2Kthis").group() == "\x1b[2K"
    assert re.search(regex, "Detect\x1b[2Kthis").groups() == ("\x1b", "2K")
