# xidff success run

```trycmd
$ xdiff-live run -p todo -c fixtures/test.yml -e a=100 -e @b=2 -e %c=3 -e m=10
1   1    | HTTP/2.0 200 OK
2   2    | content-type: "application/json; charset=utf-8"
3        |-content-length: "[..]"
    3    |+content-length: "[..]"
4   4    | x-powered-by: "[..]"
5   5    | x-ratelimit-limit: "[..]"
6   6    | vary: "Origin, Accept-Encoding"
--------------------------------------------------------------------------------
9   9    | pragma: "no-cache"
10  10   | expires: "-1"
11  11   | x-content-type-options: "nosniff"
12       |-etag: "[..]"
    12   |+etag: "[..]"
13  13   | via: "1.1 vegur"
14  14   | accept-ranges: "bytes"
15  15   | nel: "[..]"
--------------------------------------------------------------------------------
18  18   | 
19  19   | {
20  20   |   "completed": false,
21       |-  "title": "delectus aut autem",
    21   |+  "title": "quis ut nam facilis et officia qui",
22  22   |   "userId": 1
23  23   | }

```
