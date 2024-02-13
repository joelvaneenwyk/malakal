# Malakal

Malakal is a day planner application. I crafted it because I was not able to find a comfortable calendar application for Linux.

I consider it in a mostly usable state - I myself have been using it on a daily basis for around a year now. Bug reports, feature requests, and contributions are warmly welcomed.

<!-- markdownlint-disable MD033 -->
<img alt="Screenshot of day planner application" src="asset/screenshot.png" width="600"/>

## Features

### Core features

- Quickly add/modify/relocate/clone/delete events
- Notify on event starts
- Stored as standard ical files
- Post-update command (for running e.g. vdirsyncher)

### UI/UX features

- Click to edit event title
- Snapping mode (hold down shift to precision mode)
- Ctrl-z to undo modifications
- Drag on blank to create events
- Drag on an event to change its begin/end time or move the event
- Ctrl-drag on an event to clone it
- Right-click on an event to open menu (for event detail & deletion)
- Right-click on blank area to open calendar view
- Quickly jumping to dates in calendar view
- Current date/time indicator
- Full keyboard support for navigation/event manipulation

Typical calendar features that are **not** supported by Malakal:

- Synchronization/webdav: for now, you can use vdirsync for synchronization
- Command line query: malakal happily shares local ical files with [khal](https://github.com/pimutils/khal)
- Recurrent events: it may add a lot of complexity so it's not currently planned

## Keyboard shortcuts

| Keys            | Actions                            |
|-----------------+------------------------------------|
| Tab/Shift-Tab   | Focus previous/next event     |
| Arrow keys      | Focus event on the given direction |
| Ctrl+Arrow keys | Move focused event                 |
| Shift+Up/Down   | Resize focused event               |
| n               | Create new event                   |
| x/Del           | Delete focused event               |

Arrow keys can be substituted for vim-style navigation keys (hjkl) in all above cases.

## Installation

If you have rust on your computer, you can type:

```bash
cargo install malakal
```

Alternatively, you can download the binary from release page.
