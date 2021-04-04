# About

Simple CLI for Slack.

# Getting Started

### Install command
```bash
cargo install --git https://github.com/mass10/rslack-command --branch main
```

### Create settings.toml like below
```TOML
[hello]
access_token = "xoxb-xxxxxxxxxxxx-xxxxxxxxxxxxx-xxxxxxxxxxxxxxxxxxxxxxxx"
channel = "notifications"
text = "text message here"
```

### Run
```bash
rslack-command hello
```
