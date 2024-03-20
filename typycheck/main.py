from rich.console import (
    Console,
)
import typer
from typycheck import (
    rust,
)

app = typer.Typer(name="typycheck")
console = Console()
version = "0.0.1"


@app.command()
def main() -> None:
    _a = rust.sum_as_string(1, 2)
    console.rule(f"🦦 typycheck {version}")


if __name__ == "__main__":
    app()
