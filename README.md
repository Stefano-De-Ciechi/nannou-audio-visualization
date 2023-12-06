# Description

a simple rust application made with the [nannou framework](https://nannou.cc/) that visualizes audio into two separate channels, Left and Right

the audio is captured from the microphone (I still don't know how to change input device from code), but by using programs like qjackctl and editing the audio graph you can redirect inputs from other sources (and visualize them)

for example from youtube running in a browser (it has to be running to be visible), from spotify or from an analog instrument connected to a sound card

if you connected the L and R channels correctly you should be able to see the differences (but you actually have to reproduce stereo audio, try for example with the song California Dreamin' from The Mamas & The Papas, in the beginning you can clearly see differences in both channels) */

# Run the program

be sure to follow nannou's [platform specific setup](https://guide.nannou.cc/getting_started/platform-specific_setup) guide to install all the necessary components for the framework

clone this repository, cd into it from a terminal, and run:


```shell
cargo run --release
```

the initial time it will download and compile all the libraries (nannou is a fairly large crate, it may take some time)

once compiled, you can tecnically just take the executable (in folder target/release/ ) and place anywhere in your pc, and then delete the directory (to save space)