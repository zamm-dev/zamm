"""Configurations for variables that we want to get LLM outputs for."""

from zamm.chains.general.get_dict import VariableConfig


def test_default_prompt() -> None:
    """Test that the simplest default options for variables work as expected."""
    config = VariableConfig(display="Action Input: ", output_key="input")
    assert config.prompt == "Action Input: "
    assert config.output_key == "input"
    assert config.stop == "\n"


def test_prompt_with_display_suffix() -> None:
    """Test that we can attach a suffix to the variable display."""
    config = VariableConfig(
        display="Action Input",
        display_suffix=": ",
        output_key="input",
    )
    assert config.prompt == "Action Input: "
    assert config.output_key == "input"
    assert config.stop == "\n"


def test_prompt_with_value() -> None:
    """Test constructing of templating string after value filled in."""
    config = VariableConfig(
        display="Action Input",
        display_suffix=": ",
        output_key="input",
    )
    assert config.prompt_with_value == "Action Input: {input}\n"


def test_prompt_with_custom_stop() -> None:
    """Test that templating string includes custom final stop."""
    config = VariableConfig(
        display="Input Code",
        display_suffix=": ```",
        output_key="input",
        stop="```",
    )
    assert config.prompt_with_value == "Input Code: ```{input}```"
