import typer

from poney.cli import server


app = typer.Typer(name="poney")

app.add_typer(server.app, name="server")


if __name__ == "__main__":
    app()
