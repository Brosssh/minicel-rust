# MINICEL

## PURPOSE
Taking inspiration from [this video](https://www.youtube.com/watch?v=HCAgvKQDJng&t=9283s), the idea is to implement a batch program that can accept a CSV file that looks like this:
```
A      | B
1      | 2
3      | 4
=A1+B1 | =A2+B2
```
And outputs:
```
A      | B
1      | 2
3      | 4
3      | 7
```