#!/usr/bin/env bash
case "$OSTYPE" in
  linux*)   echo "Linux / WSL" ;;
  darwin*)  echo "MacOS" ;;
  win*)     echo "Windows" ;;
  msys*)    echo "MSYS / MinGW / Git Bash" ;;
  cygwin*)  echo "Cygwin" ;;
  bsd*)     echo "BSD" ;;
  solaris*) echo "Solaris" ;;
  *)        echo "unknown: $OSTYPE" ;;
esac

docs() {
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
}

case "$OSTYPE" in
  linux*)   docs;;
  darwin*)  docs;;
  win*)     echo "Windows" ;;
  msys*)    echo "MSYS / MinGW / Git Bash" ;;
  cygwin*)  echo "Cygwin" ;;
  bsd*)     echo "BSD" ;;
  solaris*) echo "Solaris" ;;
  *)        echo "unknown: $OSTYPE" ;;
esac
exit
