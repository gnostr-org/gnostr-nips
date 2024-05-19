#!/usr/bin/env bash
for doc in $(ls *.md);do echo $doc;pandoc $doc | sed 's/.md/.md.html/g' > $doc.html;done;exit
