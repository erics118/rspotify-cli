# rspotify-cli

```
> rspotify-cli --help

A cli to get information and control Spotify.

Usage: rspotify-cli <COMMAND>

Commands:
  status     Print the current status
  control    Control the current playback
  play-from  Search and play songs
  help       Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

```
> cargo q status --help

Print the current status

Usage: rspotify-cli status [OPTIONS]

Options:
      --json   Print the full status in json to be used for external parsing
      --debug  Print the full status in the Rust debug format
  -h, --help   Print help

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
      --shuffle <STATE>  Set the shuffle state [possible values: enabled, disabled]
      --toggle-shuffle   Toggle the shuffle state
      --replay           Replay the current song
      --seek <POSITION>  Seek to a location in the current song in milliseconds
      --volume-up        Increase volume by a set amount
      --volume-down      Decrease volume by a set amount
  -h, --help             Print help

```

```
> rspotify-cli play-from --help

Search and play songs

Usage: rspotify-cli play-from [OPTIONS]

Options:
      --playlist <PLAYLIST>  Search for an playlist and play it
      --album <ALBUM>        Search for an album and play it
      --artist <ARTIST>      Search for an artist and play their top tracks
      --url <URL>            Search for a song's URL and play it
      --uri <URI>            Search for a song's URI and play it
  -h, --help                 Print help
```