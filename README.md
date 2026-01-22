# Intro
I use two main audio profiles at my home PC. One profile is desktop speakers with a desktop microphone. The other, a wireless headset with speakers and a microphone built in. My desktop microphone is only usable when the volume is at 40% whereas my headset microphone is great at 100%. The issue that I run into with KDE is that any time I switch audio devices is that KDE will set the audio device to 100% every time I switch. I decided that this would be a fun vibecoding experience and I was right. This codebase was almost entirely written via ChatGPT while I worked along side troubleshooting. I personally only changed the color theme so that it would match the user's preferred theme, I added a sleep between when the program sets the default audio device to when the program sets the volume level, and I changed the slider bars to take up the size of their parent. I'm quite excited by what I built and I fully plan to use it over the next couple weeks to see if it works. At some point I need to review the code because as AI slop goes there's probably some inefficiencies. The bottom line is that this works and looks great.

# Setup
```
sudo pacman -S --needed rust cargo base-devel qt6-base qt6-declarative
git clone https://github.com/llowenthal/rust-audio-profile-toggle.git
cd rust-audio-profile-toggle
cargo run
```