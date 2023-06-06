libftd3xx-py
====

Python bindings to libftd3xx


# Development Environment Setup

## Windows

1. Install [pyenv-win](https://github.com/pyenv-win/pyenv-win)
    - ```Invoke-WebRequest -UseBasicParsing -Uri "https://raw.githubusercontent.com/pyenv-win/pyenv-win/master/pyenv-win/install-pyenv-win.ps1" -OutFile "./install-pyenv-win.ps1"; &"./install-pyenv-win.ps1"```
2. Install python modules
    - `python -m pip install -r .\crates\libftd3xx-py\requirements.txt`
    - This guide is using `Python 3.11.3 (tags/v3.11.3:f3909b8, Apr  4 2023, 23:49:59) [MSC v.1934 64 bit (AMD64)] on win32`

3. Setup a Python virtual environment (This should be in the same directory as `libftd3xx/pyproject.toml`):
    - Create the virtual environment:
        - `cd C:\path\to\libftd3xx\crates\libftd3xx-py\`
        - `python -m venv .venv`
    - Enter the virutal Environment:
        - `.\.venv\Scripts\Activate.ps1`
    - Build and test the module: 
        - `maturin develop`
        - `python -c "import libftd3xx; print(libftd3xx.library_version())"`
            - Example in terminal:
                ```
                > python -c "import libftd3xx; print(libftd3xx.library_version())"
                1.3.4
                ```
    - See: [venv](https://docs.python.org/3/library/venv.html) for more documentation