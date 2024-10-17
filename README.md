# showme
SHamelessly Observing Whatever Memory Exhibits

This is simple tool that is showing you the contents of the memory and what has changed:

showme 0.1.0

Tool for monitoring changes in memory ranges


USAGE:

    showme [OPTIONS] --pid <PID>


OPTIONS:

    -h, --help                   Print help information

    -i, --interval <INTERVAL>    Interval to scan in milliseconds [default: 100]

    -o, --offset <OFFSET>        Where to start monitoring [default: 0]

    -p, --pid <PID>              PID of the process used to attach to memory

    -s, --size <SIZE>            Size of the memory to monitor [default: 256]

    -V, --version                Print version information


EXAMPLE: 

Watch PID 2271 at offset 6223840552 - press CTRL-C for end


1056b41a01000000        1056b41a01000000        c057b41a01000000        c057b41a01000000

4845f87201000000        4845f87201000000        5845f87201000000        5845f87201000000

010004001f220100        0000000000000000        0000000000000000        0000000000000000

68c4660000000000        0000000000000000        0000000000000000        0000000000000000

a9b1660000000000        68c4660000000000        43010000c9530000        68c4660000000000

0000000001000000        0100000000000000        e060481a01000000        e060481a01000000

0100000003000000        158900010a000000        1f22010000000001        0100200000000000

0000000000000000        7847f87201000000        a844f87201000000        0000f207caeb1067


Changed offsets:

@0       :    108618026

@8       :    108618026

@16      :    c08718026

@24      :    c08718026

@96      :    68

@136     :    68196102

@144     :    431

@148     :    c983

@152     :    68

@164     :    01

@216     :    01

@250     :    f27202

