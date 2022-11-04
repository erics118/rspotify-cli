# rspotify-cli

```
A cli to get information and control Spotify.

Usage: rspotify-cli [OPTIONS] --client-id <CLIENT_ID> --client-secret <CLIENT_SECRET> [COMMAND]

Commands:
  debug               Print the entire status in a debug format
  title               Print the title of the song
  artist              Print the artist of the song
  progress            Print the current progress in the song
  duration            Print the length of the song
  status              Print the status of the song
  play                Play the song if it was previously paused
  pause               Pause the song if it was previously playing
  toggle-play-pause   Toggle the state of the song between playing and paused
  like                Like the current song
  unlike              Unlike the current song
  toggle-like-unlike  Toggle like/unlike for the current song

Options:
  -i, --client-id <CLIENT_ID>
          [env: SPOTIFY_CLIENT_ID=]
  -s, --client-secret <CLIENT_SECRET>
          [env: SPOTIFY_CLIENT_SECRET=]
  -r, --redirect-uri <REDIRECT_URL>
          [env: SPOTIFY_REDIRECT_URL=] [default: http://localhost:8000/callback]
  -h, --help
          Print help information
  -V, --version
          Print version information
```

