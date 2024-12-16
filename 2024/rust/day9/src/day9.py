import sys
from collections import deque

def Checksum(arr):
    total = 0
    for n, num in enumerate(arr):
        if num != None:
            total += n * num
    return total

with open('../input.txt', 'r') as file:
    L = file.read().split()

SPACES =  []
FILES = []
l = L[0]
pos = 0
line = []
file = True
fid = 0
for ch in list(l):
    if file == True:
        line.extend([fid for _ in range(int(ch))])
        FILES.append((fid, int(ch), pos))
        fid += 1
    else:
        line.extend([None for _ in range(int(ch))])
        SPACES.append((pos, int(ch)))
    pos += int(ch)
    file ^= True
LEN = pos

dq = deque(line)
nl = []
while dq:
    h = dq.popleft()
    if h == None:
        while dq and h == None:
            h = dq.pop()
    if h != None:
        nl.append(h)
print(Checksum(nl))

p2 = [None] * LEN

for item in reversed(FILES):
    num, req, atpos = item
    moved = False
    for s in range(len(SPACES)):
        spos, slen = SPACES[s]
        if spos > atpos:
            break
        if slen >= req:
            pos = spos
            SPACES[s] = (spos + req, slen - req)
            for _ in range(req):
                p2[pos] = num
                pos += 1
            moved = True
            if SPACES[s][1] == 0:
                del SPACES[s]
            break

    if not moved:
        pos = item[2]
        for _ in range(req):
            p2[pos] = item[0]
            pos += 1
print(Checksum(p2))
