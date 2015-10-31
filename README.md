lib netrt
=========

*⚠️  This is pre-alpha stage and in heavy development ! You have been warned.*

The netrt library is a direct implementation of 
[the Glenn Fiedler article series about Networking for Game Programmers](http://gafferongames.com/networking-for-game-programmers/udp-vs-tcp/)
which design a Real-Time protocol on UDP.

You could need netrt if you have real time constraints on your network
communication, which means that you need the data to arrive in a specific time
frame after which they are no longuer of interest (like input commands in
a mutliplayer FPS).

This library use the standard library with as few dependencies as possible and
minimum runtime overhead.

If you need low-level networking, check [libpnet](https://github.com/libpnet/libpnet/).

### Licence

This library is distributed under the Mozilla Public Licence 2.0 (MPL2). Please
check [the official FAQ](https://www.mozilla.org/en-US/MPL/2.0/FAQ/) for
a readable how-to-distribute correctly this library.

Non autoritative summary:

- For all private use or software projects with internal distribution, you have
  nothing to do.  
- For software projects with distribution outside of your organization:
  - If the source is NOT MODIFIED: you need to list publicly this library as
    your dependency with a link to this repository.
  - If the source is MODIFIED: you need to distribute the modified part of the
    MPL-licensed source code under the MPL.
  - You can use whatever licence you want on all the other source code or
    softwares.
