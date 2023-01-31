import typer

from poney import serve


app = typer.Typer(name="server")


@app.command("up")
def start_server(application: str = typer.Option(...), bind: str = typer.Option(default="http://127.0.0.1:5055")):
    print("Hello From Python")
    serve()


@app.command("down")
def shutdown():
    ...


if __name__ == "__main__":
    app()
