0   set b 84                        b = 84
1   set c b                         c = b
2   jnz a 2                         if a != 0, goto A
3   jnz 1 5                         if 1 != 0, goto B
4   mul b 100                       b = b * 100
A   sub b -100000                   b = b + 100000
6   set c b                         c = b+ 17000
E   set f 1                         f = 1
B   set d 2                         d = 2
10  set e 2                         e= 2
11  set g d                         g = d
D  mul g e                          g = (g * e) -b
14  jnz g 2                         if g != 0, goto C
15  set f 0                         f = 0
16  sub e -1                        e = e+1
C  set g e                          g = e
18  sub g b                         g = g -b
20  jnz g -8                        if g != 0 goto D
21  sub d -1                        d = d +1
22  set g d                         g = d -b
24  jnz g -13                       if g != 0, goto E
25  jnz f 2                          if f != 0, goto F
26  sub h -1
F  set g b
28  sub g c
29  jnz g 2
30  jnz 1 3
31  sub b -17
32  jnz 1 -23