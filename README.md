This is a test to see whether it's faster to directly read an atomic bool and decide what to do
each time, or to cache a function pointer when possible.

This seems to suggest that they're equally fast, except when the atomic bool needs to be read *and*
a function ptr called based on the result, which is pretty much what I expected.

My results (AMD Threadripper 2950X):

```
ptr true                time:   [2.3291 ns 2.3353 ns 2.3425 ns]
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high mild
                                                        
ptr false               time:   [2.1218 ns 2.1305 ns 2.1410 ns]
Found 4 outliers among 100 measurements (4.00%)
  3 (3.00%) high mild
  1 (1.00%) high severe


cached ptr true         time:   [1.2003 ns 1.2055 ns 1.2130 ns]
Found 5 outliers among 100 measurements (5.00%)
  3 (3.00%) high mild                   
  2 (2.00%) high severe               

cached ptr false        time:   [1.1955 ns 1.1995 ns 1.2042 ns]
Found 5 outliers among 100 measurements (5.00%)
  4 (4.00%) high mild            
  1 (1.00%) high severe                     


direct true             time:   [1.2014 ns 1.2039 ns 1.2064 ns]
Found 2 outliers among 100 measurements (2.00%)
  2 (2.00%) high mild

direct false            time:   [1.1814 ns 1.1843 ns 1.1875 ns]
Found 8 outliers among 100 measurements (8.00%)
  6 (6.00%) high mild
  2 (2.00%) high severe
```
