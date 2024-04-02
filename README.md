A custom helper for git credentials using the One Password CLI

# Installation
- Installation depends on rust. You can follow the steps to download rust here - https://rustup.rs/
- Use cargo install to build off of `main`
```
cargo install --git https://github.com/zschreur/op_git.git --locked
```

# Usage
- Ensure that you have the One Password CLI installed and configured
- Add a password to 1Password with your GitHub username and Personal Access Token
  - The item needs to have a password & username field
<img width="762" alt="image" src="https://github.com/zschreur/op_git/assets/51675520/59130105-6e7c-4e52-bd26-b15742c1cf55">

- Create a `.op_git.toml` file
  - Set the `vault` and `id` fields which correspond to the item you created in 1Password
  - op_git will search upwards from the current directory until it finds a `.op_git.toml` file

For example:
```toml
vault = "git" # this is the name of the vault I use to store my git Personal Access Tokens
id = "zschreur" # this is the name of the item in the vault
```

- Edit your git config and set the `credential.helper` to `op`
  - Note that if you have other credential helpers set you may need to remove them
```bash
git config --global credential.helper op
```

# Known Issues
- When signing in with 1Password the `op` cli can sometimes hang - this seems to happen intermittently
