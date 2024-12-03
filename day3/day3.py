#!/usr/bin/env python3

import re

reg1 = re.compile(r"mul\((\d+),(\d+)\)|(do)\(\)|(don't)\(\)")

def count_multiplies(text, with_switch = False):
    total = 0
    on = True
    for multiplies in reg1.findall(text):
        if multiplies[0]=="":
            if multiplies[2] == "do":
                on = True
            else:
                on = False
        elif on or not with_switch:
            total += int(multiplies[0])*int(multiplies[1])
    return total


with open("input3.txt") as f:
    lines = f.readlines()
    text = "".join(lines)

print(f"Total without switching: {count_multiplies(text, False)}")
print(f"Total with switching:    {count_multiplies(text, True)}")
