Library to communicate with transmission rpc

spec: https://github.com/transmission/transmission/blob/master/extras/rpc-spec.txt

Supported Methods:

Torrent Actions:

- [X] torrent-start
- [X] torrent-stop
- [X] torrent-start-now
- [X] torrent-verify
- [X] torrent-reannounce

Torrent modificators:

- [ ] torrent-set
- [X] torrent-get
- [ ] torrent-add
- [ ] torrent-remove
- [ ] torrent-set-location
- [ ] torrent-rename-path
- [ ] session-set
- [X] session-get
- [ ] session-stats
- [ ] blocklist-update
- [ ] port-test
- [ ] session-close
- [ ] free-space 