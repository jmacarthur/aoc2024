#!/usr/bin/env python3

field1 = []
field2 = []
with open("input1.txt") as f:
    for l in f.readlines():
        fields = l.split()
        field1.append(int(fields[0]))
        field2.append(int(fields[1]))

print(len(field1))
total = 0

for n in field1:
    count = sum([1 for x in field2 if x==n])
    total += n * count
print(total)
