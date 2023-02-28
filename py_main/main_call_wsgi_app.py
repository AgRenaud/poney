from time import sleep
from poney import load_application


app = load_application("wsgi_app:create_app()")
