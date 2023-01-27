import io
import os
import sys

from typing import Callable, List


enc, esc = sys.getfilesystemencoding(), "surrogateescape"


def unicode_to_wsgi(u):
    return u.encode(enc, esc).decode("iso-8859-1")


def wsgi_to_bytes(s):
    return s.encode("iso-8859-1")


Writer = Callable[[bytes], None]


class WSGIHandler:
    def __init__(self, socket: io.IOBase) -> None:
        self.socket = socket
        self.headers_sent = []
        self.headers_set = []


    def write(self, data: bytes) -> None:
        out = self.socket

        if not self.headers_set:
            raise AssertionError("write() before start_response()")

        elif not self.headers_sent:
            # Before the first output, send the stored headers
            status, response_headers = self.headers_sent[:] = self.headers_set
            out.write(wsgi_to_bytes("Status: %s\r\n" % status))
            for header in response_headers:
                out.write(wsgi_to_bytes("%s: %s\r\n" % header))
            out.write(wsgi_to_bytes("\r\n"))

        out.write(data)
        out.flush()

    def start_response(self, status, response_headers, exc_info=None) -> Writer:
        if exc_info:
            try:
                if self.headers_sent:
                    raise exc_info[1].with_traceback(exc_info[2])
            finally:
                exc_info = None
        elif self.headers_set:
            raise AssertionError("Headers already set!")

        self.headers_set[:] = [status, response_headers]

        return self.write

    def run(self, application):
        # environ must be provided by Rust
        environ = {k: unicode_to_wsgi(v) for k, v in os.environ.items()}
        environ["wsgi.input"] = sys.stdin.buffer
        environ["wsgi.errors"] = sys.stderr
        environ["wsgi.version"] = (1, 0)
        environ["wsgi.multithread"] = False
        environ["wsgi.multiprocess"] = True
        environ["wsgi.run_once"] = True
        environ["REQUEST_METHOD"] = "GET"

        if environ.get("HTTPS", "off") in ("on", "1"):
            environ["wsgi.url_scheme"] = "https"
        else:
            environ["wsgi.url_scheme"] = "http"

        
        result = application(environ, self.start_response)

        return environ, result

