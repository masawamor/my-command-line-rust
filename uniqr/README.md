
Usage: uniq [OPTION]... [INPUT [OUTPUT]]

  -c, --count           prefix lines by the number of 

```bash
$ cat tests/inputs/three.txt 
a
a
b
b
a
c
c
c
a
d
d
d
d

$ uniq -c tests/inputs/three.txt 
   2 a
   2 b
   1 a
   3 c
   1 a
   4 d

```


