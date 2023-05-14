# rspotify-cli

```
> rspotify-cli --help

A cli to get information and control Spotify.

Usage: rspotify-cli <COMMAND>

Commands:
  status     Print the current status. The API quickly forgets the song if it hasn't been playing for a while
  control    Control the current playback
  play-from  Play songs
  search     Search anything
  help       Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

```
> cargo q status --help

Print the current status. The API quickly forgets the song if it hasn't been playing for a while

Usage: rspotify-cli status [OPTIONS]

Options:
      --json  Print the full status in json to be used for external parsing
  -h, --help  Print help

Display:
      --id            Print the id
      --url           Print the url
      --title         Print the title
      --artist        Print the artist name
      --progress      Print the progress
      --duration      Print the duration
      --is-playing    Print if the song is currently playing
      --repeat-state  Print the repeat_state
      --is-shuffled   Print if it is shuffled
      --device        Print the device name
      --playing-type  Print the playing type
      --is-liked      Print if the song is liked
```

```
> rspotify-cli control --help

Control the current playback

Usage: rspotify-cli control [OPTIONS]

Options:
      --play             Play the song if it was previously paused
      --pause            Pause the song if it was previously playing
      --toggle-play      Toggle the state of the song between playing and paused
      --like             Like the current song
      --unlike           Unlike the current song
      --toggle-like      Toggle like/unlike for the current song
      --previous         Go to the previous song
      --next             Go to the next song
      --repeat <STATE>   Set the repeat state [possible values: off, context, track]
      --cycle-repeat     Cycle between repeat states
      --volume <VOLUME>  Set the volume
      --volume-up        Increase volume by a set amount
      --volume-down      Decrease volume by a set amount
      --shuffle <STATE>  Set the shuffle state [possible values: true, false]
      --toggle-shuffle   Toggle the shuffle state
      --seek <POSITION>  Seek to a location in the current song in seconds
      --replay           Replay the current song
  -h, --help             Print help
```

```
Search anything

Usage: rspotify-cli search [OPTIONS]

Options:
      --artist <ARTIST>      Search for artists
      --album <ALBUM>        Search for albums
      --track <TRACK>        Search for tracks
      --playlist <PLAYLIST>  Search for playlists
      --show <SHOW>          Search for shows
      --episode <EPISODE>    Search for episodes
  -h, --help                 Print help
```
