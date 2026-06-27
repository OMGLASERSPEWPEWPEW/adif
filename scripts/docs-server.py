"""ADIF docs server — serves docs/ on port 5906 and logs navigation to docs/access.log."""

import http.server
import os
import datetime

PORT = 5906
DOCS_DIR = os.path.join(os.path.dirname(os.path.dirname(os.path.abspath(__file__))), "docs")
LOG_FILE = os.path.join(DOCS_DIR, "access.log")

class LoggingHandler(http.server.SimpleHTTPRequestHandler):
    def __init__(self, *args, **kwargs):
        super().__init__(*args, directory=DOCS_DIR, **kwargs)

    def do_GET(self):
        if self.path.endswith(('.html', '/')) and not self.path.endswith(('.js', '.css', '.ico', '.png', '.jpg')):
            ts = datetime.datetime.now().strftime("%Y-%m-%d %H:%M:%S")
            page = self.path if self.path != '/' else '/index.html'
            line = f"[{ts}] VISIT {page}\n"
            with open(LOG_FILE, "a", encoding="utf-8") as f:
                f.write(line)
            print(line.strip())
        return super().do_GET()

if __name__ == "__main__":
    with open(LOG_FILE, "a", encoding="utf-8") as f:
        f.write(f"[{datetime.datetime.now().strftime('%Y-%m-%d %H:%M:%S')}] SERVER START on port {PORT}\n")
    print(f"Serving docs at http://localhost:{PORT}")
    print(f"Logging navigation to {LOG_FILE}")
    server = http.server.HTTPServer(("", PORT), LoggingHandler)
    server.serve_forever()
