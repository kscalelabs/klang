# This file is automatically generated by pyo3_stub_gen
# ruff: noqa: E501, F401


class PyKlangProgram:
    def save_binary(self, path:str) -> None:
        ...

    def save_text(self, path:str) -> None:
        ...

    def __repr__(self) -> str:
        ...

    @staticmethod
    def load_binary(path:str) -> PyKlangProgram:
        ...

    def to_list(self) -> list[list[str]]:
        ...


def get_version() -> str:
    ...

def parse_file(path:str) -> PyKlangProgram:
    ...

def parse_string(input:str) -> PyKlangProgram:
    ...

