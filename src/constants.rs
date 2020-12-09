// #region day 1
pub const DAY_ONE: [usize; 200] = [1891,1975,1987,1923,1928,1993,1946,1947,2005,1897,1971,1929,1875,1945,1680,811,1901,1396,1942,1282,1941,1978,1884,1879,1230,2010,1881,1979,1996,1904,1934,1865,2003,2006,1966,1860,1259,1959,1931,1963,1878,1880,151,1925,1663,1908,1863,1391,1922,1968,1998,1084,1982,1960,1938,1876,1937,1882,1873,1926,1986,1416,1864,1862,1969,1913,532,1866,1242,1933,1903,965,1927,1890,1991,1388,1992,1902,1907,1964,1394,2009,1920,630,1932,1854,1951,1852,1983,1314,1855,1954,1921,1989,1871,1995,1885,1974,1915,1872,1251,1899,1985,1889,1935,1912,946,1965,1739,1973,1911,1910,1917,1918,1900,1886,1477,2000,1916,1077,2004,1456,1867,1970,1999,1919,1726,706,1930,1994,1988,1997,1870,1953,652,1893,1898,1883,1957,1972,1874,1977,1955,2001,1906,1389,1848,1940,1877,1962,1948,1887,1924,1403,1408,1861,1892,1990,1222,677,1392,1113,1085,1894,1106,1939,1961,1944,1952,1643,1404,1895,1958,1976,1206,1905,1076,1888,1896,1943,1950,2008,1967,164,1981,1868,1914,1909,1956,341,1379,2007,1563,1980,1072,1949,1250,1258,1092,2002];
// #endregion

// #region day 2

