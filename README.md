[![Crates badge](https://badge-cache.kominick.com/crates/v/transmission-rpc.svg?label=transmission-rpc)](https://crates.io/crates/transmission-rpc)

Library to communicate with transmission rpc

**WARNING:**

It is highly encouraged to use HTTPS since the Transmission authentication is using BasicAuth which could be easily intercepted

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
- [X] torrent-set-location
- [ ] torrent-rename-path
- [ ] session-set
- [X] session-get
- [X] session-stats
- [ ] blocklist-update
- [ ] port-test
- [ ] session-close
- [ ] free-space

Support the project: [![Donate button](https://www.paypalobjects.com/en_US/DK/i/btn/btn_donateCC_LG.gif)](https://www.paypal.com/cgi-bin/webscr?cmd=_s-xclick&hosted_button_id=H337RKJSC4YG4&source=url)
