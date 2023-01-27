HELLO_WORLD = b"Hello world!\n"


class AppClass:
    def __init__(self, environ, start_response):
        print("Create application")
        
        self.environ = environ
        self.start = start_response

    def __iter__(self):
        status = "200 OK"
        response_headers = [("Content-type", "text/plain")]
        self.start(status, response_headers)
        yield HELLO_WORLD