pub const DAY_TWO: [(usize, usize, char, &str); 1000] = [
(8, 11, 'l', "qllllqllklhlvtl"),
(1 as usize, 3 as usize, 'm', "wmmmmmttm"),
(2 as usize, 4 as usize, 'p', "pgppp"),
(11 as usize, 12 as usize, 'n', "nnndnnnnnnnn"),
(17 as usize, 19 as usize, 'q', "qprqdcgrqrqmmhtqqvr"),
(16 as usize, 17 as usize, 'k', "nphkpzqswcltkkbkk"),
(6 as usize, 9 as usize, 'c', "rvcvlcjcbhxs"),
(18 as usize, 20 as usize, 'v', "hbjhmrtwzfqfvhzjjvcv"),
(5 as usize, 9 as usize, 'z', "jzzhzttttnz"),
(7 as usize, 13 as usize, 'd', "bdqdtddddnwdd"),
(9 as usize, 11 as usize, 'd', "ddddddddxdldddd"),
(6 as usize, 10 as usize, 'f', "fblhfdztgf"),
(2 as usize, 11 as usize, 'b', "vszxfnwghcb"),
(15 as usize, 18 as usize, 'n', "nbnmwxnnlkmlknnnhn"),
(2 as usize, 9 as usize, 'z', "lhwqvczrrqqhqlfvkbcm"),
(15 as usize, 16 as usize, 'd', "dndddddddjdddddbdld"),
(7 as usize, 8 as usize, 'k', "kkkmkkkf"),
(1 as usize, 8 as usize, 'p', "rdcmrkbwqjpph"),
(2 as usize, 6 as usize, 's', "cswdpsjgsfvzkvqqmrqf"),
(9 as usize, 11 as usize, 'm', "mmmmmmmzbmmmv"),
(8 as usize, 9 as usize, 'j', "jjjjjjjjfj"),
(7 as usize, 8 as usize, 'd', "dddsjnds"),
(1 as usize, 4 as usize, 'f', "qffb"),
(3 as usize, 8 as usize, 'f', "cphmtfff"),
(1 as usize, 13 as usize, 's', "rjsscssstsvssss"),
(9 as usize, 14 as usize, 's', "gtsnlbqnckhxmssbbs"),
(12 as usize, 14 as usize, 'j', "jfjnjbjrjpdndj"),
(15 as usize, 16 as usize, 't', "tttttttttttttttwt"),
(7 as usize, 8 as usize, 'r', "rgrdrrrrrrrrjhrrrrrr"),
(5 as usize, 8 as usize, 't', "lpcqfgzttlt"),
(1 as usize, 12 as usize, 'r', "wrrrrrrjrrrrrrrr"),
(14 as usize, 19 as usize, 'd', "ddvcdddddddhddprldl"),
(4 as usize, 8 as usize, 'd', "pkddlzxsl"),
(7 as usize, 11 as usize, 'x', "xhxqxcfkxwxxnm"),
(3 as usize, 7 as usize, 'q', "qqqqqqjqqqd"),
(3 as usize, 13 as usize, 's', "rtzsktsdfhtbs"),
(8 as usize, 15 as usize, 'n', "nnnnnnnnknnnnnsnnn"),
(10 as usize, 13 as usize, 'r', "rrrrrrrrrrrrnr"),
(1 as usize, 9 as usize, 'r', "ldfdgzprnptrd"),
(2 as usize, 3 as usize, 'k', "rqkthj"),
(4 as usize, 7 as usize, 'p', "prrpswdpnmpxmjzsp"),
(12 as usize, 13 as usize, 'p', "pmwbptnpppjprfpkppgj"),
(4 as usize, 6 as usize, 'w', "cfwdlw"),
(2 as usize, 9 as usize, 'r', "pnnvrfjhz"),
(14 as usize, 16 as usize, 'b', "bbbbbbbbbbbbbtbbb"),
(2 as usize, 7 as usize, 'l', "xlmzgklxljcl"),
(1 as usize, 6 as usize, 'c', "cccccccc"),
(11 as usize, 12 as usize, 'w', "dmpzfzpwwnwwpggw"),
(2 as usize, 3 as usize, 'c', "xrccccmcc"),
(12 as usize, 13 as usize, 'k', "kkkkkkkvkkkvkknkkk"),
(10 as usize, 12 as usize, 'h', "hhnhcvhhhqhh"),
(17 as usize, 18 as usize, 'd', "dddjddbdzdddvddddw"),
(1 as usize, 5 as usize, 'p', "pppphp"),
(11 as usize, 13 as usize, 'v', "fvvvvjlvbvrvdhbvv"),
(10 as usize, 14 as usize, 'b', "bzxxqcgqnbkmhm"),
(1 as usize, 14 as usize, 'g', "xggghgngqnggggggggg"),
(9 as usize, 10 as usize, 's', "xslsmpfnxvvssqmgf"),
(16 as usize, 17 as usize, 's', "nfqggjzbfsssllwns"),
(9 as usize, 10 as usize, 'w', "wwwwwwwwfw"),
(13 as usize, 15 as usize, 'z', "zzzjzzzjzzzzzzgz"),
(7 as usize, 10 as usize, 'n', "jfnwgwnnnn"),
(4 as usize, 5 as usize, 'b', "btbqb"),
(1 as usize, 5 as usize, 'w', "vwflw"),
(15 as usize, 16 as usize, 'v', "vvxvdvvvvzvxmhxvv"),
(3 as usize, 4 as usize, 'b', "cgbbqk"),
(1 as usize, 3 as usize, 'f', "bffffdfclfffgfkf"),
(6 as usize, 11 as usize, 'm', "xckgmdcqmwk"),
(6 as usize, 9 as usize, 'c', "vcptncxbcg"),
(5 as usize, 6 as usize, 'm', "mmmmdm"),
(2 as usize, 3 as usize, 'd', "dmdd"),
(5 as usize, 7 as usize, 'v', "vvhrxkd"),
(7 as usize, 10 as usize, 'b', "bbbbbbqbbbbb"),
(8 as usize, 9 as usize, 'm', "mhvmmwlgm"),
(3 as usize, 4 as usize, 'x', "xvtxkz"),
(3 as usize, 9 as usize, 'w', "wwlwwwwwkqww"),
(4 as usize, 18 as usize, 'g', "mxslljzcgpwsrggqqc"),
(2 as usize, 3 as usize, 'x', "xcff"),
(16 as usize, 17 as usize, 'j', "jjjjjjjjjjfqjfjwhjjj"),
(9 as usize, 13 as usize, 'p', "ppppppppgppppp"),
(16 as usize, 17 as usize, 'f', "fffffffffffffffffff"),
(3 as usize, 5 as usize, 'c', "cncpcck"),
(9 as usize, 11 as usize, 'c', "kzcwczccccmcfsrcc"),
(3 as usize, 7 as usize, 's', "ssdsssvnsssssjs"),
(1 as usize, 6 as usize, 'v', "vvvvqv"),
(5 as usize, 7 as usize, 'b', "fzbbxbbbgbb"),
(3 as usize, 9 as usize, 't', "gtttttttftt"),
(5 as usize, 19 as usize, 'p', "ngpnpklwsclptfjvtgm"),
(3 as usize, 4 as usize, 'd', "dddtdddddd"),
(4 as usize, 5 as usize, 'm', "mjmmwl"),
(11 as usize, 13 as usize, 'l', "lllblllllvllrl"),
(2 as usize, 6 as usize, 'h', "cphqvz"),
(17 as usize, 19 as usize, 'w', "gwwrvfglsljwfgxwbbw"),
(15 as usize, 17 as usize, 'x', "gfcxzcwgjmkwfqxrxzrd"),
(13 as usize, 14 as usize, 'w', "lrmhhxwfkwnkwnbsq"),
(7 as usize, 8 as usize, 'f', "ffffffgcff"),
(11 as usize, 19 as usize, 'v', "vvvvvvvvvvzvvvvvvvlv"),
(7 as usize, 8 as usize, 'k', "kxkkkkpk"),
(7 as usize, 14 as usize, 'v', "vfvvvskttcvvvvvfvv"),
(11 as usize, 12 as usize, 'm', "mmmdhmmgmkgmjmr"),
(1 as usize, 7 as usize, 'p', "hpppppppppbnc"),
(3 as usize, 8 as usize, 'n', "rttbbpjnmzn"),
(8 as usize, 9 as usize, 'n', "nrfnnvxrp"),
(3 as usize, 4 as usize, 'x', "tnxnngq"),
(9 as usize, 12 as usize, 's', "mbhsxshssrtwvm"),
(11 as usize, 15 as usize, 'n', "nwmnlhgjnnptkmn"),
(1 as usize, 4 as usize, 'x', "xmxl"),
(1 as usize, 6 as usize, 'f', "bffffffffff"),
(2 as usize, 4 as usize, 'r', "zrrr"),
(2 as usize, 3 as usize, 't', "ttmwvt"),
(3 as usize, 5 as usize, 'n', "ngnnr"),
(13 as usize, 17 as usize, 'p', "jtpppfgklkpshpndpp"),
(1 as usize, 7 as usize, 'r', "rrrrrrrrrrrr"),
(7 as usize, 10 as usize, 'h', "hmhhhhhhhzhhhhhhh"),
(9 as usize, 15 as usize, 'l', "glxqscckgxtkzfllk"),
(2 as usize, 3 as usize, 's', "gfsh"),
(5 as usize, 6 as usize, 'b', "dpphbj"),
(6 as usize, 13 as usize, 'h', "hhhhhhhhhhhbshgh"),
(5 as usize, 13 as usize, 'd', "dddpbddddddddddd"),
(10 as usize, 11 as usize, 'p', "pppppppppdppppp"),
(5 as usize, 7 as usize, 'j', "jjjjjcx"),
(8 as usize, 9 as usize, 'r', "rrrrmrrrm"),
(1 as usize, 3 as usize, 'f', "ffff"),
(8 as usize, 15 as usize, 'b', "bbbbbbbgbbbbbbrb"),
(3 as usize, 6 as usize, 'h', "hhrhhhhhhh"),
(9 as usize, 10 as usize, 'v', "tkwvvvjvvvblvxvxhxvv"),
(5 as usize, 9 as usize, 't', "ttttttttmtf"),
(1 as usize, 4 as usize, 'm', "fmmmmm"),
(5 as usize, 9 as usize, 'n', "nfnlnkblnnfnxtzn"),
(3 as usize, 11 as usize, 'v', "dnsvvvbvnvvxvj"),
(8 as usize, 13 as usize, 'h', "nfgmbfjhdhlhb"),
(2 as usize, 14 as usize, 'q', "cxtgcrpsxnjshlqbh"),
(5 as usize, 13 as usize, 'b', "bbbbjbbbbbbbbbbbbbb"),
(4 as usize, 5 as usize, 'l', "wldljllcl"),
(19 as usize, 20 as usize, 'b', "jbvbbbbbqqbbbbbbbbbs"),
(5 as usize, 14 as usize, 'b', "bbbbbbbbbbbbbbbbb"),
(2 as usize, 5 as usize, 'q', "qmqjqhfk"),
(12 as usize, 13 as usize, 's', "sssfsnsssssssxsms"),
(3 as usize, 17 as usize, 'z', "zzzzzzzzzzzzzzzznzz"),
(13 as usize, 14 as usize, 't', "ttztttstttbttb"),
(2 as usize, 6 as usize, 'q', "vqhhrqlgvckvsrpwmqwz"),
(15 as usize, 16 as usize, 'd', "dddddddddddddddhd"),
(5 as usize, 6 as usize, 'w', "wwwwdwww"),
(1 as usize, 2 as usize, 'f', "jffkf"),
(6 as usize, 10 as usize, 'j', "jjjjjrjwjbjxgpjjjm"),
(4 as usize, 5 as usize, 'c', "cvgccmcqzrcd"),
(3 as usize, 7 as usize, 'h', "mghkhfgzmkz"),
(10 as usize, 17 as usize, 'f', "ffffsfffffffpffwkff"),
(4 as usize, 9 as usize, 'g', "ggbgcgjgggggg"),
(8 as usize, 11 as usize, 'd', "mdnddhpddddm"),
(6 as usize, 7 as usize, 'c', "ctcldgc"),
(13 as usize, 14 as usize, 'm', "mmmmmmmmmmmmlmm"),
(18 as usize, 19 as usize, 't', "ttttttttptttttnttgt"),
(7 as usize, 8 as usize, 'g', "qngggtghnxggs"),
(3 as usize, 12 as usize, 'c', "scccrbjtdccq"),
(2 as usize, 3 as usize, 'q', "qgqjq"),
(5 as usize, 6 as usize, 'x', "xxxxzxxxxx"),
(6 as usize, 10 as usize, 'v', "nwjsxvzhvgmsglftbpvc"),
(2 as usize, 5 as usize, 'm', "xjtbsffdwmxmhxrmpm"),
(4 as usize, 7 as usize, 'c', "mfgcvqccg"),
(1 as usize, 7 as usize, 'c', "cctcccc"),
(3 as usize, 4 as usize, 'm', "cnnw"),
(10 as usize, 13 as usize, 's', "sstssssszsssss"),
(9 as usize, 10 as usize, 'h', "hchhhhjhdh"),
(2 as usize, 4 as usize, 'j', "xjjgsz"),
(4 as usize, 11 as usize, 'b', "rbblbbpmbmbdbjgcbhk"),
(10 as usize, 11 as usize, 'r', "rrrrrrrrrjrrr"),
(3 as usize, 6 as usize, 'g', "hvgfzgjrkdf"),
(1 as usize, 10 as usize, 'b', "tbbbbbbpbbbbbb"),
(12 as usize, 13 as usize, 'n', "nhnnnnnnnnjnznnrnrxl"),
(7 as usize, 13 as usize, 'w', "wwmwfncfwdxww"),
(3 as usize, 4 as usize, 'c', "wccq"),
(7 as usize, 16 as usize, 'x', "xxxxxxtxxxxxxxxxx"),
(3 as usize, 7 as usize, 'k', "zmhkssxs"),
(1 as usize, 8 as usize, 'n', "gnnnnnnnnn"),
(13 as usize, 14 as usize, 'r', "rrrrrrrrrrrrmr"),
(2 as usize, 3 as usize, 'r', "rrlr"),
(3 as usize, 4 as usize, 'c', "sccccwvpjpplgctg"),
(1 as usize, 2 as usize, 'b', "svbs"),
(1 as usize, 2 as usize, 'f', "fbfffrff"),
(6 as usize, 14 as usize, 'l', "lllllvlllllxtllqllll"),
(13 as usize, 14 as usize, 'f', "ffffffffkfffjf"),
(1 as usize, 6 as usize, 'z', "zzzrzfzzzzm"),
(4 as usize, 8 as usize, 'k', "kzzktkgrzjdkq"),
(6 as usize, 8 as usize, 'j', "jjjfsgjjbt"),
(3 as usize, 7 as usize, 'q', "qtqqqqq"),
(17 as usize, 18 as usize, 'x', "djxqkrlcwxxxlvhjxh"),
(6 as usize, 8 as usize, 'c', "tqpcgcjc"),
(2 as usize, 13 as usize, 'f', "khqhkkszblvffhfwcg"),
(7 as usize, 18 as usize, 'p', "kjxrtcpptzpxddbkts"),
(5 as usize, 12 as usize, 'l', "llldllllllntm"),
(2 as usize, 7 as usize, 'q', "bvvrnhqhpqw"),
(2 as usize, 6 as usize, 's', "scssswssss"),
(1 as usize, 2 as usize, 'n', "njdnnnn"),
(4 as usize, 8 as usize, 'h', "hhhzhhhhhhhhhhhhhhh"),
(9 as usize, 10 as usize, 'j', "jjjjjjjjkjjxqjjw"),
(4 as usize, 9 as usize, 't', "tzdqtlttttktttttcttt"),
(4 as usize, 5 as usize, 'w', "wdwht"),
(8 as usize, 9 as usize, 'x', "xfxxxxfrsxp"),
(3 as usize, 4 as usize, 'm', "mmmmgwwbmztpmbmmmtls"),
(8 as usize, 17 as usize, 'f', "mgzhfgfffswbgnvbc"),
(5 as usize, 6 as usize, 't', "tttttg"),
(4 as usize, 18 as usize, 'x', "xxxwxxxxxxxbxxxnxxx"),
(2 as usize, 4 as usize, 'l', "cqglmhmtjls"),
(2 as usize, 4 as usize, 'z', "zfgzr"),
(11 as usize, 18 as usize, 'k', "nrdngbjkpckjxwdbrh"),
(8 as usize, 9 as usize, 'f', "fffffffpf"),
(4 as usize, 7 as usize, 'g', "ggtgghgsnggr"),
(5 as usize, 6 as usize, 'w', "wswwlw"),
(3 as usize, 5 as usize, 'b', "dfbbjbccx"),
(2 as usize, 3 as usize, 't', "tdwfzg"),
(4 as usize, 5 as usize, 'r', "rhrbw"),
(4 as usize, 5 as usize, 'j', "jjjjtjjn"),
(3 as usize, 4 as usize, 'k', "hkzkk"),
(1 as usize, 2 as usize, 'c', "cdzccc"),
(3 as usize, 5 as usize, 'r', "vhlrrvhr"),
(4 as usize, 9 as usize, 'l', "qbjdqldwzdl"),
(6 as usize, 11 as usize, 'f', "flvpvfcfrgg"),
(17 as usize, 18 as usize, 'g', "xgggggghgggglggggz"),
(5 as usize, 6 as usize, 'v', "bhvgvl"),
(7 as usize, 15 as usize, 'n', "nnnnnnnnnnnnnngnnnnn"),
(6 as usize, 15 as usize, 'd', "dtddsddfddmcpdf"),
(10 as usize, 12 as usize, 'b', "bbbbbbbbvlbbbbbbb"),
(2 as usize, 12 as usize, 'd', "szdghlzwxpnd"),
(3 as usize, 4 as usize, 'z', "zzzw"),
(6 as usize, 15 as usize, 't', "mrnjvfhtlqwlfzt"),
(8 as usize, 10 as usize, 'x', "lcxcbrxxjw"),
(6 as usize, 13 as usize, 'r', "rgszrlzmmlpdngchhxz"),
(1 as usize, 14 as usize, 'g', "fgggggggggggggggggg"),
(3 as usize, 5 as usize, 'n', "znnnjpksqtzt"),
(17 as usize, 18 as usize, 'l', "bcfmqlsltppxwsxslb"),
(5 as usize, 8 as usize, 't', "tttnctttt"),
(4 as usize, 5 as usize, 'm', "mmmmnmml"),
(15 as usize, 20 as usize, 'k', "kmkpvxkgnckknzkpkqkt"),
(12 as usize, 15 as usize, 'x', "xxxxvxlxxkdxxxx"),
(1 as usize, 5 as usize, 'v', "vjdndsvsjvqzvnv"),
(1 as usize, 4 as usize, 'k', "kkkkkkk"),
(5 as usize, 8 as usize, 'c', "cxccccccc"),
(2 as usize, 4 as usize, 'v', "vdvd"),
(12 as usize, 14 as usize, 'w', "wwwwwwwwwwqwwbww"),
(2 as usize, 3 as usize, 'b', "kbbtbrllwp"),
(14 as usize, 15 as usize, 'f', "ffffffffffffffm"),
(9 as usize, 10 as usize, 'v', "nlngldlnvsbwcvvt"),
(4 as usize, 5 as usize, 'c', "kccxccc"),
(5 as usize, 9 as usize, 'm', "bmmmmmmmzmf"),
(8 as usize, 9 as usize, 'f', "fzfvffjffffv"),
(3 as usize, 14 as usize, 'r', "rrqrrrrrrrrrrjrh"),
(13 as usize, 14 as usize, 'p', "ppptpppppptpqdpm"),
(6 as usize, 12 as usize, 't', "tttjtvtgcwvttttqkt"),
(5 as usize, 10 as usize, 'z', "glsrzctzzz"),
(5 as usize, 8 as usize, 's', "gckwcshsl"),
(17 as usize, 19 as usize, 'n', "nnnnhnnnnnnnntwnnnd"),
(7 as usize, 9 as usize, 'r', "rrrrfdrrxrrrrrrrrrrq"),
(17 as usize, 20 as usize, 'm', "rmxbmmvwphmxmzlmbmxm"),
(14 as usize, 15 as usize, 'l', "kkjwtlsrlhltmdl"),
(14 as usize, 17 as usize, 'f', "ffffffffffffffffff"),
(3 as usize, 13 as usize, 'l', "khxtqtwbvpmgll"),
(2 as usize, 4 as usize, 'w', "wwbmww"),
(9 as usize, 15 as usize, 'n', "nwxcnxnckttrkdqnn"),
(3 as usize, 4 as usize, 't', "fgnwjbtlntsr"),
(15 as usize, 16 as usize, 'x', "mcxxxxxxrxxxpczx"),
(6 as usize, 16 as usize, 'w', "vtcvkmrwvlmwdvrwmqj"),
(1 as usize, 3 as usize, 'c', "mmcjckwn"),
(1 as usize, 10 as usize, 'c', "ccccccccckccccc"),
(14 as usize, 16 as usize, 'l', "kqjhpjgzvxlnxxll"),
(4 as usize, 7 as usize, 'r', "xrtrrrrrcrrmrrrr"),
(8 as usize, 13 as usize, 'm', "mmmmmmmmmmmmcmm"),
(7 as usize, 8 as usize, 'r', "trkrrrrwf"),
(3 as usize, 4 as usize, 'n', "pnjn"),
(1 as usize, 5 as usize, 'k', "skkknkk"),
(11 as usize, 16 as usize, 'k', "kfkkkjkqpqgkzkkkkwsn"),
(13 as usize, 17 as usize, 'f', "gdllffxlxwncljgwf"),
(3 as usize, 5 as usize, 's', "gwspdtjtnlbsfffvhlg"),
(15 as usize, 17 as usize, 'm', "krmfcsqbmmmjwgkdmm"),
(13 as usize, 14 as usize, 'l', "knhdrdzcmdhlll"),
(2 as usize, 3 as usize, 'p', "frps"),
(2 as usize, 9 as usize, 'z', "lzwnzmvnqgkpbxv"),
(5 as usize, 9 as usize, 'n', "nnngrnnbj"),
(3 as usize, 5 as usize, 'c', "mncnbk"),
(2 as usize, 5 as usize, 'n', "djgnnnnnzbnnnx"),
(7 as usize, 8 as usize, 'v', "vvgvgvvm"),
(5 as usize, 15 as usize, 'w', "wwwwwwwwwwwwwwwwwww"),
(6 as usize, 7 as usize, 'd', "dddddcz"),
(7 as usize, 9 as usize, 'g', "glrgcggvgckrgggz"),
(2 as usize, 3 as usize, 'n', "dnwbnc"),
(6 as usize, 8 as usize, 't', "lttztzqt"),
(1 as usize, 4 as usize, 'm', "mmmxm"),
(4 as usize, 14 as usize, 'l', "qqhgtftklcnmllcbgbrx"),
(2 as usize, 3 as usize, 'd', "sdnk"),
(12 as usize, 15 as usize, 'l', "lqllljfglvldcql"),
(2 as usize, 10 as usize, 'k', "kgkkkkkkkkxkkkkkkkkk"),
(10 as usize, 17 as usize, 'h', "ljpwchmhhzmhdhmrchp"),
(6 as usize, 10 as usize, 'w', "lpcfgkslrwwrlkhx"),
(3 as usize, 7 as usize, 'w', "wrpwcpw"),
(8 as usize, 9 as usize, 'z', "kczxltgzh"),
(6 as usize, 11 as usize, 'n', "nnnnnhnsnlnnn"),
(2 as usize, 9 as usize, 's', "smssssssgss"),
(2 as usize, 4 as usize, 'x', "xxwhxbfjj"),
(1 as usize, 2 as usize, 'z', "fzzzzzzzzzzzzzz"),
(4 as usize, 5 as usize, 'p', "pplcdpp"),
(3 as usize, 4 as usize, 'c', "gncxlzc"),
(16 as usize, 17 as usize, 'x', "fxqltszfgnnkxgrxhcbk"),
(13 as usize, 17 as usize, 'n', "nnnnnnnnnnnnnnnnvn"),
(1 as usize, 6 as usize, 'x', "dxxxxxx"),
(7 as usize, 8 as usize, 'r', "scbnvqrpcbgmpmrrs"),
(2 as usize, 17 as usize, 'd', "ddddddddddddddddhd"),
(13 as usize, 19 as usize, 'v', "fvtphwfnmpfpbpjnnbv"),
(7 as usize, 18 as usize, 'q', "cpwqnhqjqfkqqncbsh"),
(6 as usize, 10 as usize, 'c', "cccdcxccncfxcgc"),
(2 as usize, 4 as usize, 'g', "fggsgbgggggcggt"),
(13 as usize, 17 as usize, 'r', "hspwrxrzbrvlmlwgrkxr"),
(14 as usize, 15 as usize, 'l', "pllgllllllllrmv"),
(12 as usize, 15 as usize, 'g', "hqgcgggsxgjxljgdz"),
(3 as usize, 4 as usize, 'd', "dtxd"),
(7 as usize, 12 as usize, 'd', "kddvbkkdldqbkn"),
(3 as usize, 13 as usize, 'v', "vvvvvvvvvvvvtv"),
(8 as usize, 13 as usize, 't', "tttttttttfttdt"),
(18 as usize, 19 as usize, 'q', "hprbdznbqlfnwzwpqckb"),
(5 as usize, 12 as usize, 'c', "wwlqcgzqzvtczvcldg"),
(3 as usize, 5 as usize, 'z', "xzzzv"),
(2 as usize, 11 as usize, 'c', "xbblzgtwcjcfqqb"),
(8 as usize, 9 as usize, 'n', "nnnvbnmvl"),
(8 as usize, 9 as usize, 'z', "zzzszzzzt"),
(2 as usize, 3 as usize, 'l', "chsrlrl"),
(2 as usize, 4 as usize, 'f', "nffm"),
(6 as usize, 7 as usize, 'h', "hhhhhhhh"),
(10 as usize, 16 as usize, 'x', "xxxxxxxxxxxxxxxwxxxx"),
(2 as usize, 19 as usize, 'v', "ztpvktjgjlmqfrrxfpv"),
(2 as usize, 5 as usize, 'g', "gncgg"),
(1 as usize, 3 as usize, 't', "hjtttttvgtttttttttt"),
(3 as usize, 4 as usize, 's', "nbvs"),
(5 as usize, 10 as usize, 'n', "nnnqnnnnbvnnn"),
(7 as usize, 15 as usize, 'q', "qqqqpqqqqqqqqqzqqsqq"),
(3 as usize, 5 as usize, 'b', "sjtwbr"),
(2 as usize, 4 as usize, 't', "sttxln"),
(1 as usize, 5 as usize, 'd', "ddddd"),
(12 as usize, 13 as usize, 'v', "zvdpfbkkvcpvdvb"),
(3 as usize, 6 as usize, 'j', "cnnjhj"),
(7 as usize, 8 as usize, 'q', "qqrqpbfqjvbtqlqjqkqh"),
(2 as usize, 4 as usize, 'v', "wvvq"),
(2 as usize, 7 as usize, 'm', "mpmrmmmmdnmmmmk"),
(10 as usize, 14 as usize, 'g', "ggmcgggpggcngglm"),
(3 as usize, 5 as usize, 'g', "fsbpglh"),
(4 as usize, 5 as usize, 'r', "rdrtq"),
(3 as usize, 4 as usize, 't', "qttltttl"),
(16 as usize, 18 as usize, 's', "sssszpssbnsssssfss"),
(6 as usize, 9 as usize, 'b', "lbxbwbbqn"),
(2 as usize, 3 as usize, 'm', "dmwsg"),
(4 as usize, 12 as usize, 'p', "lmppwmsplppx"),
(3 as usize, 15 as usize, 'c', "lvjmlzwctxnckvclsj"),
(13 as usize, 14 as usize, 't', "tttttttftstttw"),
(1 as usize, 5 as usize, 'm', "jmmmm"),
(2 as usize, 3 as usize, 'r', "rsfr"),
(1 as usize, 4 as usize, 'd', "xdns"),
(2 as usize, 3 as usize, 'k', "qklrwnskqnx"),
(1 as usize, 2 as usize, 'r', "rrrr"),
(5 as usize, 8 as usize, 'l', "vlsbftlltc"),
(3 as usize, 12 as usize, 'n', "nhjlchbwphmn"),
(6 as usize, 7 as usize, 'h', "thhghhv"),
(1 as usize, 11 as usize, 'v', "vvvvvvsvvvk"),
(9 as usize, 11 as usize, 'c', "ckdqzdkbjczkkcpdj"),
(7 as usize, 12 as usize, 'b', "bbbbbjbbbbbfzbbb"),
(3 as usize, 6 as usize, 'v', "vvwxkv"),
(6 as usize, 8 as usize, 't', "twttttttt"),
(12 as usize, 17 as usize, 'g', "gfggggggggggggggg"),
(2 as usize, 3 as usize, 'g', "gqgggggggggggggg"),
(8 as usize, 9 as usize, 'h', "fmjhhbjhvv"),
(4 as usize, 7 as usize, 'q', "qqqqqqjsq"),
(4 as usize, 5 as usize, 'p', "hpkjp"),
(2 as usize, 10 as usize, 'h', "bhsgwpwnhh"),
(15 as usize, 18 as usize, 'p', "nwpqxrcxgjxbbxczxb"),
(2 as usize, 3 as usize, 'k', "mtkszk"),
(9 as usize, 11 as usize, 'c', "zccccpccrrc"),
(5 as usize, 6 as usize, 'c', "qnzjgh"),
(7 as usize, 11 as usize, 't', "ttttttmtttct"),
(1 as usize, 5 as usize, 'p', "pppppprplmpq"),
(3 as usize, 4 as usize, 'x', "sxlc"),
(12 as usize, 14 as usize, 'q', "xsqzxsrrmxvdxq"),
(1 as usize, 3 as usize, 'k', "kklkjkvkkkkkk"),
(11 as usize, 12 as usize, 'k', "ffflkkkkkkqkkks"),
(2 as usize, 3 as usize, 'z', "zlzzz"),
(10 as usize, 13 as usize, 'k', "kkxkkbkkfkckn"),
(11 as usize, 15 as usize, 'p', "wkppvppxqxpnpbpkpppp"),
(2 as usize, 11 as usize, 'r', "krqxlrvhwhlj"),
(3 as usize, 4 as usize, 'l', "llllllrrbll"),
(12 as usize, 14 as usize, 'n', "nthpvpzmwnsnnn"),
(15 as usize, 18 as usize, 'w', "jwsnzwwwwwvwfdwggcw"),
(15 as usize, 16 as usize, 'k', "gtxkxjvtkktkkhkr"),
(1 as usize, 3 as usize, 'm', "kmzmmm"),
(9 as usize, 10 as usize, 'j', "jjjjjjjjvwj"),
(5 as usize, 8 as usize, 'p', "sppkrxzpbppppphpwv"),
(5 as usize, 7 as usize, 'w', "wwwwgwhwwhppmqw"),
(5 as usize, 6 as usize, 'h', "hchhhplrhphqq"),
(4 as usize, 5 as usize, 'g', "bggbg"),
(3 as usize, 4 as usize, 'h', "sbhmtvhhrbd"),
(1 as usize, 4 as usize, 'l', "lqfl"),
(5 as usize, 7 as usize, 'j', "jjjpjljjjj"),
(3 as usize, 5 as usize, 'q', "qqqqdqqqjqqqqqqqqqqq"),
(1 as usize, 13 as usize, 'k', "kkkkkkkkxkkknkk"),
(12 as usize, 14 as usize, 'z', "jzzzzzzzzzzzzvz"),
(1 as usize, 4 as usize, 'q', "bqqq"),
(8 as usize, 9 as usize, 'w', "wkwftfmfx"),
(7 as usize, 9 as usize, 's', "kssjlslpmqssx"),
(1 as usize, 2 as usize, 'n', "dxzmtsvnfhjnqsfln"),
(15 as usize, 17 as usize, 'q', "bqmqnrcjsmgghgqjr"),
(8 as usize, 11 as usize, 'z', "zzzzzzzdzzfz"),
(6 as usize, 7 as usize, 'z', "znznzzz"),
(8 as usize, 11 as usize, 'l', "jvlntmjwwrrqlkzrhg"),
(1 as usize, 5 as usize, 'r', "rrvrjtrrjzr"),
(4 as usize, 20 as usize, 'd', "fbvprndxpfqplmtkntdd"),
(7 as usize, 9 as usize, 'l', "llllllqlclllll"),
(3 as usize, 6 as usize, 'n', "xrnjzmlbnjwwzdzmdj"),
(17 as usize, 19 as usize, 'd', "ddddddddddddddddxddd"),
(9 as usize, 10 as usize, 'w', "wwwwwckbwhww"),
(2 as usize, 5 as usize, 'h', "gchshhhn"),
(1 as usize, 4 as usize, 'l', "gtlq"),
(15 as usize, 16 as usize, 'z', "zzzzlzzzzzzzzzzhzdzz"),
(5 as usize, 6 as usize, 'l', "lllfllld"),
(14 as usize, 16 as usize, 'j', "jjjjgjjjjjjjjjjjc"),
(6 as usize, 8 as usize, 'd', "dddddrddd"),
(4 as usize, 5 as usize, 'h', "zhshc"),
(8 as usize, 9 as usize, 'g', "gmgxgbfqg"),
(1 as usize, 8 as usize, 'r', "lrrrrrrzrrgrrrrr"),
(4 as usize, 13 as usize, 'c', "mccqccdccccwccccccc"),
(3 as usize, 4 as usize, 'z', "zhzz"),
(10 as usize, 11 as usize, 'c', "crmmvznptct"),
(2 as usize, 4 as usize, 'l', "slblllt"),
(1 as usize, 6 as usize, 'q', "wqqdqqtqqqgdqqq"),
(2 as usize, 13 as usize, 'l', "nlllpwllpjdbxvbp"),
(6 as usize, 8 as usize, 'l', "mxsflqrlhkqhsrmhtwxq"),
(4 as usize, 9 as usize, 't', "tpwbtdttt"),
(2 as usize, 7 as usize, 'q', "fzqdrbg"),
(7 as usize, 8 as usize, 'd', "ddpldttdddsd"),
(14 as usize, 17 as usize, 'b', "bbbbbbbbbbbbbmbbbb"),
(4 as usize, 11 as usize, 'x', "wfrxkjtpxlcbgc"),
(6 as usize, 7 as usize, 'n', "nnnnnjn"),
(13 as usize, 16 as usize, 'z', "zmqczdggpqzpcrlz"),
(1 as usize, 8 as usize, 'j', "jjjjjjzdmjjtjj"),
(5 as usize, 6 as usize, 'v', "vjsnvmb"),
(5 as usize, 7 as usize, 'q', "nzqqwbqmbjwllj"),
(2 as usize, 3 as usize, 'j', "mtjg"),
(12 as usize, 15 as usize, 'd', "ddxdddddddddddcddd"),
(4 as usize, 15 as usize, 'g', "hssvxrqgngtkcmh"),
(1 as usize, 4 as usize, 'm', "mmmmmmm"),
(11 as usize, 13 as usize, 'j', "jqjjjjjjjjmjj"),
(3 as usize, 4 as usize, 'z', "zznzz"),
(2 as usize, 6 as usize, 'c', "cccmcs"),
(6 as usize, 10 as usize, 'x', "xxxxxgxxlxxpxxxx"),
(1 as usize, 2 as usize, 'b', "bbrbbbbb"),
(2 as usize, 5 as usize, 'f', "xfmkcf"),
(4 as usize, 5 as usize, 'r', "rrrkxr"),
(3 as usize, 4 as usize, 'z', "zslz"),
(3 as usize, 4 as usize, 'w', "kwwh"),
(15 as usize, 17 as usize, 'x', "rfxxcxwxsxsdgnxlxz"),
(17 as usize, 18 as usize, 'w', "rwqlwwgwwwwjwbcjtw"),
(2 as usize, 4 as usize, 'p', "ppjrpp"),
(16 as usize, 17 as usize, 'b', "bbbbbbbbbbbbbbbtb"),
(5 as usize, 6 as usize, 'b', "fbwbqt"),
(3 as usize, 5 as usize, 'b', "bbjvxg"),
(4 as usize, 5 as usize, 'j', "jbhljfjz"),
(4 as usize, 5 as usize, 'k', "fmkkckpj"),
(18 as usize, 19 as usize, 'w', "wpqtwhngztqkvgqrcjf"),
(5 as usize, 6 as usize, 't', "wttthhtt"),
(12 as usize, 15 as usize, 'v', "kvgvvvcfglsvnsp"),
(12 as usize, 14 as usize, 'n', "nnnnnnnnnnnknnn"),
(5 as usize, 8 as usize, 'k', "xxzhdkmmkkkbwv"),
(8 as usize, 9 as usize, 'f', "fdffdgvwpfffff"),
(12 as usize, 14 as usize, 'k', "kdbsqwkjhvbxrkh"),
(4 as usize, 7 as usize, 'f', "fvhkstfdrwfkvv"),
(7 as usize, 17 as usize, 'x', "cvkbcvbfxxgxhbxxxpbx"),
(11 as usize, 14 as usize, 'm', "jjnmmmsvhzcmcm"),
(3 as usize, 9 as usize, 'w', "qwxsnsxnwzsnmk"),
(1 as usize, 5 as usize, 'k', "tkkkkkkkkkkkk"),
(5 as usize, 7 as usize, 'h', "hhhhhhdh"),
(3 as usize, 13 as usize, 'c', "cclccccccccwccccc"),
(1 as usize, 4 as usize, 'w', "wwwnw"),
(3 as usize, 7 as usize, 'z', "wzzblltdglmfkl"),
(9 as usize, 12 as usize, 'k', "kkkwqjnqskkdhckhvkk"),
(2 as usize, 5 as usize, 'r', "xjtrrsxrrdzlbjvflqxr"),
(9 as usize, 13 as usize, 'g', "gggbzggggjgxkgg"),
(1 as usize, 8 as usize, 'm', "zmmmmmmhmmmmmhmmmmm"),
(16 as usize, 18 as usize, 'h', "hzhhhhhhhhhhhhhhhh"),
(2 as usize, 7 as usize, 'w', "wwwwwwvw"),
(3 as usize, 4 as usize, 'd', "ddhd"),
(3 as usize, 5 as usize, 'x', "jxvzx"),
(15 as usize, 18 as usize, 'k', "kkkkhkckkkkkkkkkkkxk"),
(11 as usize, 12 as usize, 'm', "mmmmmmmmsmwkm"),
(7 as usize, 8 as usize, 'k', "khfkkktj"),
(2 as usize, 7 as usize, 'f', "ffffffff"),
(2 as usize, 6 as usize, 'q', "hqqdhbfvc"),
(3 as usize, 5 as usize, 'f', "rlpffgf"),
(3 as usize, 4 as usize, 't', "wtltht"),
(4 as usize, 5 as usize, 'f', "fscfx"),
(2 as usize, 16 as usize, 't', "nmtppmqttqztvdstc"),
(1 as usize, 15 as usize, 'j', "jwgcbkdjlmjjxzwvpvd"),
(10 as usize, 12 as usize, 'v', "vvhvfvvqvvvv"),
(5 as usize, 6 as usize, 'l', "llllbwlll"),
(1 as usize, 2 as usize, 'z', "xmszvzrwpm"),
(6 as usize, 11 as usize, 'd', "dddjndddddq"),
(4 as usize, 9 as usize, 'r', "xwkfwcztcq"),
(9 as usize, 10 as usize, 'k', "ckskkkktkr"),
(2 as usize, 4 as usize, 'x', "txpxfq"),
(1 as usize, 3 as usize, 'j', "sjzj"),
(7 as usize, 11 as usize, 'x', "bbhcswxtnhx"),
(9 as usize, 10 as usize, 'q', "jlqnqmhjqhqq"),
(4 as usize, 19 as usize, 'd', "qddkdmptbvjpbrjdzddl"),
(7 as usize, 9 as usize, 'd', "sqdpdhhdx"),
(7 as usize, 8 as usize, 'j', "gjzmzjgd"),
(10 as usize, 15 as usize, 's', "gkgsssssssqssssrpc"),
(5 as usize, 6 as usize, 'v', "vvvvhbvh"),
(1 as usize, 3 as usize, 'c', "cccc"),
(1 as usize, 3 as usize, 'c', "ccwcccczgccpccz"),
(2 as usize, 4 as usize, 't', "tgtmqtl"),
(11 as usize, 13 as usize, 'w', "wwwcwwwwwwlhw"),
(4 as usize, 5 as usize, 'z', "nzgzrz"),
(4 as usize, 11 as usize, 's', "lhzxmwclxss"),
(15 as usize, 18 as usize, 's', "hmszwkscbdzsrgssjj"),
(4 as usize, 5 as usize, 'm', "wkvgzjmhxmwlmlmvsjv"),
(11 as usize, 12 as usize, 't', "lndqtmsfwpjp"),
(2 as usize, 10 as usize, 'w', "wkwwwwwwwww"),
(10 as usize, 11 as usize, 't', "ttgpwkjltgn"),
(3 as usize, 9 as usize, 'b', "bbvbbbbbtb"),
(5 as usize, 7 as usize, 'h', "rqlbntrhhkjhhhrdhq"),
(1 as usize, 2 as usize, 'n', "rnnrbnn"),
(8 as usize, 11 as usize, 'n', "nnnnnnnpnnnnnn"),
(4 as usize, 5 as usize, 's', "vhsnsjc"),
(5 as usize, 7 as usize, 'b', "tbbbbbcbb"),
(1 as usize, 3 as usize, 'q', "frbq"),
(3 as usize, 4 as usize, 's', "xsssmfsgs"),
(13 as usize, 17 as usize, 'k', "kkkkkbfkkkkkvkkkkkkk"),
(1 as usize, 13 as usize, 'v', "zvvvvvvvvvvvvv"),
(11 as usize, 14 as usize, 'c', "cbcmcccccccmccc"),
(15 as usize, 17 as usize, 'r', "skkrrvsrlmrrrrrjdrrr"),
(1 as usize, 7 as usize, 'm', "jmmqmmmmkmmmrkmmr"),
(9 as usize, 14 as usize, 'f', "kstfsxflhffxsffkb"),
(7 as usize, 9 as usize, 'g', "ggggggggvggggg"),
(13 as usize, 16 as usize, 't', "tttttttgtttttttvtt"),
(9 as usize, 10 as usize, 'p', "ppppppppphp"),
(3 as usize, 4 as usize, 'w', "wwxw"),
(9 as usize, 13 as usize, 'g', "ggggggggrgggvg"),
(3 as usize, 4 as usize, 'f', "ffkffq"),
(8 as usize, 11 as usize, 'h', "hbhhzhhhhhfh"),
(2 as usize, 4 as usize, 'd', "dcnss"),
(6 as usize, 7 as usize, 'r', "rtrrrbr"),
(5 as usize, 6 as usize, 'r', "rrrrxq"),
(1 as usize, 11 as usize, 'g', "fgggggggmkglk"),
(14 as usize, 15 as usize, 'h', "vlqkqhhhfwhxfvs"),
(3 as usize, 4 as usize, 'w', "wlrsgfsw"),
(1 as usize, 2 as usize, 'v', "dxkwzvvxv"),
(2 as usize, 4 as usize, 'r', "rvrcrtrrl"),
(4 as usize, 6 as usize, 't', "ttktttt"),
(10 as usize, 15 as usize, 'j', "jjjjbtjjtjnjjjk"),
(5 as usize, 6 as usize, 's', "ssssssss"),
(5 as usize, 7 as usize, 's', "sfnkzss"),
(4 as usize, 5 as usize, 'b', "shbtb"),
(2 as usize, 5 as usize, 'j', "hjktjm"),
(1 as usize, 5 as usize, 'h', "hhhhdhhhhh"),
(5 as usize, 17 as usize, 'm', "mmmmgmmmmmmmmmmmmrmn"),
(2 as usize, 6 as usize, 'b', "cxgxbbskzgdhr"),
(10 as usize, 12 as usize, 'k', "kkkkkkkkkbkkkkknkmks"),
(13 as usize, 16 as usize, 'g', "ggggggqggggghggggggg"),
(1 as usize, 2 as usize, 'w', "wwwl"),
(6 as usize, 9 as usize, 'b', "bkbbmbbbzb"),
(6 as usize, 7 as usize, 'm', "qrfhmmndrkmc"),
(5 as usize, 11 as usize, 'p', "ggzmjkxpnrpf"),
(2 as usize, 3 as usize, 'r', "rhrr"),
(6 as usize, 7 as usize, 'f', "vppvpwf"),
(8 as usize, 10 as usize, 'w', "wrwwwdvwwjwwww"),
(6 as usize, 11 as usize, 'c', "wxrbztwpcccj"),
(14 as usize, 17 as usize, 'x', "xxxxxxxxxxxxxrxxxxxx"),
(5 as usize, 8 as usize, 'c', "cccccczqccc"),
(2 as usize, 6 as usize, 'j', "jgqjjfjzjjjjjjmjjj"),
(4 as usize, 7 as usize, 't', "zphkzttgtjdxdtd"),
(4 as usize, 7 as usize, 't', "wsrtdqgthqjvznbj"),
(15 as usize, 19 as usize, 'h', "hmhhhhzhhhchhmhhhtxh"),
(1 as usize, 3 as usize, 'z', "zzzz"),
(2 as usize, 3 as usize, 'j', "jcvl"),
(1 as usize, 7 as usize, 'w', "wcpwswwgjfb"),
(3 as usize, 6 as usize, 'c', "crsvmcckc"),
(9 as usize, 10 as usize, 'f', "fffffffffjff"),
(3 as usize, 6 as usize, 'v', "hfvpwvgg"),
(2 as usize, 5 as usize, 'r', "dkhrrd"),
(1 as usize, 5 as usize, 'f', "cflmflfdvbz"),
(3 as usize, 13 as usize, 'k', "sfkgcgktfkhrh"),
(3 as usize, 9 as usize, 'v', "mmrprsvzv"),
(3 as usize, 4 as usize, 'q', "qqqbcrkq"),
(11 as usize, 13 as usize, 'r', "rrrwrrrrrrrgrr"),
(6 as usize, 11 as usize, 'j', "tjjjzpsjrjdj"),
(14 as usize, 18 as usize, 't', "dtbhmtltcwpnzwqtgt"),
(2 as usize, 5 as usize, 'c', "rsccchcc"),
(11 as usize, 14 as usize, 'm', "kmmmmmlvmmtmmm"),
(7 as usize, 10 as usize, 'x', "xhxxxxxbxbhxxxx"),
(10 as usize, 13 as usize, 'n', "nnntnnnnnpnnn"),
(3 as usize, 10 as usize, 'w', "wwwjwgwwwgwmww"),
(17 as usize, 18 as usize, 'p', "phpppnpqppjsrpppzj"),
(8 as usize, 12 as usize, 'r', "rsrbwrrrrrrzr"),
(9 as usize, 15 as usize, 'q', "bqlrdqqxrdqqnxq"),
(5 as usize, 11 as usize, 'd', "sldcndtlpzdb"),
(1 as usize, 3 as usize, 'w', "zwww"),
(11 as usize, 12 as usize, 'k', "tkbkwkkvsblpt"),
(13 as usize, 14 as usize, 'c', "ccccccccccccqc"),
(1 as usize, 5 as usize, 'c', "ccccrc"),
(4 as usize, 5 as usize, 'f', "fffnf"),
(3 as usize, 4 as usize, 'w', "wwwvw"),
(2 as usize, 4 as usize, 'k', "kzkk"),
(16 as usize, 18 as usize, 'j', "jjjjjjjjjjjjjjvqjj"),
(2 as usize, 8 as usize, 'v', "wvqlrnrtgbzrp"),
(6 as usize, 10 as usize, 'c', "cccccdcccccc"),
(1 as usize, 4 as usize, 'q', "bqqqq"),
(5 as usize, 6 as usize, 'n', "nnnnnnn"),
(2 as usize, 16 as usize, 'f', "cjrffhfpfflxljjfp"),
(3 as usize, 8 as usize, 'g', "ggfggggggg"),
(7 as usize, 8 as usize, 'z', "zmzkzzzczwzzzz"),
(7 as usize, 8 as usize, 'm', "mmmmmmmmmm"),
(7 as usize, 9 as usize, 'f', "vzlffftfw"),
(4 as usize, 10 as usize, 'w', "kckwgbmtws"),
(4 as usize, 5 as usize, 'g', "ggghgp"),
(6 as usize, 17 as usize, 'w', "wwwwwwwwwwwwwwwwkw"),
(3 as usize, 16 as usize, 'f', "fffbfffffffffffcff"),
(9 as usize, 14 as usize, 'l', "lllllllwmllmblllhlml"),
(1 as usize, 4 as usize, 's', "sssdssss"),
(3 as usize, 4 as usize, 'm', "lmnm"),
(10 as usize, 11 as usize, 'v', "vvvvkvsvvvmvhv"),
(3 as usize, 4 as usize, 'p', "pprb"),
(3 as usize, 4 as usize, 'k', "pkqk"),
(3 as usize, 4 as usize, 'd', "ddxd"),
(7 as usize, 8 as usize, 'b', "bbbbbbfb"),
(5 as usize, 7 as usize, 'w', "qbmhsmt"),
(11 as usize, 12 as usize, 'b', "bbbbbbbbbbbgb"),
(3 as usize, 5 as usize, 'x', "xpxbljxt"),
(2 as usize, 9 as usize, 'z', "kzmpqtbvzrqzh"),
(3 as usize, 16 as usize, 'v', "qwvfvltjrpdxmvqv"),
(2 as usize, 6 as usize, 'n', "pdjxzkn"),
(7 as usize, 8 as usize, 'j', "jmzvjkjk"),
(2 as usize, 5 as usize, 'r', "rrfjqqft"),
(2 as usize, 5 as usize, 'h', "pwhfh"),
(6 as usize, 7 as usize, 'm', "mmgvjmm"),
(11 as usize, 12 as usize, 'r', "rrrrrrrrrrxqrr"),
(1 as usize, 4 as usize, 'n', "nnnw"),
(1 as usize, 5 as usize, 'z', "szzzzzdtzz"),
(7 as usize, 13 as usize, 'j', "jjjjjjnjjjjjbj"),
(10 as usize, 15 as usize, 'w', "rwwwwtmwswwwwwwwnmbk"),
(11 as usize, 13 as usize, 't', "twxhrldqtttmnt"),
(1 as usize, 2 as usize, 'r', "bkbbrwr"),
(11 as usize, 17 as usize, 'h', "hhdhhhhhhshqpbhhn"),
(4 as usize, 7 as usize, 'c', "crgchccbnr"),
(9 as usize, 11 as usize, 'r', "bdhgrzkmrrl"),
(6 as usize, 8 as usize, 'g', "gggggggzz"),
(3 as usize, 9 as usize, 'g', "ggggggggqg"),
(9 as usize, 11 as usize, 'z', "zrfcqtrxxqzcx"),
(3 as usize, 9 as usize, 's', "zstjqhnvgjjfxknt"),
(12 as usize, 13 as usize, 'p', "pppppwpgcppjppppptp"),
(6 as usize, 7 as usize, 'k', "kkwrkckb"),
(8 as usize, 9 as usize, 'k', "kkkkqzjkn"),
(8 as usize, 9 as usize, 'l', "lrxlkbflrl"),
(1 as usize, 3 as usize, 'n', "nndn"),
(8 as usize, 9 as usize, 'd', "ddhddddddd"),
(4 as usize, 12 as usize, 'g', "zdclfqvdgnzfv"),
(3 as usize, 5 as usize, 'd', "ddddkddddddd"),
(9 as usize, 11 as usize, 'x', "xxxxxxxxqxxx"),
(4 as usize, 7 as usize, 't', "ttttfftt"),
(2 as usize, 4 as usize, 'n', "wfmnnddqxfm"),
(16 as usize, 19 as usize, 'r', "zhjsgxjkjpqmpvkrjgr"),
(3 as usize, 7 as usize, 'v', "vvfvvvvv"),
(1 as usize, 2 as usize, 'd', "qdwdfj"),
(6 as usize, 10 as usize, 'h', "hhhhhhhhhrhh"),
(4 as usize, 16 as usize, 'x', "xxxpxxxxxxxxxxxxx"),
(18 as usize, 19 as usize, 'q', "qqqqqqqqqqqqqlqqqqf"),
(6 as usize, 10 as usize, 'g', "gkcntgbgbggklsx"),
(8 as usize, 9 as usize, 'n', "nnnnxnnnpnn"),
(7 as usize, 9 as usize, 'm', "msmmmtdvm"),
(2 as usize, 15 as usize, 'd', "twjdrfzntqhnwkd"),
(1 as usize, 4 as usize, 'z', "kzzz"),
(16 as usize, 18 as usize, 'b', "tbbbtbjbtbtflzckhb"),
(4 as usize, 12 as usize, 'k', "kkbhkgkrkgfk"),
(8 as usize, 10 as usize, 'q', "lrqrjqvwmrb"),
(1 as usize, 3 as usize, 'f', "vfhf"),
(7 as usize, 14 as usize, 'v', "vvvvvvrtvvvvvvvv"),
(4 as usize, 5 as usize, 'n', "xnntnwntrfnbqqdk"),
(3 as usize, 5 as usize, 'r', "rhkrzwrhrrr"),
(2 as usize, 4 as usize, 'b', "bspbjb"),
(5 as usize, 6 as usize, 's', "sfscsc"),
(6 as usize, 7 as usize, 'x', "xxxxxhx"),
(8 as usize, 10 as usize, 'w', "wwwwbzlmqw"),
(7 as usize, 10 as usize, 'v', "fkvdvjbfvd"),
(2 as usize, 5 as usize, 'q', "qtqspqqq"),
(8 as usize, 9 as usize, 'k', "kmhkkhpsk"),
(5 as usize, 8 as usize, 'h', "xhdhjfph"),
(3 as usize, 6 as usize, 'b', "dlbkbb"),
(1 as usize, 3 as usize, 'w', "wwbswwww"),
(2 as usize, 4 as usize, 'x', "mxtx"),
(2 as usize, 4 as usize, 'l', "llrll"),
(3 as usize, 7 as usize, 'j', "kclqzgc"),
(2 as usize, 3 as usize, 'r', "rxrrrgrrrrr"),
(2 as usize, 4 as usize, 'q', "nzwxlmcqqqm"),
(15 as usize, 16 as usize, 'h', "hhhhvmhbhdtbblbh"),
(13 as usize, 19 as usize, 'l', "ltkftclmlllflzltlnb"),
(4 as usize, 5 as usize, 'p', "zmwtpjrltqdmfppz"),
(6 as usize, 10 as usize, 't', "tjdxqtsbzhvprspljmv"),
(14 as usize, 17 as usize, 'q', "qcqqqqqcqghqqqqqjq"),
(1 as usize, 5 as usize, 'j', "flxrjspwlrdqsnjcs"),
(14 as usize, 15 as usize, 'm', "mmmlmmmmmmmmmwm"),
(3 as usize, 5 as usize, 'd', "dddvkwksdcrktlpd"),
(8 as usize, 11 as usize, 'l', "llcllllxllml"),
(2 as usize, 4 as usize, 'v', "vvvbv"),
(1 as usize, 3 as usize, 'g', "llggz"),
(3 as usize, 5 as usize, 'q', "znqqmt"),
(15 as usize, 17 as usize, 'f', "ffffffffffffffjfff"),
(17 as usize, 18 as usize, 'q', "zwnkmcqdqlqgkwfmqc"),
(8 as usize, 11 as usize, 'f', "fffsrffbfffffvfxf"),
(1 as usize, 7 as usize, 'b', "bbbbbbbb"),
(3 as usize, 4 as usize, 'l', "llzh"),
(8 as usize, 9 as usize, 'n', "nhnnnnqknnbnncncnnl"),
(9 as usize, 11 as usize, 'v', "wvvvvvvbhjc"),
(15 as usize, 16 as usize, 'q', "qcjqvfdcsqwdrqqt"),
(9 as usize, 10 as usize, 'j', "jcckdzkzjjb"),
(1 as usize, 2 as usize, 's', "hssmsssms"),
(1 as usize, 3 as usize, 'w', "xwww"),
(2 as usize, 4 as usize, 'l', "lllll"),
(2 as usize, 4 as usize, 'q', "qnmq"),
(16 as usize, 18 as usize, 't', "tttttttgtftttttttt"),
(5 as usize, 6 as usize, 't', "kttttj"),
(16 as usize, 17 as usize, 't', "twlqttttttttttmct"),
(8 as usize, 15 as usize, 'x', "xxwpxsqkxgkxgxxbdgx"),
(17 as usize, 18 as usize, 'h', "hhhhhhhhhhblhhhhrq"),
(12 as usize, 17 as usize, 'm', "fmkmmmmqkmmdrbvthm"),
(2 as usize, 4 as usize, 'b', "fbcb"),
(1 as usize, 14 as usize, 't', "tttttttttttttqtt"),
(17 as usize, 18 as usize, 'v', "vvvvvvvvvvvvvvvvvnv"),
(7 as usize, 10 as usize, 'x', "vxxtxlxxlk"),
(3 as usize, 5 as usize, 'n', "nnmnqnnb"),
(2 as usize, 8 as usize, 's', "vssjqsssssb"),
(9 as usize, 11 as usize, 'l', "wlllllllllllll"),
(4 as usize, 14 as usize, 'r', "zrlcrxrrrzrrrrr"),
(3 as usize, 14 as usize, 'n', "wrnjpnkndsshqk"),
(12 as usize, 16 as usize, 'p', "ppppzpppppphppppp"),
(9 as usize, 12 as usize, 'r', "rrrrrcbrrfprrrrr"),
(2 as usize, 3 as usize, 'b', "bbrb"),
(14 as usize, 16 as usize, 'd', "tzdjdndddgsddlnddgd"),
(16 as usize, 18 as usize, 'c', "cccccccccccccccwcc"),
(5 as usize, 6 as usize, 'v', "rvvqvt"),
(11 as usize, 17 as usize, 's', "ssssssssssssssssps"),
(8 as usize, 9 as usize, 'v', "vpvxqvvdvnvhgnvvlvs"),
(7 as usize, 8 as usize, 'd', "ddddqlrt"),
(7 as usize, 13 as usize, 'd', "bfzrkddtdwqld"),
(4 as usize, 6 as usize, 'c', "cccccq"),
(6 as usize, 8 as usize, 'd', "hkdndlqq"),
(11 as usize, 13 as usize, 'l', "ngmllbdklvlmqlz"),
(8 as usize, 17 as usize, 'm', "mmmmfmmmmmmmmmmmlmm"),
(12 as usize, 15 as usize, 'b', "bbsbbcblbsnbzbbfcfzz"),
(12 as usize, 13 as usize, 'k', "gbwkkkkkkkksk"),
(12 as usize, 14 as usize, 'x', "xxxxwxxxxxxdxxxxxxx"),
(3 as usize, 4 as usize, 'm', "mwsmp"),
(5 as usize, 6 as usize, 'k', "kkkkzk"),
(4 as usize, 5 as usize, 'h', "pqslhh"),
(7 as usize, 13 as usize, 'l', "gmpxpvwqrnlfp"),
(3 as usize, 6 as usize, 't', "sttxtmtn"),
(11 as usize, 13 as usize, 'r', "rrbmbrwrrrrrkhrr"),
(14 as usize, 16 as usize, 's', "ssssssssssssstsss"),
(7 as usize, 10 as usize, 'v', "vvvvvvhvdvvvkv"),
(5 as usize, 6 as usize, 'z', "sxpzzx"),
(2 as usize, 4 as usize, 'd', "rmxd"),
(16 as usize, 17 as usize, 'z', "zzzzzzzzzzzzzzzzzz"),
(1 as usize, 3 as usize, 'k', "kjkkkkcckkzk"),
(1 as usize, 11 as usize, 'k', "xzkkkkzkppk"),
(8 as usize, 9 as usize, 'f', "bfvfdffzb"),
(4 as usize, 14 as usize, 'r', "rfzcrrlmxqlrrrqr"),
(7 as usize, 19 as usize, 't', "gtnxjqtnjbkrwpzshqqn"),
(2 as usize, 5 as usize, 'j', "kjjgpddjpjjjffzjjp"),
(2 as usize, 3 as usize, 'f', "cfffh"),
(1 as usize, 2 as usize, 'x', "xxxxx"),
(3 as usize, 13 as usize, 'j', "jjjjjjjjjjjjzfjjj"),
(7 as usize, 8 as usize, 'm', "sgmmpmjmwmmmtfs"),
(4 as usize, 12 as usize, 'z', "zfzqzzszvtml"),
(6 as usize, 9 as usize, 'b', "jsfbpkzwb"),
(13 as usize, 16 as usize, 'x', "zsxxjxxsxxqxpxxx"),
(8 as usize, 12 as usize, 'b', "rlzdlplbgbdgd"),
(3 as usize, 14 as usize, 'h', "hmrhhhhhhhhhrthhhh"),
(15 as usize, 19 as usize, 'g', "mgggggcgggggqgghggg"),
(2 as usize, 9 as usize, 'p', "ppppptppzcf"),
(6 as usize, 7 as usize, 'b', "bbbbbbbbb"),
(4 as usize, 20 as usize, 'q', "skqqvxptdswwnrflkvxq"),
(4 as usize, 5 as usize, 't', "lqttq"),
(1 as usize, 10 as usize, 'l', "lqkqllvllj"),
(11 as usize, 15 as usize, 'm', "qmmmmmrmqmmmsmf"),
(6 as usize, 15 as usize, 's', "ssssstssssssssss"),
(2 as usize, 4 as usize, 'x', "xtxxx"),
(9 as usize, 11 as usize, 'q', "qqqqqqqqhqgqq"),
(1 as usize, 4 as usize, 'n', "gpnnfnn"),
(1 as usize, 3 as usize, 'l', "lltl"),
(11 as usize, 15 as usize, 'k', "kkfkkfkmmkrkkkk"),
(11 as usize, 12 as usize, 'f', "fkcvfvtqfcfffffffffj"),
(1 as usize, 4 as usize, 'c', "ccjc"),
(14 as usize, 15 as usize, 'n', "bvbvfvzcbfnzqlsvh"),
(4 as usize, 5 as usize, 'x', "xxlmxx"),
(3 as usize, 6 as usize, 'n', "nnrnnnwlnncnn"),
(6 as usize, 9 as usize, 'j', "jjjjjjjjq"),
(7 as usize, 10 as usize, 'd', "pdplmxdczddbd"),
(12 as usize, 13 as usize, 'c', "ccccbctccccccccc"),
(12 as usize, 13 as usize, 'j', "jfjdjjjjjjjjjj"),
(6 as usize, 7 as usize, 'h', "mrnphwh"),
(2 as usize, 9 as usize, 'n', "njnnnnnnnnnnnn"),
(3 as usize, 6 as usize, 'g', "rgxgggggnjghgggntg"),
(9 as usize, 12 as usize, 'b', "bbbbbbbbbbbcbb"),
(3 as usize, 5 as usize, 'p', "ppppvpp"),
(16 as usize, 20 as usize, 't', "ctkgpgzrwwngltvxcqct"),
(4 as usize, 5 as usize, 's', "sssdsh"),
(12 as usize, 14 as usize, 'v', "vvvvvsvvvvvvvsv"),
(8 as usize, 13 as usize, 'w', "zwwwwwwvwwrwgv"),
(12 as usize, 17 as usize, 'r', "wrcrrrrrrrbrwrrrxr"),
(12 as usize, 13 as usize, 'x', "xxxxxxxxxjsvrnxx"),
(7 as usize, 9 as usize, 'n', "nqnnqnvnn"),
(14 as usize, 19 as usize, 'n', "nnnnnnnnnnnnnnnnnnnn"),
(4 as usize, 5 as usize, 'c', "vscjrl"),
(1 as usize, 3 as usize, 'l', "llrl"),
(11 as usize, 12 as usize, 'w', "wwwwwwwwwwzww"),
(6 as usize, 7 as usize, 't', "wlcktht"),
(2 as usize, 10 as usize, 'r', "rrrrrrrrrwrrrmrr"),
(2 as usize, 6 as usize, 'x', "lhqvpx"),
(10 as usize, 16 as usize, 'h', "kqrhxclktcqhxchg"),
(6 as usize, 10 as usize, 'm', "mmsmkmmjmlmhfmmnmm"),
(5 as usize, 7 as usize, 'h', "hchhhhph"),
(5 as usize, 7 as usize, 'z', "vtzzzwl"),
(3 as usize, 12 as usize, 'z', "zzfzzzzzzzzzz"),
(7 as usize, 9 as usize, 'z', "zzzzzzszzzzzzzzjz"),
(8 as usize, 9 as usize, 'g', "ggggggggg"),
(13 as usize, 16 as usize, 'f', "ptvzfmfkxfdkfhjff"),
(1 as usize, 10 as usize, 'w', "cvhnfgnwpw"),
(5 as usize, 8 as usize, 'd', "fvvmdlfqgjc"),
(6 as usize, 9 as usize, 's', "rzlrwzngshvt"),
(2 as usize, 4 as usize, 'v', "vgql"),
(1 as usize, 3 as usize, 'r', "rrmrr"),
(5 as usize, 7 as usize, 'j', "jkjgwjj"),
(4 as usize, 7 as usize, 'b', "bbbzdzbbcbbb"),
(4 as usize, 10 as usize, 'k', "kkkkrkckkgkkk"),
(10 as usize, 12 as usize, 'm', "mmmmmmddmjmn"),
(4 as usize, 10 as usize, 'k', "mskmvkcpqkk"),
(5 as usize, 10 as usize, 'm', "wbtdmxnvrmwqbqkwmtq"),
(7 as usize, 14 as usize, 'z', "cfzftzzqnxffzh"),
(12 as usize, 13 as usize, 'z', "zzzzkzzzzdzzz"),
(4 as usize, 5 as usize, 'l', "lllslllvl"),
(5 as usize, 8 as usize, 'k', "kkkkkkkkkk"),
(10 as usize, 11 as usize, 'l', "llllllllllwl"),
(3 as usize, 5 as usize, 'v', "hvzpxfvmvcv"),
(8 as usize, 10 as usize, 't', "tbtnrtbqzwtkqtf"),
(6 as usize, 10 as usize, 'j', "njjjpjjjjkjsj"),
(8 as usize, 16 as usize, 'f', "cvpxnsxfdnpdfswdhbb"),
(6 as usize, 12 as usize, 'n', "nnndnnnnnnnz"),
(2 as usize, 3 as usize, 'd', "dzdd"),
(1 as usize, 4 as usize, 's', "jshkscssssssssssssss"),
(5 as usize, 7 as usize, 'k', "kkkqckwkcl"),
(3 as usize, 4 as usize, 'f', "ffdf"),
(9 as usize, 11 as usize, 'c', "cpccccncccqccc"),
(1 as usize, 8 as usize, 'x', "gxxxxgxx"),
(5 as usize, 15 as usize, 'p', "ppvkmmpcvzmmczpz"),
(12 as usize, 13 as usize, 'p', "xppppppppvpnpppp"),
(7 as usize, 12 as usize, 'n', "nwnnnnhcbnjnc"),
(1 as usize, 4 as usize, 'f', "fnzjf"),
(2 as usize, 5 as usize, 's', "tltqss"),
(3 as usize, 10 as usize, 'r', "rrqkrzvkrtbqcrp"),
(3 as usize, 14 as usize, 'h', "hhhhthhhlrwhhhthp"),
(2 as usize, 4 as usize, 'b', "bkbhbq"),
(15 as usize, 16 as usize, 'v', "vvvvvvvvvvvvvvvsvvv"),
(1 as usize, 17 as usize, 'h', "vtjjhtxrchshpxhsh"),
(4 as usize, 7 as usize, 'n', "jnpnpnn"),
(3 as usize, 4 as usize, 'h', "jvhz"),
(4 as usize, 5 as usize, 'w', "wcpzw"),
(9 as usize, 10 as usize, 'q', "tvxbsfmqqblhq"),
(3 as usize, 5 as usize, 's', "jssstxfbsssshssgkss"),
(3 as usize, 9 as usize, 'r', "fnrhqkrmtstqjgc"),
(12 as usize, 15 as usize, 'n', "xqwnnnnnnnnmnnn"),
(13 as usize, 15 as usize, 'q', "qqqqqqqqqqqqqqtq"),
(3 as usize, 4 as usize, 'd', "dcdl"),
(4 as usize, 12 as usize, 'd', "vrldnmpndmlgdzrv"),
(2 as usize, 4 as usize, 'h', "mhhh"),
(3 as usize, 4 as usize, 'f', "fcvfc"),
(1 as usize, 2 as usize, 'w', "whwwwz"),
(7 as usize, 8 as usize, 'm', "mpmlmmmmhdbh"),
(2 as usize, 4 as usize, 'q', "qxbqqdsjrdpxf"),
(6 as usize, 14 as usize, 'r', "wbmlhrcgrgrkzqfj"),
(2 as usize, 7 as usize, 'c', "ghcvcdcmcztckct"),
(2 as usize, 9 as usize, 'n', "nnnnnnnnpnnn"),
(3 as usize, 5 as usize, 'f', "zlgffv"),
(1 as usize, 6 as usize, 'm', "ntmmmm"),
(2 as usize, 4 as usize, 'w', "jgqwv"),
(5 as usize, 12 as usize, 'f', "gscfzhmrtxfw"),
(5 as usize, 7 as usize, 'r', "rwzklcrnrrg"),
(8 as usize, 10 as usize, 'h', "hhhzhhhpxhhh"),
(9 as usize, 11 as usize, 'x', "xxxxxxxxfxbx"),
(7 as usize, 8 as usize, 'q', "qdnqnzbq"),
(2 as usize, 10 as usize, 's', "sssmssslbb"),
(8 as usize, 9 as usize, 'n', "wgfnghnlnkf"),
(4 as usize, 10 as usize, 'd', "dddsdddlds"),
(1 as usize, 5 as usize, 'k', "bfkkkn"),
(2 as usize, 5 as usize, 'w', "wwwwww"),
(14 as usize, 16 as usize, 's', "bjszbzmcnsvplsrh"),
(8 as usize, 9 as usize, 'b', "bbjbbbbbbvvbbx"),
(2 as usize, 10 as usize, 'm', "dmnrsmtqkf"),
(7 as usize, 12 as usize, 'f', "fbtwftvffsgfwlnw"),
(9 as usize, 10 as usize, 'h', "shhhpshfxhbrdhshh"),
(4 as usize, 9 as usize, 't', "tgpdtwrmt"),
(2 as usize, 6 as usize, 't', "vhtwntl"),
(3 as usize, 5 as usize, 'j', "ljjjd"),
(2 as usize, 3 as usize, 'w', "hxwvbxwwbwsvc"),
(7 as usize, 8 as usize, 'r', "rrrzrrnr"),
(3 as usize, 4 as usize, 'x', "jxjh"),
(7 as usize, 12 as usize, 'w', "mjmbtgntdwjwnqztv"),
(5 as usize, 6 as usize, 'l', "vlvllt"),
(7 as usize, 8 as usize, 'n', "nnnnnnpnnnn"),
(3 as usize, 10 as usize, 'c', "wcgcxzcdwmcn"),
(16 as usize, 18 as usize, 'h', "hchhhhhhhhhhhhhcmh"),
(5 as usize, 11 as usize, 'f', "fflffffffflfff"),
(3 as usize, 13 as usize, 'z', "zzzzzzzzzzzzpzz"),
(6 as usize, 9 as usize, 'k', "kkkkkskkkk"),
(6 as usize, 15 as usize, 'c', "ccccccccbccccctccc"),
(9 as usize, 18 as usize, 'p', "klcpzpdwzvpqppspfpp"),
(10 as usize, 13 as usize, 'b', "pbbbbmbbdbwtmd"),
(10 as usize, 11 as usize, 'v', "xvvvvvvvvnnvv"),
(2 as usize, 4 as usize, 'm', "msmmm"),
(1 as usize, 4 as usize, 'w', "rwwlwrwrwwrfngc"),
(8 as usize, 9 as usize, 'r', "rjjlddjrnbr"),
(13 as usize, 16 as usize, 'd', "pzdfzqbwclbjddxtvddf"),
(14 as usize, 15 as usize, 'q', "qqqqqqqqqqqqqbq"),
(12 as usize, 14 as usize, 'k', "kkkjgrkkqkkkkl"),
(3 as usize, 4 as usize, 'd', "gsdnkdfnf"),
(8 as usize, 17 as usize, 'h', "glhfvrshlrqwdrfrh"),
(2 as usize, 12 as usize, 'l', "mflqfvxfgzkmd"),
(5 as usize, 8 as usize, 'f', "ckllfnfbflqgrsd"),
(1 as usize, 17 as usize, 'm', "kckvffhnlmjvdtgpm"),
(16 as usize, 17 as usize, 'p', "pplppppcppppppppppp"),
(5 as usize, 8 as usize, 'h', "hhjbmplh"),
(7 as usize, 10 as usize, 's', "jsjlwgsssbsvfsvk"),
(2 as usize, 8 as usize, 'x', "xpfxbqxxqxhdrxhqm"),
(12 as usize, 16 as usize, 'n', "nnnnnznnnnnnnnnmnvn"),
(6 as usize, 12 as usize, 'v', "vvvvvvvvvvvgv"),
(8 as usize, 9 as usize, 'j', "pjjjjmjnj"),
(16 as usize, 17 as usize, 'h', "hhhbhjhrhhhhxhhgt"),
(3 as usize, 11 as usize, 'd', "ldpmvddhdrdjdj"),
(6 as usize, 7 as usize, 'n', "nnnnnnnn"),
(5 as usize, 8 as usize, 'f', "tglffvhgnfxzfhf"),
(13 as usize, 18 as usize, 'r', "rrrrrrrrrrbrhrrrrrrr"),
(19 as usize, 20 as usize, 'n', "nnnnnnnnnnnnnnnnnnnj"),
(7 as usize, 8 as usize, 'w', "tpmmxqsw"),
(5 as usize, 7 as usize, 'c', "ccccccr"),
(9 as usize, 10 as usize, 'l', "qltnnlnfllqlw"),
(6 as usize, 7 as usize, 'g', "xggbggz"),
(7 as usize, 10 as usize, 's', "sssssfcssss"),
(5 as usize, 7 as usize, 'j', "jsjkxwqhjcvjtwjzl"),
(10 as usize, 14 as usize, 't', "qdtttzttcvtttnn"),
(12 as usize, 13 as usize, 'b', "bbbbbbrbbbbqb"),
(1 as usize, 15 as usize, 'd', "dshhrjkwcjjhlthdts"),
(7 as usize, 12 as usize, 'p', "hrxkphmqpvpptpqbw"),
(13 as usize, 14 as usize, 'd', "ddndddxdtdrkvldd"),
(3 as usize, 4 as usize, 'h', "htht"),
(7 as usize, 8 as usize, 'c', "xtsvzccfckccx"),
(4 as usize, 5 as usize, 'r', "gstrwshptzrdtjj"),
(7 as usize, 8 as usize, 'b', "wbbnbbbm"),
(15 as usize, 17 as usize, 'c', "cgpqxbccqcjpzlcctmx"),
(2 as usize, 7 as usize, 'k', "kvtqqmsx"),
(8 as usize, 11 as usize, 's', "ssxssssqsssssssss"),
(3 as usize, 9 as usize, 'd', "ddddddddld"),
(13 as usize, 16 as usize, 'p', "pppppppppppppppwpj"),
(6 as usize, 8 as usize, 'v', "sxkghpckvb"),
(17 as usize, 18 as usize, 's', "ssssssssbsssssssksss"),
(1 as usize, 2 as usize, 'w', "wlwxdsw"),
(8 as usize, 9 as usize, 'q', "qqqqqqqqnq"),
(9 as usize, 16 as usize, 'f', "fjdsfvkfqffffjcfpff"),
(12 as usize, 13 as usize, 'h', "bhhhhhhfwhphhhhhh"),
(7 as usize, 8 as usize, 'k', "kkkkkknkkkkkkk"),
(4 as usize, 7 as usize, 'w', "wwwfpsw"),
(8 as usize, 11 as usize, 'd', "rsndldddddxddmf"),
(2 as usize, 10 as usize, 'c', "cjcdcccccc"),
(6 as usize, 7 as usize, 'v', "zvnrhth"),
(3 as usize, 8 as usize, 'z', "zzxzzzzzdzjzzzzz"),
(11 as usize, 12 as usize, 't', "tctdttttwtrtttttjth"),
(8 as usize, 9 as usize, 'c', "ccccccccrccc"),
(17 as usize, 18 as usize, 'p', "pppppppppppppppppvp"),
(3 as usize, 8 as usize, 'l', "svlmlkspljr"),
(1 as usize, 2 as usize, 'n', "nwnkq"),
(1 as usize, 11 as usize, 'j', "jjjjjjjjjjjj"),
(18 as usize, 19 as usize, 'g', "ggggggggggggggggggr"),
(10 as usize, 11 as usize, 'j', "jjjjjcdjgjv"),
(3 as usize, 7 as usize, 'p', "ptttppppppj"),
(2 as usize, 5 as usize, 'd', "cdndsd"),
(6 as usize, 10 as usize, 's', "sssssmssssss"),
(15 as usize, 16 as usize, 'k', "vxwxxhhkkhklqksd"),
(3 as usize, 4 as usize, 'x', "rpxn"),
(1 as usize, 6 as usize, 'g', "vmgckg"),
(3 as usize, 4 as usize, 'j', "jjbs"),
(5 as usize, 10 as usize, 'd', "qrnmbddndvcmdsjjbdhd"),
(7 as usize, 9 as usize, 'v', "vvmgvvvpvm"),
(1 as usize, 7 as usize, 'z', "zzzzzzwzzzz"),
(4 as usize, 7 as usize, 'n', "nnnnnnqn"),
(8 as usize, 9 as usize, 'k', "kwkknknkrkgkbklmpb"),
(1 as usize, 5 as usize, 'z', "zzmzfzz"),
(6 as usize, 10 as usize, 'm', "mmmmmfmmmm"),
(9 as usize, 11 as usize, 's', "sssssstsssgss"),
(2 as usize, 6 as usize, 'n', "nnfnpgnnnmnnn"),
(15 as usize, 17 as usize, 'w', "wwwwrswthgwhkwwrw"),
(5 as usize, 9 as usize, 'h', "lbhdhplmbnwh"),
(5 as usize, 6 as usize, 'd', "jdddqqt"),
];
// #endregion

