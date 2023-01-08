from flask import Flask, render_template
from indic_transliteration import sanscript


app = Flask(__name__)


@app.template_filter()
def d(s) -> str:
    return sanscript.transliterate(s, 'hk', 'devanagari')


@app.route('/')
def index():
    return render_template("index.html")


app.run(debug=True)
