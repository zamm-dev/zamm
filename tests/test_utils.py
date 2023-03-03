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


def test_remove_multiple_lines():
    input = b"  \x1b[34;1m\xe2\x80\xa2\x1b[39;22m \x1b[39mInstalling \x1b[39m\x1b[36mfaiss-cpu\x1b[39m\x1b[39m (\x1b[39m\x1b[39;1m1.7.3\x1b[39;22m\x1b[39m)\x1b[39m: \x1b[34mInstalling...\x1b[39m\r\n  \x1b[32;1m\xe2\x80\xa2\x1b[39;22m \x1b[39mInstalling \x1b[39m\x1b[36mflake8\x1b[39m\x1b[39m (\x1b[39m\x1b[32m6.0.0\x1b[39m\x1b[39m)\x1b[39m\r\n  \x1b[32;1m\xe2\x80\xa2\x1b[39;22m \x1b[39mInstalling \x1b[39m\x1b[36mgoogle-search-results\x1b[39m\x1b[39m (\x1b[39m\x1b[32m2.4.1\x1b[39m\x1b[39m)\x1b[39m\r\n  \x1b[32;1m\xe2\x80\xa2\x1b[39;22m \x1b[39mInstalling \x1b[39m\x1b[36mgorilla\x1b[39m\x1b[39m (\x1b[39m\x1b[32m0.4.0\x1b[39m\x1b[39m)\x1b[39m\r\n  \x1b[32;1m\xe2\x80\xa2\x1b[39;22m \x1b[39mInstalling \x1b[39m\x1b[36misort\x1b[39m\x1b[39m (\x1b[39m\x1b[32m5.11.4\x1b[39m\x1b[39m)\x1b[39m\r\n  \x1b[32;1m\xe2\x80\xa2\x1b[39;22m \x1b[39mInstalling \x1b[39m\x1b[36mlangchain\x1b[39m\x1b[39m (\x1b[39m\x1b[32m0.0.100\x1b[39m\x1b[39m)\x1b[39m\r\n  \x1b[32;1m\xe2\x80\xa2\x1b[39;22m \x1b[39mInstalling \x1b[39m\x1b[36mmypy\x1b[39m\x1b[39m (\x1b[39m\x1b[32m0.991\x1b[39m\x1b[39m)\x1b[39m\r\n  \x1b[32;1m\xe2\x80\xa2\x1b[39;22m \x1b[39mInstalling \x1b[39m\x1b[36mopenai\x1b[39m\x1b[39m (\x1b[39m\x1b[32m0.26.4\x1b[39m\x1b[39m)\x1b[39m\r\n  \x1b[32;1m\xe2\x80\xa2\x1b[39;22m \x1b[39mInstalling \x1b[39m\x1b[36mpytest\x1b[39m\x1b[39m (\x1b[39m\x1b[32m7.2.1\x1b[39m\x1b[39m)\x1b[39m\r\n  \x1b[32;1m\xe2\x80\xa2\x1b[39;22m \x1b[39mInstalling \x1b[39m\x1b[36mvcrpy\x1b[39m\x1b[39m (\x1b[39m\x1b[32m4.2.1\x1b[39m\x1b[39m)\x1b[39m\r\n\x1b[10A\x1b[0J  \x1b[32;1m\xe2\x80\xa2\x1b[39;22m \x1b[39mInstalling \x1b[39m\x1b[36mflake8\x1b[39m\x1b[39m (\x1b[39m\x1b[32m6.0.0\x1b[39m\x1b[39m)\x1b[39m\r\n".decode(  # noqa
        "utf-8"
    )
    assert (
        remove_ansi_escapes(input.replace("\r\n", "\n"))
        == "  • Installing flake8 (6.0.0)\n"
    )
