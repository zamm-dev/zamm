import os

# choose something that is unlikely to appear in code files
FILE_SEPARATOR = "~~~~~~~~~~"


class FilesTool:
    """Sets up file reading and writing"""

    def read(self, path: str) -> str:
        if not os.path.isfile(path):
            return f"Unfortunately, the file at `{path}` does not exist."

        with open(path) as f:
            contents = f.read()
            return f"""
The contents of the file at `{path}` are

{FILE_SEPARATOR}
{contents}
{FILE_SEPARATOR}
""".strip()

    def write(self, path: str, contents: str) -> str:
        try:
            with open(path, "w") as f:
                f.write(contents)
                return f"The file at `{path}` was successfully written to."
        except Exception as e:
            return f"The file at `{path}` could not be written to: {e}"
