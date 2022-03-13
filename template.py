#!/usr/bin/env python3

import chevron

__INDEX_HTML = 'index.html'
__INDEX_MUSTACHE = 'templates/index.html.mustache'
__DIST_FOLDER = 'dist'
__DOCS_FOLDER = 'docs'

data = {
    "title": "Rquote",
    "url": 'https://rquote.vercel.app',
    "description": 'Web client for animechan API',
    'logo': 'resources/logo.png',
    'author': 'Altair-Bueno',
    'theme': {
        'dark': '202528',
        'light': 'F8F9FA'
    }
}

template = open(__INDEX_MUSTACHE, 'r')
rendered = chevron.render(template, data)
out = open('index.html', 'w')
out.write(rendered)
out.close()
