# mactags

Manipulate Mac file tags on Linux in addition to Mac

## Installation

For Rustaceans:

```bash
cargo install --path .
```

## The format of Mac's `metadata:\_kMDItemUserTags

https://eclecticlight.co/2017/12/27/xattr-com-apple-metadata_kmditemusertags-finder-tags/

**Examples**

```
Red

62 70 6C 69 73 74 30 30 A1 01 55 52 65 64 0A 36
08 0A 00 00 00 00 00 00 01 01 00 00 00 00 00 00
00 02 00 00 00 00 00 00 00 00 00 00 00 00 00 00
00 10

bplist00[A10155}Red\n6[080A]...
```

```
abc

62 70 6C 69 73 74 30 30 A1 01 53 61 62 63
08 0A 00 00 00 00 00 00 01 01 00 00 00 00 00 00 00 02
00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 0E

bplist00[A1][01][53]abc[080A]...
```

```
abc[red]

62 70 6C 69 73 74 30 30 A1 01 55 61 62 63 0A 36
08 0A 00 00 00 00 00 00 01 01 00 00 00 00 00 00
00 02 00 00 00 00 00 00 00 00 00 00 00 00 00 00
00 10

bplist00[A1][01][55]abc\n6[080A]...
```

```
abc[red], def, ghi[yellow]

62 70 6C 69 73 74 30 30       // bplist00
A3 01 02 03
55
61 62 63 0A 36                // abc\n6
53
64 65 66                      // def
55
67 68 69 0A 35                // ghi\n5
08 0C 12 16
00 00 00 00 00 00 01 01 00 00 00 00 00 00 00 04
00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 1C
```

```
abc[red], def, ghi[yellow], 中国, ひらがな[green], ประเทศไทย

62 70 6C 69 73 74 30 30       // bplist00
A6 01 02 03 04 05 06
55
61 62 63 0A 36                // abc\n6
53
64 65 66                      // def
55
67 68 69 0A 35                // ghi\n5
62
4E2D 56FD                     // \utf16{中国}
66
3072 3089 304C 306A 000A 0032 // \utf16{ひらがな\n2}
69
0E1B 0E23 0E30 0E40 0E17 0E28 0E44 0E17 0E22
                              // \utf16{ประเทศไทย}
08 0F 15 19 1F 24 31
00 00 00 00 00 00 01 01 00 00 00 00 00 00 00 07 00 00 00 00 00
00 00 00 00 00 00 00 00 00 00 44
```

```
abcdefghijklmnopqrstuvwxyz

62 70 6C 69 73 74 30 30       // bplist00
A1 01
5F 10 1A                      // 5F 10 0F for 15 chr; 5E for 14 chr
                              // 6F 10 0F for 15 chr of utf16; 6E for 14 chr
61 62 63 64 65 66 67 68 69 6A 6B 6C 6D 6E 6F 70 71 72 73
74 75 76 77 78 79 7A          // abcdefghijklmnopqrstuvwxyz
08 0A
00 00 00 00 00 00 01 01 00 00 00 00 00 00 00 02 00 00 00 00 00 00 00
00 00 00 00 00 00 00 00 27
```


### Trailing mysterious bytes

```
08 00 00 00000000                             010100000000000000 0100000000000000000000000000000009    null
08 0A 00 0000000000                           010100000000000000 020000000000000000000000000000000C    a
08 0A 00 0000000000                           010100000000000000 020000000000000000000000000000000C    c
08 0B 0D 000000000000                         010100000000000000 030000000000000000000000000000000F    a, b
08 0C 0E 10000000000000                       010100000000000000 0400000000000000000000000000000012    a, b, c
08 0D 0F 1113000000000000                     010100000000000000 0500000000000000000000000000000015    a, b, c, d
08 0C 0E 10000000000000                       010100000000000000 0400000000000000000000000000000013    a, b, cd
08 0D 0F 1114000000000000                     010100000000000000 050000000000000000000000000000001A    a, b, cd, abc
08 16 18 1E243138414F5C6A6E757E000000000000   010100000000000000 0E00000000000000000000000000000081
08 17 19 1F25323942505D6B6F767F82000000000000 010100000000000000 0F00000000000000000000000000000087
```