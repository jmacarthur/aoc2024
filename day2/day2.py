#!/usr/bin/env python3

def sgn(x):
    if x>0:
        return 1
    elif x<0:
        return -1
    return 0

def safe(fields):
    p1 = 0
    p2 = 1
    direction = None
    prev = fields[0]
    for i in range(1,len(fields)):
        delta = fields[i] - prev
        if delta == 0 or abs(delta)>3:
            return False
        elif direction and sgn(delta) != direction:
            return False
        else:
            direction = sgn(delta)
            prev = fields[i]
    return True

with open("input2.txt") as f:
    safe_reports = 0
    for l in f.readlines():
        fields = [int(field) for field in l.split()]
        print(fields)
        if safe(fields):
            print(f"{fields} => safe")
            safe_reports += 1
        else:
            for j in range(0,len(fields)):
                reduced_fields = fields[:j] + fields[j+1:]
                if safe(reduced_fields):
                    safe_reports += 1
                    break

print(safe_reports)
