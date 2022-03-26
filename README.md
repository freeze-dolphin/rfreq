# rfreq

a simple tool for word frequency statistics.

## usage

just follow the help message:

```log
> rfreq --help

USAGE:
    rfreq [OPTIONS] --filter <FILTER> --input <INPUT>

OPTIONS:
    -c, --case-sensitive     Consider different cases as different words
    -f, --filter <FILTER>    Regex filter
    -h, --help               Print help information
    -i, --input <INPUT>      File to perform analysis on
    -n, --no-sorting         Disable sorting
    -r, --raw-print          Print result without using table form
    -V, --version            Print version information
```

for small files, you can use this in a normal way  
and for big files, it is recommended to do like this: 

```shell
rfreq -f "your regex" -i "filename" | tee result.txt
```
