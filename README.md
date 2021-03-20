# mactags

Read mac file tags on Linux and Windows in addition to Mac

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

62 70 6C 69 73 74 30 30 A1 01 53 61 62 63 08 0A
00 00 00 00 00 00 01 01 00 00 00 00 00 00 00 02
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