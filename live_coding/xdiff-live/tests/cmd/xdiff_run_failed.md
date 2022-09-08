# xdiff failed run

```trycmd
$ xdiff-live run -p todo1 -c fixtures/bad.yml -e a=100 -e @b=2 -e %c=3 -e m=10
failed to validate profile: todo1

Caused by:
    0: req1 failed to validate
    1: Params must be an object but got
       abcd
       
```
