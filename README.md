rTrackr
===
*Personal productivity tracker*

Allows user to track their activity across open windows and browser tabs and classify the time spent as *Productive*, *Leisure* or *Neutral* time.

### Roadmap
---
**ATTENTION** - this project is still a work-in-progress

---

In order of priority from highest to lowest.

- [x] Support for Linux (xorg)
- [x] Persistent record store on file system
- [ ] TUI
- [ ] Express time classification via TOML conf (release)
- [ ] Projects
- [ ] Support for Windows
- [ ] Support for OSX
- [ ] Persistent record store in the cloud

### Development notes
Current unit tests mutate filesystem, which makes them dependent on the order of execution. Until this is fixed, it is preferable to run tests with
```bash
cargo test -- --test-threads 1
```