import sys
from rich.console import (
    Console,
)
import time
import typer
from typycheck import (
    rust,
)

app = typer.Typer(name="typycheck")
console = Console()
version = "0.0.1"


@app.command()
def main(
    path: str = typer.Argument(help="Path to the directory or file to analyze"),
) -> None:
    console.rule(f"🦦 typycheck {version}")

    start_time = time.time()
    output = rust.type_check_file(path)
    execution_time = time.time() - start_time

    console.print(f"Execution time: {execution_time:.4f} seconds")

    if not output:
        console.rule("🫠  [bold red]Error![/bold red]")
        sys.exit(1)
    else:
        console.rule(":tada: [bold green]Success![/bold green] :tada:")


if __name__ == "__main__":
    app()
