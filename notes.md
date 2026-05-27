# Notes

### Things to keep in mind


***Let MySQL handle filtering, sorting, and limiting***

*Only pull into rust what is necessary to complete the objective*

Since the database size is large (~200GB), it's important to optimize the code whenever allowed for quicker enumeration. Because I'm running a seperate MySQL server caching allows for quick data fetching, however, the overhead of transfering the data into the program will still result in significant overhead.  

***Utilize indexed keys in queries whenever allowed.***

This statement 
```SQL
mysql> SELECT COUNT(*) FROM categorylinks WHERE cl_type = 'subcat';
+----------+
| COUNT(*) |
+----------+
| 10075205 |
+----------+
1 row in set (5 min 43.34 sec)
```
Takes 5 minutes because the keys only filter 33.33% of the data
```SQL
mysql> EXPLAIN SELECT COUNT(*) FROM categorylinks WHERE cl_type = 'subcat';
+----+-------------+---------------+------------+-------+---------------+---------------+---------+------+-----------+----------+--------------------------+
| id | select_type | table         | partitions | type  | possible_keys | key           | key_len | ref  | rows      | filtered | Extra                    |
+----+-------------+---------------+------------+-------+---------------+---------------+---------+------+-----------+----------+--------------------------+
|  1 | SIMPLE      | categorylinks | NULL       | index | cl_sortkey_id | cl_sortkey_id | 245     | NULL | 213242429 |    33.33 | Using where; Using index |
+----+-------------+---------------+------------+-------+---------------+---------------+---------+------+-----------+----------+--------------------------+
```