// #region day 3
pub const DAY_THREE: &str = r"........#....#..##..#...#.....#
...............#....##........#
.#....##...##..#...............
.#.......#......#.##..##...#...
.....#.#....#..##...#.....#....
...#.#..##...###......#..#..#.#
.....#..##........#.##......#..
..##.....###.........##........
..............##..#.#.#.#......
.#....##..#.##.#....#..#.#..#..
.#.#....#.##.#...#....#.....#..
..#...#.#.....#....#.......##..
.#.#..##.....#...........#.....
.#.##...#.....#......#.##......
..#..#..........#.....#..###.#.
##....##....#.#...........#..#.
.....#.#.......#.#.#..#.##....#
...##.#....#..#.....#.........#
.....#........#.##...#.........
.....#................#.#...#..
...#....##.....##....#.......#.
....##.#.....#.#.......#.......
#...............#..#...........
.......###.#.......#.##....#.#.
..#........###........#......#.
.#.......#...##.....####....##.
..##.#....#.....#..#....#......
..#...#..#.#..##...#.....#.....
.#.......###.......#....#......
...#...#.......#........#...#.#
..#....#...#.......#.#..##.....
##............#.#..#..........#
.......###...##..#.....#....#..
##..######.#..#.......###....##
###..#...#.##......##....#...#.
..............##.###..........#
.....#........##.#.###....#....
..#...#.....##.#......#.#..#.#.
#....#.............#.#.........
.........##.#........#...#.....
..........#..##.#.#.....#..##..
........##......#..#..#...#.#..
.##.......#..#.#...#.####..#...
##...#........#.###...##....#..
....###.####...#..#..#......###
#....#....#.#.....##.........#.
#.......#....#....##...........
##...##.#.......#....#...#....#
....#....#........##..#.#..#.#.
..##.....##...#..........#...#.
.#.#.#...#.....##..#........#..
#....#.....#..........#....#...
...##.#.......#.#.........#....
##.##.........##.....##.....##.
##.#..##..#...##........##.....
.........##.......#....#...#...
.#.....#........####.#.#.....#.
...........##..#.###...........
..#....##....#...#.............
#.#............#.......#.......
.##........#...#..##.....#.#...
#.##..............##..##.......
##.........#......#......#..#..
##.#....#...#....##....#..#.##.
......#...#..#.#...#.#....#.##.
##.......#.....#.........#.....
...##...#................#.#...
....#.####...#.#.....##....##.#
#...#..#.#.##................##
.........##.....##...#..#......
......####....#.##.#.....#.....
...#..#.#....#.#.#..#..........
.....#........##...#.##....#.#.
..##......#...................#
.....#..#...............#..#...
....#........#..#.#...##...#.##
..#.#.......#.##.........#...#.
...##......#.#.................
.#....#...#.............##.#...
........#.##...#..#...###.....#
#....#.#........##....#......##
.###.......#..#..........#..#..
#....#..#....#........#...#...#
##.#.###.##.#...##.#......#.#.#
#..#..#..##........#..###.#...#
....#..#..#.....#...##....#...#
.......##.......#..#.##...##..#
.##....#..###................##
#...#.##.##...#.##......##.....
...##.....##..##...#..#........
...............##.....##.......
.#..#.#..#....#.....#..#...#...
.#....#..#........#.#...#.....#
##.....####..#......#..........
........#.........#.........#..
#...####....#.##...#....#...##.
.#....####..#...##..#......####
...........##.##..#.##...##....
..#..#.......#.##....#.#...#.##
#...........#..#...............
.......#.##..#.....##......#...
....##.#.##.....#...........#.#
.............#.##..#...#......#
#......#...........#........#..
#.#..#.............#...#.......
#.........##...#....#...##.....
##...#..#..#..#....#...........
.#.....#........#.....#.##..#.#
...#..............##.####.#..#.
##.....#..#.#..#..##...........
...#...#.......#...............
..#..................###..#..##
....###..........#.#..#...#.#.#
..#..#..#.#..........#.#......#
....#....#.#...#.###...##..#...
....#.......#...#....##........
.#.....#.......###....#........
....#..#..#.....#......#.......
......#...#..#....#.#.......#..
.##.#..#...#.#.#...........#...
..#....##.#....#.#....#...#.#.#
...##..#.......#....#.#.....##.
##.#......#.#.......##...#.....
......#...#.##..............#..
.##.........#......##.#..#....#
#.......#.....#...##...#..#...#
..#..##.......#......#......##.
#..##...###.#.#...........#....
##......#.....####..#..#....#.#
.......##...##.#...#...........
....#..#.##.#.....#.#....#.#...
....#.....#.....####...#..#.##.
.##..#..#..###...#....#.##.#.#.
..#.#.##..........##...........
#.##.#.#....#.##....#..#...##.#
#...#....#...###....#.......#..
.......#..#............#.......
................##.#.#.....#..#
..........................#....
.##....##...#.#....####..#....#
......#...#....#...#.##..###.#.
.........#............#.......#
.#.#..#........#..#.........#..
#..#...#......#.#....#..#.#....
...........#.................#.
.#.#..#...##..###......##....##
.#.#.##......####.........##...
..#....#.#..#................#.
##.......#....#.........##.#.#.
##..#.###...........#..#.#..#.#
...#............##.#....#......
...#................##.#..#....
....#..##.#...#.#.....#.......#
......#......#.#........#..##..
...##...#.....#.##.......#.....
##...#...#.............#..#....
..#...##.....#..........#..#.##
#.##...#..................#.###
.........#..........#.###...#.#
#..#.....#.#.#....#......#...#.
.............#.##..###.....#.##
..#..#.....#..#.............#..
.#.....##.#.#..#.........#.....
..#.......#....#.....##.#......
.#.........#..#....##...#.##...
.##..##................###....#
.#..##..............#...#......
.#..............#.##....##.....
.#......#..#..##..#...###.....#
................##...#.#..#...#
##.#.......#...................
....#.#.......#..#.##..........
....###............##...#......
.......#....#.#.....##.#.....#.
....#...............#.#........
..#.##....#.#.#......##..#.....
.##......#...#.#..#..#.......#.
....#...#........#.#..##.......
.##...###.#....#..........##..#
..#.......##..#.....###......#.
...#.#..##.#.#...........#.....
##........#.#..##.........#..#.
.....###.......#..#.#.....##.#.
..#...##.#..............#......
......#...#...............##.#.
##...#..#....#...#.####.##.....
...#............#.##...........
...#........##.#.##.......#....
...#..#..##....#...#......#..#.
#.....#..#......#.#.....##.#.#.
.....#.##......#...#..#..###..#
...........##..##.#.#..........
...#........##........##..##.#.
......###...#.....#..###.#..#..
#.....#.#....#...##....##.....#
.##....#......#.....#.#..##.##.
##....###.......#...##.......##
...##......#....###............
..#...#...#.#..#..........##.#.
#.#.###...#..#.....#....#.###..
..##.....#.#.#.......#.........
...####..#....#..#.........#...
.##...........#.##.#...#.#.##..
...#.#....#.##......#........#.
##....##....#..#...#..#.#......
#......#..#...#...#.#.#.#.####.
....##.#.#.....#.###........#..
....##..#.#.#.#....#....#.#..#.
..#.###..#............##..#.#..
...#...#..#...#.#.#.....#.....#
..........#.....#..#.......##.#
..............##...........#...
.......#.....#...#.....#.....#.
.#.###.....##......##....#...#.
.....#.........#.#....#........
..#.#....#.##...#.##....##...#.
...#......#.#.....#.......###..
#.##....##.....#.#.#...#......#
#..#...#..........#.........##.
....#.#.#.#.....#...###........
#.#..#...#......#...#.##...####
.#...#......#....##...#........
..#.........#............#...#.
##......#..#...#....#.##....#..
.#...##..#..##.#.#.#........#.#
.##.........###...#......#..###
...##.....##..#.#.........#....
...........##........#...#.....
..##..#...#..#..#.....#......#.
..#..#.#....#.....#..#.##...#..
#....#........##..........#.###
......#...#...#....#...##.#....
...#......#.#.....##......##...
#....#..##............#....#.#.
...#...##.#..........##........
......#.###......#...#.#.......
..................#.##..#..#..#
....#.....#...#.....#...#....#.
.#....##.#..#..#.....###.##...#
#.......#..#....##.##.#.....##.
..##........##...#.....#....##.
#.........#...........##.#.....
.#....#.#...##..###..........#.
....##..##....####...#......#..
##.##..#..#....#....####...#...
..#...............#.##.........
...#.#....#..#....#......#.....
.#..#...#........#...#.....##..
#.....###.......#.....#........
...#.##..#.......#....#........
....##..###.##...#.#....#.#....
#.####...#.......#.....#.#....#
#.......#......#.......#.#.#...
##....#......#..#...#..#..####.
.##.....#........#..#...#......
#.#.#....#....#...#.##..##.....
....#..#.........###.##.##.....
...##...##.###..#..##.....#.###
..###.......................#..
......##..#.#.........#......#.
.###......##....#.....#.......#
.....#..#..##........#......##.
..##.....#....#.#.............#
..##.........##.#..#.........##
......#......#.#......#........
.#...#..#......##...#..#....#..
...............###............#
#.####.#....#...#...........#.#
............................#.#
.#..#...#.#.#.###..##.....##...
....##...#.................##..
......##....#...............##.
.#......#.##.#..#.....##...##..
.............#........#......#.
#..........#.#....#####.#...#.#
.#.#...##..#.#...#.#..#.#..#...
#.##.......##......#.#.#....#..
##.....##.#.#.##..........##..#
....##..#.#.......#....#.##....
..#.#.#...#.....#.......#......
.#....#..#...........###.......
#.#...#.....#......#...#.....#.
#........#.#..........#...#.#..
...#...#....#.........#........
.....................#..##.....
...#......##........#.##.#.#.##
.............###...#.#...#..#..
.#..##........##....#...###..##
.#..#.#...............#.....##.
...........##.#....#..##.#....#
.##.#.#..#.#..#...#.#.#..#.#.##
.......#.#..#..#..#..#...#.....
.#......##............#.#..#...
..#...#..##..#..#...##......#..
...##......##....#............#
.......#.....##...##.#...#..#..
......#.......#..##.........#..
..#...#.#.....#.#.......#.#...#
.#......##.##.#.#.#.##..#....##
#.....#.........#.#....#....##.
.......#.........#....#..#.#.##
.....##....#..#.#.#...#.....##.
#####.#.......######......#....
..##.#.......#.#..............#
..#.##....#.....#...#.#...##...
.....#...#..#....#.#..#........
.#....#.#..#.#.#.##..#.......#.
....#..#..#..........##...#....
.......#.#......#........#.....
##.#.#.###....##.#..#..#....#..
#.##......#..#.......#.#...#...
..##...#.......#.......#...#...
........##.........#.#....#.#..
..#...#..##.#.#.#...#....#.....
.###......#........#....#...#..
.#.......##......###..##.......
#....#.#....#.##.........####..
......#..........#..##.....#...
.............#......#..##.#....
...................#....#...#..
.#..........#...#.#..##...#....
.....#...#..........##.##......
#...#..#.##........#...#.......";
// #endregion

