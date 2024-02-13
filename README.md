* Malakal

Malakal is a day planner application. I crafted it because I was not able to find a comfortable calendar application for Linux.

I consider it in a mostly usable state - I myself have been using it on a daily basis for around a year now. Bug reports, feature requests, and contributions are warmly welcomed.

#+html: <img src="asset/screenshot.png" width="600"/>

* Features

Core features:

- quickly add/modify/relocate/clone/delete events
- notify on event starts
- stored as standard ical files
- post-update command (for running e.g. vdirsyncher)

UI/UX features:

- click to edit event title
- snapping mode (hold down shift to precision mode)
- ctrl-z to undo modifications
- drag on blank to create events
- drag on an event to change its begin/end time or move the event
- ctrl-drag on an event to clone it
- right-click on an event to open menu (for event detail & deletion)
- right-click on blank area to open calendar view
- quickly jumping to dates in calendar view
- current date/time indicator
- full keyboard support for navigation/event manipulation

Typical calendar features that are not supported by malakal:

- synchronization/webdav: for now, you can use vdirsync for synchronization
- command line query: malakal happily shares local ical files with [[https://github.com/pimutils/khal][khal]]
- recurrent events: it may add a lot of complexity so it's not currently planned

* Keyboard shortcuts

| Keys            | Actions                            |
|-----------------+------------------------------------|
| Tab/Shift-Tab   | Focus previous/next event     |
| Arrow keys      | Focus event on the given direction |
| Ctrl+Arrow keys | Move focused event                 |
| Shift+Up/Down   | Resize focused event               |
| n               | Create new event                   |
| x/Del           | Delete focused event               |

Arrow keys can be substituted for vim-style navigation keys (hjkl) in all above cases.

* Installation

If you have rust on your computer, you can type:

#+begin_src
cargo install malakal
#+end_src

Alternatively, you can download the binary from release page.
