[![Crates badge](https://badge-cache.kominick.com/crates/v/transmission-rpc.svg?label=transmission-rpc)](https://crates.io/crates/transmission-rpc)

Library to communicate with transmission rpc

**WARNING:**

It is highly encouraged to use HTTPS since the Transmission authentication is
using BasicAuth which could be easily intercepted.

#### Transmission RPC Spec

https://github.com/transmission/transmission/blob/main/docs/rpc-spec.md

#### Supported Methods

##### Torrent Actions

- [X] torrent-start
- [X] torrent-stop
- [X] torrent-start-now
- [X] torrent-verify
- [X] torrent-reannounce

##### Torrent Mutators

- [X] torrent-set (some)
- [X] torrent-get (most fields)
- [X] torrent-add
- [X] torrent-remove
- [X] torrent-set-location
- [X] torrent-rename-path
- [ ] session-set
- [X] session-get
- [X] session-stats
- [X] blocklist-update
- [X] port-test
- [X] session-close
- [X] free-space

Support the project: [![Donate button](https://www.paypalobjects.com/en_US/DK/i/btn/btn_donateCC_LG.gif)](https://www.paypal.com/cgi-bin/webscr?cmd=_s-xclick&hosted_button_id=H337RKJSC4YG4&source=url)
