# Setting up VS Code

## Installation

Follow the instructions at https://code.visualstudio.com/docs/setup/linux

## Customization

### Maximum line length

Just showing the ruler:

```json
    "editor.rulers": [100, 120, 140]
```

Actually getting text to soft-wrap:

```json
    "editor.wordWrap": "wordWrapColumn",
    "editor.wordWrapColumn": 160
```

Source: https://stackoverflow.com/questions/60060373/in-visual-studio-code-how-to-extend-the-maximum-line-width
