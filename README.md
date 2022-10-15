# bf_and_bf2bf
Brainfuck/BrainFNORD2 interpreter and Brainfuck/BrainFNORD2 to Brainfuck/BrainFNORD2 converter

# About
I was looking at the esolang BrainFNORD: https://esolangs.org/wiki/BrainFNORD
and I noticed how absolutely random it was. Now I like a good bit of chaos, don't get me wrong, but I saw very quickly that it was going to very rarely place the words hail and eris next to each other, and this was the kind of bad chaos that we should avoid. so to bring balance towards a more hodge podge sort of approach I endeavored to lean on the scales a little and make a very serendipitous probability of the hail eris occurance to... occur.

and thus brainFNORD2 was born, simply a reshuffling of symbols and the like to make nice patterns that I like...

# Running
both programs are rust code, barely tested, not likely to work under most circumstances, but hello world seems to work, so I'm happy.
download the zip/repo go to either bf directory or bf2bf directory depending on whether you need an interpreter (bf) or the crosscompiler (bf2bf)
run ```cargo run``` or some such thing, and see what happens.

if I figure out github, I may add the compiled versions to release. just have a look around, see if I was lazy or not.

helpful video: https://www.youtube.com/watch?v=vHkKyY_Hpuw

# Implementation
using this: https://github.com/arkark/15puzzle-brainfuck
and this code:
```
with open("15puzzle.bf.txt") as fi:
	lines = fi.readlines()
line = lines[0]
res = {}
for i, x in enumerate(line):
    if i+1 == len(line):
        break
    res[str(line[i]+line[i+1])] = 0
for i, x in enumerate(line):
    if i+1 == len(line):
        break
    res[str(line[i]+line[i+1])] = res[str(line[i]+line[i+1])] + 1
```
and then sorting with this:
```
dict(sorted(res.items(), key=lambda item: item[1]))
```
I created this: 
```
{'..': 1,
 '.+': 1,
 '.\n': 1,
 '-+': 3,
 '<.': 4,
 '+.': 8,
 '-.': 8,
 '.>': 8,
 '.-': 10,
 '.[': 12,
 '-[': 12,
 '>,': 16,
 ',<': 16,
 '<>': 20,
 '.<': 32,
 '>.': 44,
 '<]': 49,
 '><': 52,
 '[[': 81,
 '>]': 81,
 ']+': 105,
 ']]': 110,
 '->': 116,
 '[-': 196,
 '+[': 214,
 '-<': 214,
 '<[': 350,
 '--': 368,
 '[<': 440,
 ']<': 454,
 '>-': 508,
 '>+': 515,
 '+>': 521,
 '[>': 550,
 '+<': 568,
 ']>': 598,
 '>[': 598,
 '<-': 666,
 '<+': 687,
 '-]': 1027,
 '++': 2572,
 '<<': 8907,
 '>>': 8912}
```
^brainfuck digram sorted occurences^

to find the most common digram with 2 separate symbols: -]
and then altered BrainFNORD accordingly to make hail eris a common pairing

from this:
```
23              [
5               ]
pineal          <
fnord           >
kallisti        .
chaos           ,
hail            +
eris            -
```
to this:
```
fnord           <
kallisti        >
pineal          .
chaos           ,
23              [
5               +
hail            -
eris            ]
```
other changes also being occurence related

