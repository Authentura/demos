from flask import Flask, render_template, render_template_string, url_for
import os
import requests

app = Flask(__name__, static_folder="static")

@app.route('/') 
def index():
    return render_template('index.html', BASE_PATH=os.environ["BASE_PATH"])

@app.route('/<er>')
def errorHandlerfunc(er):
    template = '''{%% block body %%}
    <div class="center-content error">
        <center>
            <h1 style="color:red;">ERROR!</h1>
            <h1>Page not Found: %s</h1>
        </center>
    </div>
    {%% endblock %%}
    ''' % (er)

    return render_template_string(template), 404 

if __name__ == "__main__":
        app.run(host="127.0.0.1")
