Lightweight desk calculator with arbitrary precision support and decent error diagnostics.

Inspired by Ivy and APL. 

Operator precedence is right-to-left. `2*3+4` is `14`, not `10`.

```
$ robin
> 1 + 2
3
> 7 add 35
42
> 5 - 12
-7
> 6 * 7
42
> neg 7
-7
> 2 * 3 + 4
14
> 1 - 2 - 3
2
> -5 + 12
-17
> 2 pow 10 mul 3
1073741824
> 1234567890 * 9876543210
12193263111263526900
> 2 pow 200
1606938044258990275541962092341162602522202993782792835301376
> 0xff + 0b1011
266
> 0x123ABCdef
4893429231
> 34/10
17/5
> 1/3 + 1/6
1/2
> 1.5 + 1/2
2
> 0.125
1/8
> 1.4e5
140000
> 0x
    ^ error: empty literal
> 1 2
    ^ error: trailing input
> 1 +
     ^ error: unexpected eof
> 1 @ 2
    ^ error: unexpected token
```
