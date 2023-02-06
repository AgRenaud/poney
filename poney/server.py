
import socket
import multiprocessing


class Server:

    def __init__(self,
                application: str,
                *,
                address: str = "127.0.0.1",
                port: int = 3055,
                workers: int = 1,
                threads: int = 1,
                pthreads: int = 1,
                https: bool  = False
            ):

            self.application = application
            self.address = address
            self.port = port
            self.workers = max(1, workers)
            self.threads = max(1, threads)
            self.pthreads = max(1, pthreads)

    def up(self):
        ...

    def down(self):
        ...

    def _get_socket(self):
        sfd = socket.socket()
        sfd.bind((self.address, self.port))
        return sfd


if __name__ == "__main__":
    server = Server("test.py")
    sfd = server._get_socket()
    
    print("sfd:", sfd)