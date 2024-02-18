
** UNRELEASED

- auto-completion for event title
- customization day column width

** 0.1.9

- Fix the unreasonably long notification timeout (33 min -> 5 sec)
- Fix crash on cross day event
- Create `malakal` folder for database file if missing (#1, thanks @jakeisnt)
- Keyboard navigation and event manipulation

** 0.1.8

- Update calendar directory mtime on modification
  + This allows external tools (e.g. khal) to know it's time to refresh their cache
- Fix resizer rect which may not be clickable/draggable when events are too narrow
- Add config for =post_update_hook= and =post_update_hook_delay= for running =vdirsync= automatically
