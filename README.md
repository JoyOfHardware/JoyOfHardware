# About

The beginnings of the Joy of Hardware Platform.

# Using

The following assumes NixOS. Release builds take much longer to build
but can result in substantial page loadtime savings.

```bash
nix-shell
mzoon start --release
```

# TODO

## Frontend Tasks

1. Responsive layout that tells you to use desktop version on improperly sized mobile

## Saturday August 17th

### By Tasks

- [x] Finish landing page
- [x] Pin `Cargo.toml` to a specific commit!
- [x] Implement routing

## Later
- [x] basic log in UI(no logic)
- [ ] message Martin about cloning difficulties on is_mobile
- update Favicon to smiley face emoji!
- [ ] re-factor out theme bits of code colors and assets
- [ ] Create `globals.rs`
- [ ] Create `utilities.rs`
- [ ] Mock out login page
- [ ] Blocking login attempts to mongoDB
  - [ ] Add time out if necessary

### Goals

- [ ] Responsive design
  - [ ] If on mobile, after landing tutorial, we re-direct user to Desktop for the best experience.
  - [ ] Maybe later support basic browsing of projects on mobile
  - [ ] Light/dark themes...
    - [ ] Dark mode needs orange glow on dark grey background
- [ ] read/store to/from MongoDB database
   - [ ] create accounts in MongoDB database
   - [ ] Stay signed in with auth token
- [ ] Call into git-tea API
  - [ ] Create account
    - [ ] Send e-mail upon creating account
  - [ ] Register JOH app as auth provider in git-tea
  - [ ] Go to URL with auth token
- [ ] Dev ops injection


### IDE

- [ ] Stateful RPC style executor(JOH exec)
  - [ ] Should have some sort of way to gather metrics about load and condition of executor
  - [ ] Clone git repo into folder on compute node backend
    - [ ] Shuttle files to frontend for display if under 500KB
- [ ] fix warngings(each release should compile warning free)
- [ ] File tree pane
- [ ] Code editor
  - [ ] Syntax highlighting enable - likely using sitting-tree parser
- [ ] Integrated terminal
  - [ ] Under the hood, JOH exec fires up docker container against the nix-shell file in the repo
- [ ] Integrate web programmer
