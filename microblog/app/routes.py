from flask import render_template
from app import app

@app.route('/')
@app.route('/index')
def index():
    user = {'username': 'Max'} #mock user
    posts = [                   # mock post
        {
            'author': {'username': 'John'},
            'body': 'Beautiful day today!'
        },
        {
            'author': {'username': 'Susan'},
            'body': 'The movie was so good!'
        }
    ]
    return render_template('index.html', title='Home', user=user, posts=posts)
