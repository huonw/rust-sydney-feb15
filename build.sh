#!/bin/sh

rustdoc intro.md -o . --html-in-header=header.inc.html --markdown-no-toc

#mkdir -p haiku
#rustdoc haiku.md -o haiku --html-in-header=header.inc.html --markdown-no-toc
