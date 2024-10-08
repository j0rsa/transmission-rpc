[![Crates badge](https://badge-cache.kominick.com/crates/v/transmission-rpc.svg?label=transmission-rpc)](https://crates.io/crates/transmission-rpc)

Library to communicate with [transmission
rpc](https://github.com/transmission/transmission/blob/main/docs/rpc-spec.md)

**WARNING:**

It is highly encouraged to use HTTPS since the Transmission authentication is
using [BasicAuth](https://wikipedia.org/wiki/Basic_access_authentication) which
could be easily intercepted.

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
- [X] torrent-get
- [X] torrent-add
- [X] torrent-remove
- [X] torrent-set-location
- [X] torrent-rename-path
- [X] session-set
- [X] session-get
- [X] session-stats
- [X] blocklist-update
- [X] port-test
- [X] session-close
- [X] free-space

##### Feature Flags

- `sync`: Enables a thread-safe version of `TransClient`.
- `tor-get-serde`: Enables serde of `TorrentGetField`s.

-----

Support the project: [![Donate button](https://www.paypalobjects.com/en_US/DK/i/btn/btn_donateCC_LG.gif)](https://www.paypal.com/cgi-bin/webscr?cmd=_s-xclick&hosted_button_id=H337RKJSC4YG4&source=url)

<a href="https://www.buymeacoffee.com/j0rsa" target="_blank"><img src="https://www.buymeacoffee.com/assets/img/custom_images/orange_img.png" alt="Buy Me A Coffee" style="height: 41px !important;width: 174px !important;box-shadow: 0px 3px 2px 0px rgba(190, 190, 190, 0.5) !important;-webkit-box-shadow: 0px 3px 2px 0px rgba(190, 190, 190, 0.5) !important;" ></a>
