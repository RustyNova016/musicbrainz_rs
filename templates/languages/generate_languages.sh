curl -s https://musicbrainz.org/statistics/languages-scripts | \
  grep -Eo '<td>[^<]*</td><td class="t"><a href="https://musicbrainz.org/search\?query=lang%3A%22[^"]*%22' | \
  sort | \
  sed -e 's,<td>\([^<]*\)</td><td class="t"><a href="https://musicbrainz.org/search?query=lang%3A%22\([^"]*\)%22,\/\/\/ \1\n\u\2\,,' -e "s/&#x27;/'/" > ./templates/languages/enum.txt

curl -s https://musicbrainz.org/statistics/languages-scripts | \
  grep -Eo '<td>[^<]*</td><td class="t"><a href="https://musicbrainz.org/search\?query=lang%3A%22[^"]*%22' | \
  sort | \
  sed -e 's,<td>\([^<]*\)</td><td class="t"><a href="https://musicbrainz.org/search?query=lang%3A%22\([^"]*\)%22,Self::\u\2 => "\1"\,,' > ./templates/languages/name.txt

curl -s https://musicbrainz.org/statistics/languages-scripts | \
  grep -Eo '<td>[^<]*</td><td class="t"><a href="https://musicbrainz.org/search\?query=lang%3A%22[^"]*%22' | \
  sort | \
  sed -e 's,<td>\([^<]*\)</td><td class="t"><a href="https://musicbrainz.org/search?query=lang%3A%22\([^"]*\)%22,Self::\u\2 => "\2"\,,' > ./templates/languages/code.txt


