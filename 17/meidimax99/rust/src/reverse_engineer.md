2,4,    Bst     Store Value of Register A % 8 in B
1,3,    Bxl     Xor Value register B with 3
7,5,    Cdv     c = a / 2**b
0,3,    Adv     a = a / 2**3
4,3,    Bxc     b = b ^ c
1,5,    Bxl     b = b ^ 5
5,5,    Out     print b % 8
3,0     Jnz     jump to 0



- Store 3 LSb of A in B
- Flip 2 LSb of B
- Right shift a by b bits, store to c
- Right shift a by 3 bits, store to a
- xor b and c, store to b
- xor b with 5 ( 101), store to b
- print b % 8



b = ( ( ( (a % 8) ^ 3) ^ ( a >> ( ( a % 8 )^ 3) ) ) ^ 5)



000     011     3
001     010     2
010     001     1
011     000     0
100     111     7
101     110     6
110     101     5
111     100     4

0 -> Only current group impacts result
1-3 -> Current and next group
4-6 -> Current and second next
7 Current and third next

- Operations are on 3 bit groups
- But the 


LSB -> MSB


Map desired number -> Possible bitpatterns
Overlapping bit patterns with size 3-10 bits

