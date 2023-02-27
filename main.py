import sys

from simple_app import AppClass, simple_app
from flask_example import create_app

from poney.specs.wsgi import WSGIHandler
from poney import Worker


class bcolors:
    HEADER = "\033[95m"
    OKBLUE = "\033[94m"
    OKCYAN = "\033[96m"
    OKGREEN = "\033[92m"
    WARNING = "\033[93m"
    FAIL = "\033[91m"
    ENDC = "\033[0m"
    BOLD = "\033[1m"
    UNDERLINE = "\033[4m"


if __name__ == "__main__":

    print("\n" + bcolors.OKCYAN + "# Run with Rust worker and " + bcolors.OKBLUE + "Python App function" + bcolors.ENDC)
    request_handler = WSGIHandler(sys.stdout.buffer)
    worker = Worker(simple_app, request_handler, "127.0.0.1", 8000)

    worker.run_with_cgi()

    print("\n" + bcolors.OKCYAN + "# Run with Rust worker and " + bcolors.OKGREEN + "Python App class" + bcolors.ENDC)
    request_handler = WSGIHandler(sys.stdout.buffer)
    worker = Worker(AppClass, request_handler, "127.0.0.1", 8000)

    worker.run_with_cgi()

    print("\n" + bcolors.OKCYAN + "# Run with Rust worker and " + bcolors.WARNING + "Flask" + bcolors.ENDC)
    request_handler = WSGIHandler(sys.stdout.buffer)
    application = create_app()
    worker = Worker(application, request_handler, "127.0.0.1", 8000)

    worker.run_with_cgi()
