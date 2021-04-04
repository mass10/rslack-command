# About

Simple CLI for Slack.

# Getting Started

```bash
cargo install --git https://github.com/mass10/rslack-command --branch main
```

# To run

```bash
rslack-command "NAME_OF_TASK_IN_TOML"
```

# settings.toml

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
