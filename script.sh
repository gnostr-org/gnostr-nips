#!/usr/bin/env bash
for doc in $(ls *.md);do \
echo $doc; \
pandoc --ascii           $doc | \
sed 's/.md/.md.html/g'   > docs/$doc.html;done;

for doc in $(ls *.md);do \
echo $doc; \
pandoc --ascii -t plain  $doc | \
sed 's/.md/.txt/g'       > docs/$doc.txt;done;

for doc in $(ls *.md);do \
echo $doc; \
pandoc -s                $doc | \
sed 's/.md/.md.css.html/g' > docs/$doc.css.html;done;
exit
