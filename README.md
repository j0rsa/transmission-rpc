[![Crates badge](https://badge-cache.kominick.com/crates/v/transmission-rpc.svg?label=transmission-rpc)](https://crates.io/crates/transmission-rpc)

Library to communicate with transmission rpc

spec: https://github.com/transmission/transmission/blob/master/extras/rpc-spec.txt

#### Supported Methods

##### Torrent Actions

- [X] torrent-start
- [X] torrent-stop
- [X] torrent-start-now
- [X] torrent-verify
- [X] torrent-reannounce

##### Torrent Mutators

- [ ] torrent-set
- [X] torrent-get
- [X] torrent-add
- [X] torrent-remove
- [ ] torrent-set-location
- [ ] torrent-rename-path
- [ ] session-set
- [X] session-get
- [ ] session-stats
- [ ] blocklist-update
- [ ] port-test
- [ ] session-close
- [ ] free-space 

[https://crates.io/crates/transmission-rpc]: https://img.shields.io/badge/crates.io-transmission_rpc%20=%20%220.2.0%22-brightgreen.svg
