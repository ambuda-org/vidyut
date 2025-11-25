#!/usr/bin/env python3
import http.server
import socketserver
import mimetypes
import os
import argparse
import sys

# Make .rs files plain text
mimetypes.add_type("text/plain", ".rs")

# --- Parse command-line args ---
parser = argparse.ArgumentParser(description="Simple Python webserver with path mapping.")
parser.add_argument(
    "--map",
    action="append",
    help="Map a URL prefix to a local directory, e.g. --map /rust=/home/me/rust/src",
)
parser.add_argument("--port", type=int, default=8000, help="Port to listen on")
args = parser.parse_args()

# Convert --map arguments into a dict
ROUTES = {}
if args.map:
    for m in args.map:
        try:
            prefix, directory = m.split("=", 1)
        except ValueError:
            print(f"Invalid --map argument: {m}", file=sys.stderr)
            sys.exit(1)
        ROUTES[prefix] = os.path.abspath(directory)

class MyHandler(http.server.SimpleHTTPRequestHandler):
    def translate_path(self, path):
        # Check if path matches a custom route
        for prefix, directory in ROUTES.items():
            if path.startswith(prefix):
                rel_path = path[len(prefix):].lstrip("/")
                return os.path.join(directory, rel_path)
        # Fallback to normal behavior
        return super().translate_path(path)

Handler = MyHandler

with socketserver.TCPServer(("", args.port), Handler, bind_and_activate=False) as httpd:
    httpd.allow_reuse_address = True
    httpd.server_bind()
    httpd.server_activate()
    print(f"Serving on http://localhost:{args.port}")
    if ROUTES:
        print("Mappings:")
        for k, v in ROUTES.items():
            print(f"  {k} → {v}")
    httpd.serve_forever()
