# Notify-Sound

A tool to play a sound when a notification is received on environments that don't have that feature by default.

## Config
The program is configured using the config file in `~/.config/notify_sound.toml`. When you run the program for the first time, a default config will be generated there.

Here is an example configuration file:
```toml
[[sounds]]
app_name = "whatsapp|vesktop"
sound_path = "instant-message.wav"

[[sounds]]
app_name = ".*"
sound_path = "default.wav"

# 'notify_sound_urgent' is a special notification used to assign a special sound to
# notifications marked as urgent.
[[sounds]]
app_name = "notify_sound_urgent"
sound_path = "urgent.wav"
```
