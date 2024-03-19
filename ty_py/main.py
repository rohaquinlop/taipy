from rich.console import (
    Console,
)
import typer
from ty_py import (
    rust,
)

app = typer.Typer(name="ty-py")
console = Console()
version = "0.0.1"


@app.command()
def main() -> None:
    _a = rust.sum_as_string(1, 2)
    console.rule(f"🦦 ty-py {version}")


if __name__ == "__main__":
    app()
