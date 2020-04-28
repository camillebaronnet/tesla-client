# Tesla Client

Tesla Client is an unofficial CLI written in Rust to provide Tesla API.

## 🔧 Development

```
git clone https://github.com/camillebaronnet/tesla-client

cd tesla-client
cargo run -- --help
```

## 🕮 Commands

See flags on commands by using the `--help` flag. (ex: `tesla-client login --help`)

### Login



```bash
$ tesla-client login
```

```
Username : YOURLOGIN
Password: *******

Login succeeded!
Token : xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx
```

Token will be stored in these folders:

- **Linux**:  /home/YOU/.config/tesla-client
- **macOS**: /Users/YOU/Library/Preferences/tesla-client
- **Windows**: C:\Users\YOU\AppData\Roaming/tesla-client

### List products

```bash
$ tesla-client products
```

```
ID                      VIN                     State           Name
xxxxxxxxxxxxxxxxx       xxxxxxxxxxxxxxxxx       offline         Blackmobile
```

## 📋 Todo

This program is currently WIP, help me finish it.

Here is the main roadmap :

### Features

- [X] Login support
- [X] List  of products
- [ ] Detail each product (vehicles in first, then energy sites and Powerwalls)
- [ ] Support car settings
- [ ] Support vehicles commands like car unlocking...

### Others

- [ ] Catch API errors (like unauthorized)
- [ ] Code reorganization (it's a noob code that discovers Rust)
- [ ] Test the program (#shame)
- [ ] Add a CI/CD (Tavis ?)
- [ ] Add the right license (Free and prohibit any commercial use)
- [ ] And, create issues for all these points
