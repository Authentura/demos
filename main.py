from flask import Flask
from flask import request
from flask import redirect
from flask import send_file
from flask import make_response
from flask import render_template


from flask_restful import Api
from flask_restful import Resource

import sqlite3
from pprint import pprint

app = Flask(__name__)
restful = Api(app)

class StaticFile(Resource):
    def get(self, path):
        """ Return login page """
        # return any static file
        # NOTE: could be lfi
        return send_file(f"templates/{path}")


class Login(Resource):
    def get(self):
        """ Send the login page """
        return send_file('templates/login.html')

    def post(self):
        """ Get login credentials sent to user """

        # Get all users where the username and the password match
        conn = sqlite3.connect("./db.sqlite")
        cur = conn.cursor()
        cur.execute(f"SELECT * FROM users WHERE username='{request.form.get('username')}' AND password='{request.form.get('password')}'")
        rows = cur.fetchall()

        # If a user exists, set their cookie and return
        # NOTE: this is not the way you should handle cookies
        for a in rows:
            res = make_response(redirect('/'))
            print("cookie: ", a[2])
            res.set_cookie("token", a[2])
            return res

        # If no user was found then redirect back to /login
        return redirect('/login')




class Home(Resource):
    def get(self):
        """ 
        Check cookie, if valid then show some
        basic page. If invalid then move to login
        """

        # Check if the user has a cookie
        # If they don't then redirect them to login
        if not request.cookies.get('token'):
            return redirect('/login')

        # Get all users whose cookie matches
        # the cookie that the user has.
        conn = sqlite3.connect("./db.sqlite")
        cur = conn.cursor()
        cur.execute("SELECT username FROM users WHERE cookie=?", (request.cookies.get('token'),))
        rows = cur.fetchall()

        # If there are any users returned then
        # log in with the first one.
        for a in rows:
            res = make_response(render_template('home.html', name=a[0]))
            return res

        # If no users are returned then go back
        # to /login
        return redirect('/login')


restful.add_resource(Home, "/")
restful.add_resource(Login, "/login")
restful.add_resource(StaticFile, "/static/<path>")


if __name__ == "__main__":
    app.run(host="0.0.0.0", port=3000, debug=True)
