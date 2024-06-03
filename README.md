# MINICEL

## PURPOSE
Taking inspiration from [this video](https://www.youtube.com/watch?v=HCAgvKQDJng&t=9283s), the idea is to implement a batch program that can accept a CSV file that looks like this:
```
A           | B             | C
1           | 2             | Hello
3           | 4             | World
=A1+B1+10   | =A2+B2        |=C1+", "+C2
```     
And outputs:
```
A      | B      | C
1      | 2      | Hello
3      | 4      | World
13     | 7      | Hello, World
```