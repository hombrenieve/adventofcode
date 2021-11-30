#!/usr/bin/env python3

def number_in_list(number, number_list):
    return number in number_list

def find_sum(number, number_list):
    for i in number_list:
        rem = number - i
        if number_in_list(rem, number_list):
            return i, rem
    return None


def find_numbers(number_list):
    head = number_list[0]
    tail = number_list[1:]
    while len(tail) > 0:
        rem = 2020-head
        rest = find_sum(rem, tail)
        if rest != None:
            print(rest + (head,))
            return 0
        head = tail[0]
        tail = tail[1:]
    print("Not found")


with open('input') as f:
    content = f.readlines()
    find_numbers(list(map(int, content)) )