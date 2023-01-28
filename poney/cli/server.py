import typer

from poney import serve


app = typer.Typer(name="poney")


@app.command("up")
def start_server():
    print("Hello From Python")
    serve()

@app.command("down")
def shutdown():
    ...

if __name__ == "__main__":
    app()
