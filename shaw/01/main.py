import sys

with open(sys.argv[1]) as f:
    data = f.read()

lines = data.split('\n')

nums = '0123456789'
words = ['one', 'two', 'three', 'four', 'five', 'six', 'seven', 'eight', 'nine']

def see_if_num(word):
    for w in words:
        if w in word:
            return w
    return None


sum = 0
for line in lines:
    for c in line:
        if c in nums:
            sum += (int(c) * 10)
            break
    line = line[::-1]
    for c in line:
        if c in nums:
            sum += int(c)
            break

print("Part 1:", sum)

sum = 0
for line in lines:
    word = ''
    for c in line:
        if c in nums:
            sum += (int(c) * 10)
            break
        word = word + c
        num = see_if_num(word)
        if num != None:
            sum += ((words.index(num) + 1) * 10)
            break

    word = ''
    line = line[::-1]
    for c in line:
        if c in nums:
            sum += int(c)
            break
        word = c + word
        num = see_if_num(word)
        if num != None:
            sum += (words.index(num) + 1)
            break
    

print("Part 2:", sum)