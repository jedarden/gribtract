# bf-28dl: wgrib2 Installation

## Summary
wgrib2 v3.1.3 has been successfully installed on the system.

## Binary Details
- **Path**: `/home/coding/.local/bin/wgrib2`
- **Version**: v3.1.3 (October 2023)
- **Size**: 3.9M
- **Build**: Stock build
- **Authors**: Wesley Ebisuzaki, Reinoud Bokhorst, John Howard, and others

## Installation Process
1. Found pre-existing `wgrib2.tgz` archive in workspace (28M)
2. Extracted archive to `grib2/wgrib2/` directory
3. Discovered pre-compiled `wgrib2` binary (already built from previous attempt)
4. Copied binary to `~/.local/bin/wgrib2` with executable permissions
5. Verified installation with `which wgrib2` and help output

## Verification
```bash
$ which wgrib2
/home/coding/.local/bin/wgrib2

$ wgrib2
wgrib2 v3.1.3 10/2023  Wesley Ebisuzaki, Reinoud Bokhorst, John Howard, Jaakko Hyvätti, Dusan Jovic, Daniel Lee, Kristian Nilssen, Karl Pfeiffer, Pablo Romero, Manfred Schwarb, Gregor Schee, Arlindo da Silva, Niklas Sondell, Sam Trahan, George Trojan, Sergey Varlamov
   stock build
```

## Acceptance Criteria Met
- ✅ wgrib2 command is available in PATH
- ✅ wgrib2 responds to help flags (displays version and usage)
- ✅ wgrib2 binary path noted for later use (`~/.local/bin/wgrib2`)

## Source
Binary was built from the COLA wgrib2 source distribution (`wgrib2.tgz`).
