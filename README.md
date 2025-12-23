# Hyprlayout

## Purpose
I'm building my shell using Quickshell and there was no easy way to get current layout on shell startup except `hyprctl devices -j` and I had to watch socket2 changes for layout. So I decided to build this application that gets current layout on application startup and after that it listens to socket2 active layout changes.

## Why do I need it? 
Because I don't want to have two Processes in quickshell for startup and for sock2 changes listener.
