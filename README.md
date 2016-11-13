# red
ed implementation in rust

Following the manpage for the OpenBSD version of ed, my intention
is to implement the same behavior and functionality present there.

Once this goal has been reached, I may consider developing further. Or
perhaps I will simply move on (maybe come back later). This particular
project is less about need or use and more about practicing Rust.

Much of the basic functionality is already present: can read a file or
command output into the buffer, jump to any line using a very flexible
line addressing scheme including regex search, and write files to disk.

Some notable 'todo's that come to mind:
* incorporate markers in arithmetic addressing (e.g. `'x+4,'x+5p` )
* several commands and functions implemenenting them
    create/change/remove marker
    list lines
    delete line(s)
    substitution
    join lines
    move lines
    print with line numbers
    ... several others
* complete invocation flags and parameters
    

