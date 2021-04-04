# About

Simple CLI for Slack.

# Getting Started

### Install command
```bash
cargo install --git https://github.com/mass10/rslack-command --branch main
```

### Create settings.toml like below
```TOML
[task01]
access_token = "xoxb-xxxxxxxxxxxx-xxxxxxxxxxxxx-xxxxxxxxxxxxxxxxxxxxxxxx"
channel = "notifications"
text = "text message here"

[task02]
access_token = "xoxb-xxxxxxxxxxxx-xxxxxxxxxxxxx-xxxxxxxxxxxxxxxxxxxxxxxx"
channel = "notifications"
text = "text message here"

[task01]
access_token = "xoxb-xxxxxxxxxxxx-xxxxxxxxxxxxx-xxxxxxxxxxxxxxxxxxxxxxxx"
channel = "notifications"
text = "text message here"
```

### Run
```bash
rslack-command "NAME_OF_TASK_IN_TOML"
```
