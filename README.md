# aup3conv

Scrape Audacity project files ... without actually running Audacity.

# Notes

aup3conv builds upon [rusqlite](https://github.com/rusqlite/rusqlite). In order
to support Windows, rusqlite builds its own sqlite library and links against it
using the "bundled" feature. So, it does not use the same sqlite version that
you may have preinstalled on you device.
