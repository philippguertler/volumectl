# volumectl
This program lets you set the volume of predefined applications. 

## Requirements
* libpulse

## Configuration
Create the file `~/.config/volumectl.conf` and write your matching rules in each line.

A matching rule follows this pattern:
`{{prop}}={{regex}}`

* `prop`: a property of a pulse audio input sink. Run `pactl list sink-inputs` to see the props of all currently running output streams.
* `regex` a regular expression matching the value of `prop`.

At least one of the rules have to match in order to set the volume of the stream.

## Usage
After setting up the configuration file run `volumectl set-volume <VOLUME>`, where volume
is a number between 0 and 1. 

## Help
```
volumectl 0.1.0

USAGE:
    volumectl <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    help          Prints this message or the help of the given subcommand(s)
    set-volume    Sets the volume of the configured sink inputs
```

## set-volume

```
volumectl-set-volume 
Sets the volume of the configured sink inputs

USAGE:
    volumectl set-volume <VOLUME>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <VOLUME>    The volume to set. e.g. 0.3
```
