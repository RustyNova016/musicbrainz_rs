curl -s https://musicbrainz.org/statistics/languages-scripts | \
  grep -Eo '<td>[^<]*</td><td class="t"><a href="https://musicbrainz.org/search\?query=script%3A%22[^"]*%22' | \
  sort | \
  sed 's,<td>\([^<]*\)</td><td class="t"><a href="https://musicbrainz.org/search?query=script%3A%22\([^"]*\)%22,\/\/\/ \1\n\2\,,' > ./templates/release_scripts/enum.txt

curl -s https://musicbrainz.org/statistics/languages-scripts | \
  grep -Eo '<td>[^<]*</td><td class="t"><a href="https://musicbrainz.org/search\?query=script%3A%22[^"]*%22' | \
  sort | \
  sed 's,<td>\([^<]*\)</td><td class="t"><a href="https://musicbrainz.org/search?query=script%3A%22\([^"]*\)%22,            Self::\2 => "\1"\,,' > ./templates/release_scripts/name.txt

curl -s https://musicbrainz.org/statistics/languages-scripts | \
  grep -Eo '<td>[^<]*</td><td class="t"><a href="https://musicbrainz.org/search\?query=script%3A%22[^"]*%22' | \
  sort | \
  sed 's,<td>\([^<]*\)</td><td class="t"><a href="https://musicbrainz.org/search?query=script%3A%22\([^"]*\)%22,            Self::\2 => "\2"\,,' > ./templates/release_scripts/code.txt