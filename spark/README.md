# spark

a hack version of [spark](https://github.com/holman/spark)

![spark](../images/spark.png)

## python

same as the origin. but because of some unknown problem, can't work with `shell pipeline`.
use `$()` can easily work same as pipeline.
```
not works:
awk '{ print length($0) }' ../print_tables/print_tables.py | grep -Ev 0 | ./spark.py

works:
./spark.py $( awk '{ print length($0) }' ../print_tables/print_tables.py | grep -Ev 0  )
```
