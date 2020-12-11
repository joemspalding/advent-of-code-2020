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

// #region day 4
pub const DAY_FOUR: &str = r#"eyr:2024 pid:662406624 hcl:#cfa07d byr:1947 iyr:2015 ecl:amb hgt:150cm

iyr:2013 byr:1997 hgt:182cm hcl:#ceb3a1
eyr:2027
ecl:gry cid:102 pid:018128535

hgt:61in iyr:2014 pid:916315544 hcl:#733820 ecl:oth

hcl:#a97842
eyr:2026 byr:1980 ecl:grn pid:726519569 hgt:184cm cid:132 iyr:2011

ecl:grn hcl:#6b5442 pid:619743219 cid:69 hgt:176cm eyr:2027 iyr:2012
byr:1980

ecl:brn byr:1969 iyr:2014
hgt:164cm eyr:2020 pid:982796633 hcl:#602927

ecl:gmt
iyr:1987 eyr:2039 pid:15115163 byr:2006
hcl:bfab0d

cid:117
hcl:#efcc98
iyr:2010 pid:322719183
hgt:176cm
eyr:2020
byr:1957
ecl:brn

byr:1954 hgt:178cm hcl:#38f7fd pid:838813262 ecl:blu
eyr:2029 iyr:2019

eyr:2023 ecl:amb iyr:2020 byr:1927 pid:242570886 hcl:#18171d hgt:192cm

iyr:1990 cid:295 hgt:131 pid:187cm byr:2014
ecl:xry hcl:z
eyr:1928

ecl:hzl
byr:1953
eyr:2023 hcl:#866857
hgt:181cm iyr:2010 pid:568185567

byr:2030 hcl:#fffffd ecl:#a4a596 hgt:168cm
iyr:1936 eyr:2020 cid:296 pid:168786676

byr:2030 iyr:2026 eyr:1974 hcl:7fcaa5 ecl:utc
pid:190cm
hgt:67cm

byr:2023 eyr:2037 hgt:59cm
ecl:lzr hcl:z iyr:2026 pid:#ea9083

byr:2003 hcl:z hgt:91 iyr:1990 eyr:2024 ecl:#123d73
pid:48494230

byr:2022 eyr:2020 iyr:2030 ecl:gmt
hgt:191cm pid:3509331253 hcl:#888785

iyr:1994
ecl:#c3d564 byr:2009
hgt:162cm hcl:336498 pid:#e99d09
cid:288
eyr:1921

byr:1924 cid:290 iyr:2010 ecl:amb eyr:2020
hgt:156cm hcl:#7d3b0c pid:795497164

cid:301 iyr:2017 hgt:67cm
hcl:#888785 ecl:#0405b9 byr:1964 pid:707857518 eyr:1976

ecl:gry pid:474303066
iyr:2011 hcl:#18171d hgt:165cm byr:1921 eyr:2024

hcl:#6b5442 ecl:amb iyr:2020 hgt:191cm
byr:1949 cid:301
pid:075846582 eyr:2029

hcl:#a97842 cid:186 iyr:2014
ecl:gry
hgt:191cm eyr:2023 pid:645548969
byr:1956

pid:154cm hcl:z ecl:gmt iyr:1989 hgt:69in cid:53 byr:2010

hgt:72cm byr:2023
eyr:2034 hcl:z ecl:#f5249e iyr:1997 pid:#79af7a

eyr:2038 byr:2015
hgt:70cm ecl:grt hcl:9d58a1 iyr:1926 pid:6290928420

pid:620857794 eyr:2022
byr:1950
hgt:159cm
hcl:#ceb3a1 ecl:amb iyr:2015

eyr:1954 ecl:#ab2ce4 pid:#14eedd
iyr:2009
hcl:29e484
byr:2022 hgt:73cm

hgt:59cm byr:2026 cid:245 iyr:2020
eyr:2029 pid:073943129 ecl:hzl
hcl:#b6652a

iyr:2014 byr:2015 hcl:#a97842 eyr:2029
pid:#132098
hgt:150 ecl:oth

hgt:151in ecl:#967d49 eyr:2026 hcl:#18171d
pid:384230726 byr:1934
iyr:2018

iyr:2020 eyr:2021 byr:1937 pid:735047371 cid:159 ecl:blu hgt:177cm hcl:#22b774

ecl:brn hcl:#6b5442 pid:117807698 cid:105 iyr:2016 byr:1977 hgt:183cm

ecl:hzl hcl:#6b5442 byr:1933
iyr:2019 pid:348486702
eyr:2020 hgt:193cm

byr:1928
ecl:gry
eyr:2028 hcl:#fffffd pid:571149069
iyr:2012 hgt:175cm

pid:359108298
eyr:2027 hgt:158cm ecl:amb iyr:2016
hcl:#602927

iyr:2027 byr:2015
hgt:191in pid:102033301 ecl:xry
eyr:2031 hcl:#602927

ecl:oth cid:163 hcl:z iyr:2014
byr:1944 hgt:173cm
eyr:2027 pid:#0524c1

ecl:brn
byr:2030 hgt:71cm eyr:1931 cid:165 iyr:2010 hcl:#cfa07d
pid:509642098

hgt:166 iyr:2020 cid:308
eyr:2022 pid:950463527
byr:2017
hcl:z

ecl:amb
eyr:2023 byr:1924
pid:901038027 hgt:70in
iyr:2010 hcl:z

byr:1972
iyr:2013
hcl:d669ad hgt:64cm cid:247 ecl:#19aa26 eyr:2023

hgt:71 hcl:#fffffd
byr:1976 cid:108 eyr:2038
ecl:grt iyr:2018 pid:190cm

iyr:2017
byr:1963 ecl:grn hgt:175cm
pid:160915270 eyr:2028 hcl:#cfa07d

pid:569740130 hgt:171cm hcl:#733820
ecl:gry eyr:2024 iyr:2020 byr:1973

byr:1937
iyr:2016 ecl:gry hgt:181cm pid:521705827 hcl:#b6652a eyr:2027 cid:295

hgt:156cm ecl:blu iyr:2019 hcl:#866857
pid:662418718 byr:2000 eyr:2024

byr:1971 pid:693616099
hcl:#efcc98
hgt:175cm iyr:2016 ecl:gry
eyr:2023

iyr:2013
eyr:2024
ecl:gry
pid:414295491 byr:1986
hgt:188cm hcl:#b6652a

eyr:2022 byr:1975 iyr:2020
ecl:grn cid:68 hcl:#a97842
hgt:151cm pid:229803943

cid:258 iyr:2012
ecl:hzl
byr:2001
eyr:2021
hcl:#866857 pid:990590217 hgt:172cm

cid:339 byr:1957 hcl:#866857 pid:343480061 eyr:2039
hgt:191cm
iyr:2021
ecl:utc

cid:281 hcl:z ecl:blu
byr:2020 pid:132694306 eyr:2020 iyr:1953

hcl:#602927
byr:1933 eyr:2028
hgt:165cm ecl:gry iyr:2018 pid:658484617

ecl:oth
hgt:188cm cid:110 pid:056975690 iyr:2016 byr:1950 eyr:2023 hcl:#cfa07d

cid:342 hcl:#fffffd eyr:2024
pid:153555359 byr:1974
ecl:gry hgt:191cm iyr:2020

byr:2019 ecl:#160ed3 eyr:1999 hcl:z
cid:146 pid:195693972 hgt:159cm

iyr:2015 eyr:2030 hgt:191cm byr:1979
ecl:#ec4873 pid:994113786 hcl:#cfa07d

pid:552331609
ecl:grn
hgt:171cm eyr:2022 hcl:#b6652a
iyr:2020 byr:1931

hgt:177cm iyr:2010 pid:934058099
eyr:2020
ecl:blu
byr:1967
cid:112 hcl:#7d3b0c

iyr:2028
hgt:138
cid:180 hcl:z
eyr:2022 pid:3286566621 byr:2002

eyr:2020
iyr:2019
hcl:#a97842 pid:149148750 ecl:brn hgt:159cm
byr:1981 cid:339

cid:344
eyr:2021 byr:1968 pid:777786047
ecl:grn hgt:192cm hcl:#888785
iyr:2015

hgt:173cm
eyr:2030
hcl:#733820 pid:610226642 byr:1954 cid:80
iyr:2013 ecl:blu

byr:1999 eyr:2023
ecl:amb pid:912145128
hgt:181cm
iyr:2015 hcl:#a97842

eyr:2027 hgt:188cm
pid:080715145 hcl:#341e13 iyr:2013
ecl:oth
byr:1965

hgt:170cm byr:1950 iyr:2013
pid:010541784
eyr:2027 ecl:zzz
hcl:a3bae8

hgt:190cm eyr:2024 ecl:#6dcedc pid:909319684
iyr:2011 byr:1959 hcl:z cid:182

eyr:2028
iyr:2016 hcl:#623a2f pid:208417572 byr:1929 cid:137 ecl:hzl
hgt:167cm

hcl:#6b5442
ecl:grn
byr:1938
eyr:2023 cid:307
hgt:59in iyr:2014 pid:205268145

pid:047489285 eyr:2026
hcl:#b6652a byr:1920
iyr:2015
hgt:183cm ecl:gry

ecl:blu hcl:#508e8b iyr:2016 eyr:1954 hgt:151cm pid:086752750 byr:1920

iyr:2011 byr:1981 hgt:186cm
cid:117 hcl:#6b5442 ecl:amb
pid:756830713 eyr:2026

eyr:2037 pid:364464758 hcl:z ecl:grn
hgt:112 iyr:2013 byr:2022

ecl:hzl
cid:65 pid:679487194
byr:1986 hgt:169cm hcl:#cfa07d eyr:2025 iyr:2013

cid:192
byr:1921 pid:#5fe831 ecl:#fbb2b9 hgt:62cm eyr:1971 iyr:2024
hcl:z

hcl:#cfa07d eyr:2026
hgt:74in
iyr:2019
ecl:xry
pid:622690982 byr:1982

eyr:2026 pid:523515724 iyr:2013 byr:1973 hgt:167cm
ecl:grn hcl:#866857

byr:2009
eyr:1985 pid:484497014 ecl:#0bfcf2 iyr:1992 cid:131 hcl:39d6b0 hgt:177in

eyr:2020 iyr:2016 ecl:brn hcl:#ceb3a1 byr:1966 pid:696621560 cid:62
hgt:59in

hgt:166cm hcl:#7d3b0c
iyr:2016
ecl:brn pid:190cm
eyr:2020
byr:2001

eyr:2021
iyr:2012 hcl:#6b5442
ecl:amb hgt:169cm
pid:969150085
byr:1925

ecl:brn hgt:175cm byr:1992 iyr:2016 pid:415209726 eyr:2027
cid:72 hcl:#866857

iyr:2017
hcl:#733820 byr:1938 eyr:2020 pid:274486958 hgt:163cm

hcl:4f5dd1 cid:336 ecl:grn iyr:1931 pid:6212280197
byr:2016 eyr:2037
hgt:187in

iyr:2017 byr:1940 eyr:2025 pid:115098205 hgt:151cm
ecl:grn
cid:122
hcl:#6b5442

hcl:#efcc98
iyr:2020 pid:709548547 hgt:179cm
eyr:2030 ecl:gry byr:1975

cid:217 hcl:#888785 eyr:2029
ecl:hzl iyr:2013 pid:160053490
hgt:166cm byr:1992

eyr:2024 cid:188 iyr:2016 hcl:ff3a59 ecl:xry pid:296357512 byr:2026

hgt:154cm iyr:2010
ecl:blu pid:717041634 byr:1928 cid:123
eyr:2027
hcl:#a97842

pid:391011205 ecl:hzl hgt:191cm iyr:2016 eyr:2028 cid:281 byr:1934

byr:1937 hgt:65in
pid:667975382 ecl:gry cid:270 eyr:2024
iyr:2012

hgt:179cm pid:065528723
hcl:#888785 byr:1937 eyr:2028
iyr:2013 ecl:hzl

iyr:2027 cid:261 eyr:2037 ecl:#ced7d5 pid:157cm
hcl:3a80c1 byr:2029 hgt:187in

eyr:2028
hgt:157cm hcl:#733820
iyr:2012 ecl:blu byr:1952 pid:915063263 cid:335

eyr:2023 hcl:#efcc98 pid:490625944 byr:1961 ecl:grn hgt:155cm iyr:2018

cid:247 pid:2807544665 eyr:2021
ecl:oth
hgt:191cm
byr:1928
iyr:2013 hcl:#623a2f

eyr:2015
byr:2021
hcl:40d2fc hgt:69cm pid:159cm ecl:gmt

hgt:175cm eyr:1992 cid:328 pid:263110997 ecl:#e53989 byr:2014 hcl:#a97842 iyr:2026

pid:491396731 eyr:2027 hgt:172cm hcl:#623a2f cid:92 iyr:2017 byr:1983 ecl:grn

hcl:#fffffd
iyr:2018 byr:1983 pid:714591144 ecl:grn eyr:2021
hgt:160cm

eyr:2027
hgt:63in ecl:blu byr:1987 pid:397963077 iyr:2018 hcl:#ceb3a1

eyr:2027
hgt:184cm
hcl:#6b5442 iyr:2012 byr:1984 ecl:blu pid:196287205

iyr:1998
ecl:hzl
pid:7872103596 byr:1991
cid:275 eyr:2039
hgt:174cm hcl:0d2ad6

iyr:2010 hcl:#efcc98
byr:1992 hgt:65cm eyr:2038 pid:383236012 cid:68 ecl:lzr

hgt:190in cid:127
byr:1947 pid:515728209 hcl:#733820 iyr:2014 ecl:amb eyr:2020

iyr:2017 eyr:2028
hcl:#623a2f
byr:1964 ecl:grn pid:198467794 hgt:169cm

ecl:utc
hgt:59cm byr:2007 iyr:2030
hcl:7ac4db eyr:2038 pid:#7206c6

iyr:2010
hcl:z eyr:2021 ecl:brn
hgt:173 cid:86
pid:194240791 byr:1975

pid:9347286034
hgt:63cm
iyr:1992 eyr:2034 hcl:66031b ecl:grt byr:1929

pid:593398904 byr:1939 iyr:2019 hcl:#b6652a ecl:gry eyr:2023
hgt:70cm

byr:1991
iyr:2019 hgt:164cm pid:282852411 cid:340 ecl:amb
hcl:#341e13 eyr:2027

eyr:2020
iyr:2014 ecl:grn hcl:#866857 hgt:158cm
byr:1931 pid:321748597

cid:98 byr:2023 iyr:2019 pid:#48f79f
hcl:73c882 eyr:1973 hgt:151in
ecl:utc

iyr:2023
hcl:#18171d
pid:52221892 eyr:2039
byr:2008 hgt:72cm ecl:#db8d14

iyr:1966 cid:274
eyr:2034 pid:12256322
byr:2006 ecl:dne
hcl:985c2d

hcl:#fd033b
eyr:2026 ecl:blu
iyr:2016
byr:1953 hgt:157cm
pid:502619036

byr:2015 pid:159cm iyr:2025
hgt:158cm eyr:1943 hcl:z ecl:grn

ecl:blu iyr:2016
pid:842400950
hcl:#733820
cid:266
eyr:2027 byr:1931
hgt:161cm

iyr:2017 hgt:190cm byr:1994 pid:706570967
ecl:hzl hcl:#18171d
cid:180

cid:197 pid:204952666 ecl:amb
hgt:70in iyr:2016 byr:1936 hcl:#98cbe3 eyr:2025

pid:555499128
byr:1971 hgt:71in
cid:83 ecl:blu
hcl:#cfa07d eyr:2027

ecl:hzl iyr:2014
pid:30428184 cid:237
hgt:171cm byr:1942 hcl:#888785 eyr:1986

eyr:2025
pid:579385370 hgt:193cm
hcl:#c0946f byr:1979 iyr:2016
ecl:amb cid:284

eyr:2029 byr:1946 pid:278271295
ecl:grn
hcl:#cfa07d cid:271
hgt:172cm
iyr:2020

pid:731752614 eyr:2020 byr:1983
cid:248 ecl:oth hgt:179cm
iyr:2017 hcl:#fffffd

hcl:z
cid:203 eyr:2032 ecl:#3f9d3d hgt:65cm pid:4042846885 byr:2019
iyr:1946

hgt:171cm ecl:gry eyr:2027
iyr:2013
hcl:#7d3b0c pid:92288579
byr:1955

ecl:brn hgt:164cm byr:1969 hcl:#cbf9c9 pid:022724981 eyr:2030 iyr:2013 cid:244

hgt:162cm byr:1974 iyr:2015 pid:927525094 hcl:#3d3011 ecl:blu
eyr:2023

hgt:157cm
eyr:2020
pid:221286943 hcl:#fffffd ecl:amb iyr:2018 byr:1945

iyr:2019
eyr:2025 byr:1997 pid:341544323 hgt:174cm cid:113
ecl:hzl

pid:138492032 hcl:e35302 ecl:#caaede
eyr:1931
byr:2001 hgt:156 iyr:1998

pid:912182030 cid:189 hgt:162 hcl:#277b39
iyr:2013 eyr:2023 byr:2023 ecl:blu

eyr:2027 hcl:#fffffd
ecl:brn
cid:304 iyr:2016 byr:1969
pid:866607511 hgt:192cm

hgt:64in
ecl:amb
byr:1958
pid:720439412
iyr:2015 eyr:2022 hcl:#ceb3a1

eyr:2024 hgt:159cm
pid:187867283 iyr:2016
ecl:oth hcl:#fffffd
byr:1988

ecl:#910bf2 byr:1969 iyr:2011 hcl:z eyr:2024 pid:579502502
cid:103 hgt:174cm

pid:718692455
eyr:2028
iyr:2016
hcl:#602927
ecl:blu byr:1954
cid:251 hgt:182cm

eyr:2021 hcl:#341e13 ecl:amb
byr:1933 hgt:179cm iyr:2011 pid:083172316

iyr:1998 hcl:z eyr:1944
byr:2006 pid:453368738
hgt:160 ecl:#9da5f1 cid:261

hcl:#7d3b0c
iyr:2018
hgt:164cm eyr:2020 byr:1940 ecl:blu

pid:993701676 eyr:2028 ecl:gry
byr:1951 hcl:#888785 cid:116
iyr:2020
hgt:192cm

hcl:z eyr:2033
ecl:lzr iyr:2029 cid:326 hgt:68cm byr:2026
pid:96742419

hcl:#a97842 ecl:brn
byr:1920
hgt:173cm iyr:2015
eyr:2024 pid:176967666

byr:1930 eyr:2025 pid:792694131
hgt:179cm ecl:brn
hcl:#a97842
iyr:2015

hgt:167cm byr:1960 eyr:2022 hcl:#efcc98
cid:87 ecl:blu iyr:2012
pid:431515059

hcl:#cfa07d
eyr:2023
hgt:188cm ecl:grn pid:081575957 byr:1938 iyr:2012

iyr:2010 byr:1973
cid:108
eyr:2026
pid:880191154 hcl:#888785 hgt:181cm
ecl:brn

eyr:2021 iyr:2010 byr:1942 hcl:#7d3b0c ecl:hzl pid:886241926 hgt:171cm

cid:53 byr:1993
pid:150cm eyr:2035
hcl:#888785 hgt:153cm ecl:#128262 iyr:2021

ecl:gry
pid:555911148
hcl:#733820 eyr:2022 hgt:154cm iyr:2012
byr:1935 cid:338

hcl:#b6652a
pid:833873846 iyr:2012
hgt:167cm eyr:2023 byr:1984

eyr:2024
ecl:blu byr:1955
hcl:#b6652a pid:517975316 iyr:2010 hgt:166cm

pid:133785752
ecl:blu
eyr:2024
byr:1973
iyr:2019 hcl:#fffffd
cid:236 hgt:173cm

cid:222
byr:2013 hcl:z eyr:2036 pid:7443967478 ecl:brn
iyr:2030 hgt:62cm

hgt:193cm cid:259
hcl:#18171d
ecl:grn
byr:1995 pid:727880050 eyr:2030 iyr:2010

hcl:#c0946f cid:275 eyr:1954 pid:772184635 ecl:#76add7 byr:2009 iyr:2018 hgt:151cm

ecl:#52ed0f eyr:2033 hcl:#18171d pid:475397948
byr:1946 iyr:2028 hgt:178cm

iyr:2012 hgt:152cm
eyr:2027 byr:1923 ecl:brn
hcl:#18171d pid:513722888 cid:171

iyr:2029
hgt:111 hcl:z ecl:#33e3bc eyr:1930
byr:1934 pid:94036732

hgt:154cm eyr:2024 hcl:#6b5442 iyr:2017
byr:1974
ecl:amb pid:470968353 cid:345

hgt:184cm hcl:#617375 eyr:2028
byr:1975 ecl:oth
iyr:2018 pid:735589126

cid:261
hcl:#cfa07d pid:213013397
hgt:187cm
ecl:gry iyr:2016

hcl:#623a2f
ecl:#34964b eyr:2009 pid:169cm byr:2028 hgt:169cm
iyr:2028

eyr:2029 iyr:2016
byr:1985
hgt:192cm hcl:#602927 cid:167
ecl:blu pid:620818510

eyr:2029
byr:1968
ecl:blu
hgt:183cm iyr:2011 pid:952376140 hcl:#efcc98

iyr:2020
byr:1981 pid:850136149 eyr:2028 hgt:159cm hcl:#7d3b0c
ecl:brn

ecl:brn pid:480452858 hgt:65in cid:340 eyr:2022
byr:1946
hcl:#602927 iyr:2015

hgt:172 hcl:z eyr:1958 iyr:1941 byr:2019 pid:389995951 ecl:dne

byr:2025 hcl:4c8dcd
hgt:177in
ecl:#55d635
cid:197 pid:91192572
iyr:1921 eyr:2038

iyr:2027 pid:154cm
hgt:185in byr:2012
eyr:2036 hcl:efd47d
ecl:#64f98d
cid:86

eyr:2029 pid:837224515 ecl:grn cid:231 hcl:#733820 iyr:2019
hgt:159cm
byr:1977

pid:974518338 byr:1964 hcl:#cfa07d ecl:grn eyr:2030
hgt:61in
iyr:2019

iyr:2019
hgt:192in cid:94
eyr:1922
byr:1925 hcl:z ecl:utc pid:#081266

eyr:2027 iyr:2019 cid:328 byr:1961 hcl:#6b5442 ecl:blu hgt:177cm pid:235426720

byr:1959
eyr:2025
pid:890034625 ecl:oth
hgt:62in cid:348 hcl:#733820

hgt:161cm iyr:2018 pid:916160791 ecl:grn
byr:1951 hcl:#44d03a eyr:2025

hgt:158cm byr:1942 iyr:2012 hcl:#602927
eyr:2026 ecl:gry pid:651231060

ecl:hzl cid:340 pid:086942161 byr:1986 hcl:#a97842 iyr:2018
eyr:2028
hgt:181cm

ecl:blu
pid:278922687 cid:238 iyr:2018 hgt:153cm eyr:2027
byr:1965
hcl:#733820

eyr:2023 cid:208 hgt:178cm hcl:#341e13 byr:1937 pid:290981079 iyr:2010 ecl:grn

hcl:#888785
ecl:amb
byr:1943 pid:559804716 eyr:2026 hgt:166cm
iyr:2019

pid:947831563
ecl:gry
byr:1960 hcl:#341e13
iyr:2016 hgt:173cm eyr:2029

ecl:blu iyr:2016 pid:724632073 hcl:#623a2f
eyr:2028 hgt:192cm byr:1958

byr:2021
eyr:2016 hcl:z iyr:1988 pid:65353943
ecl:#bb553b
hgt:125

hcl:#efcc98 byr:1963 pid:290433211 eyr:2023 ecl:hzl
hgt:172cm iyr:2013

iyr:2015 ecl:brn
byr:2023 hcl:#18171d
pid:325330679
hgt:190in eyr:2023

pid:745674970 hgt:160cm eyr:2021 byr:1925 ecl:gry hcl:#341e13 iyr:2015
cid:297

eyr:2021
pid:596411633
byr:1947 ecl:blu cid:191 hcl:#341e13 hgt:168cm iyr:2019

eyr:2030 pid:#902a6b iyr:1997 hcl:11f396 hgt:188cm byr:2025
ecl:dne

eyr:2025
byr:2006
hcl:#888785 ecl:hzl hgt:187cm
iyr:2012 pid:017702828

byr:1988 hcl:#18171d iyr:2019
pid:110591871
ecl:hzl
hgt:160cm
eyr:2029

ecl:brn
hcl:#c0946f iyr:2030 pid:264404022 byr:1984 hgt:59cm eyr:2040

pid:5973803069
hcl:#cfa07d ecl:grt
hgt:153cm eyr:2039 byr:1970
iyr:2025

hcl:#fffffd
iyr:2022 byr:2026
hgt:180 pid:82035145 eyr:2034 cid:118 ecl:utc

hgt:186cm eyr:2026
ecl:brn
iyr:2013 hcl:#8f4c9b pid:010260339 byr:1948

ecl:amb hcl:#18171d iyr:2020 pid:259501214 byr:1978 hgt:193cm
cid:263 eyr:2022

hgt:161cm iyr:2015 byr:2014 eyr:2003
pid:708958872 ecl:grt
hcl:f4a430

hgt:170cm eyr:2021 pid:911638274 cid:110 byr:1963 ecl:blu
iyr:2015 hcl:1eda64

ecl:oth byr:1949 hgt:174cm hcl:#18171d eyr:2022 iyr:2019
pid:305857230

ecl:gry hcl:#a97842 pid:971971076 byr:2002 iyr:2019
hgt:188cm
eyr:2022 cid:238

eyr:2027 pid:221315043 iyr:2010 hgt:159cm ecl:blu byr:1998 hcl:#6b5442

hcl:#888785
byr:1926 eyr:2022 pid:433807814 ecl:grn
iyr:2010
hgt:181cm

ecl:grn hgt:164cm byr:1951 hcl:#18171d cid:75 pid:845508281 eyr:2021 iyr:2017

pid:#f59bc7
eyr:1987 hgt:191cm hcl:z byr:2024
iyr:1985

hcl:#623a2f pid:497429747
hgt:189cm
byr:1987
eyr:2027 iyr:2012 cid:95 ecl:hzl

byr:2000
hgt:165cm
iyr:2017 pid:519443292 eyr:2029 cid:240 hcl:#a97842
ecl:blu

cid:67 pid:038299774
eyr:2023 iyr:2015 hgt:179cm byr:1941 hcl:#18171d ecl:amb

byr:2000
eyr:2025 ecl:oth iyr:2017
pid:334154607
hcl:#fffffd hgt:173cm

hcl:#888785 ecl:amb
cid:131 iyr:2018 byr:1996 eyr:2026
hgt:180cm pid:709543988

iyr:1988
pid:263277424
hcl:ee8912 byr:1942 ecl:gry eyr:2040 hgt:161cm

eyr:2020 byr:1966 iyr:2020 hgt:169cm pid:611918000
hcl:#7d3b0c ecl:hzl

hgt:164cm ecl:brn
iyr:2015 pid:192054454 hcl:#6b5442 byr:1987 eyr:2022

byr:1952
ecl:zzz
pid:215953654
eyr:2021 hcl:#efcc98 hgt:153cm iyr:2026

hgt:167cm
hcl:#b6652a pid:847614726
eyr:2022 ecl:gry byr:1990 iyr:2015

hgt:185cm ecl:oth iyr:2012
byr:1933
cid:250
pid:038674023
hcl:#c0946f

pid:613273980 hcl:#a97842
ecl:oth byr:1924 hgt:179cm
eyr:2027 iyr:1950

hcl:#cfa07d byr:2018 hgt:190cm pid:64530329
ecl:brn
iyr:2024

hcl:z hgt:70cm pid:18807747
cid:284 byr:2023
eyr:2035 ecl:#4a1501
iyr:1954

iyr:2016 hgt:152cm pid:886247173 byr:1940 hcl:#c0946f eyr:2027 ecl:oth cid:150

hgt:152cm hcl:#48cfdf eyr:2025 cid:277
ecl:oth pid:246230621 byr:1932
iyr:2020

ecl:amb pid:871180042
cid:117 hcl:#602927 iyr:2011 hgt:152cm
eyr:2030 byr:1999

eyr:2024 ecl:hzl hgt:171cm
byr:1934 pid:356408125 iyr:2019 hcl:#b6652a
cid:169

eyr:2023
hcl:#7d3b0c
byr:1934 hgt:67in ecl:oth pid:191785527
cid:117 iyr:2016

iyr:2029
hcl:#602927 eyr:2022 byr:1931 ecl:oth hgt:192cm
pid:231475143

ecl:grn iyr:2014 cid:250 hcl:#b6652a byr:1970 pid:675238417 hgt:162cm
eyr:2026

ecl:brn
hcl:#623a2f eyr:2021 pid:293293433 hgt:158 byr:1977 iyr:2019

ecl:oth hcl:#ceb3a1 pid:013111996 eyr:2023 hgt:180cm byr:1976 cid:224

hgt:61cm
eyr:2027 ecl:amb pid:181cm iyr:1932
byr:1974
hcl:#18171d

byr:1968 hgt:167cm
hcl:#a97842 eyr:2022 iyr:2018 ecl:hzl pid:940968694

iyr:1943
hgt:96
cid:229
hcl:z eyr:1990 byr:2007 pid:#25aa73
ecl:#74592e

hgt:182cm iyr:2018 ecl:hzl eyr:2029 byr:1946 pid:602345030
hcl:#ceb3a1

pid:750306036 eyr:2020 hgt:181in ecl:xry
iyr:2011 hcl:z byr:1971 cid:71

pid:183825747 iyr:2019 hcl:#6b5442
byr:1974
hgt:180cm eyr:2028
ecl:amb

ecl:brn cid:200 pid:576495225
byr:1924
hcl:#efcc98 eyr:2022 iyr:2017 hgt:185cm

iyr:2020 hgt:167cm byr:1965 ecl:brn hcl:#888785
eyr:2028 pid:752062953

byr:2026
hcl:z
eyr:2020
ecl:#b4ec74 pid:187cm iyr:1974
cid:326 hgt:150cm

byr:1996 pid:507323629
iyr:2015 cid:347 eyr:2026 hcl:#efcc98
ecl:amb hgt:157cm

byr:2017 pid:456780590 hcl:#888785 eyr:1966 ecl:amb iyr:2023 cid:187 hgt:62cm

ecl:hzl iyr:2015 hcl:#6b5442 hgt:152cm eyr:2028 byr:1982 pid:003269467

iyr:2017 eyr:2026
ecl:blu cid:70 hcl:#7d3b0c
byr:1966 pid:160330947 hgt:189cm

iyr:2010 ecl:amb
hgt:164cm eyr:2029 byr:1963
pid:596606374 hcl:#efcc98

hcl:#fffffd cid:277 pid:102326370 hgt:154cm eyr:2026 iyr:2012 byr:1968
ecl:hzl

ecl:oth pid:477189554 hcl:#6b5442 eyr:2022 byr:1948 hgt:74in cid:181
iyr:2016

hgt:169cm hcl:#d7bc93
cid:344 ecl:oth
pid:#09c55d iyr:2017
eyr:2030 byr:1928

hcl:5d02ff ecl:#ca7901 iyr:1959 byr:2006 eyr:2022
hgt:164in
pid:#d6cdfd

ecl:amb pid:5739190196 eyr:2021 hgt:157in hcl:#efcc98 byr:2018 iyr:2028

byr:1995 ecl:hzl
iyr:2017
hcl:#a97842 pid:917039291 eyr:2026 hgt:175cm

iyr:2017 pid:756519868
hcl:#623a2f
eyr:2028
hgt:158cm
ecl:amb byr:1957

iyr:2012
hgt:158cm
byr:2014 pid:973021666 hcl:f04766 eyr:2035 ecl:utc

ecl:blu
byr:1989 eyr:2022
pid:520765501
cid:200 hgt:193cm hcl:#a97842 iyr:2011

byr:1959
ecl:blu hcl:#733820 cid:284 hgt:162cm
eyr:2022 pid:751629408 iyr:2016

byr:1978 cid:301
ecl:oth hgt:67cm hcl:#888785
eyr:2040 iyr:2025 pid:26038514

iyr:2020 byr:1974 hgt:163cm ecl:blu hcl:#7d3b0c eyr:2028 cid:99

hcl:#a97842
hgt:186cm
ecl:grn byr:1969 pid:460360492 iyr:2011 eyr:2028

byr:2009
pid:489490924 eyr:2031
hcl:cb5351 ecl:#083a25 hgt:164cm

iyr:2019
hcl:3463cc ecl:amb pid:4089063078 eyr:2022 hgt:150cm
byr:2007

eyr:2028 hcl:#ceb3a1
hgt:191cm iyr:2019 pid:737842199 ecl:blu cid:268 byr:1925

pid:868397851
hcl:#efcc98 ecl:grn iyr:2017 eyr:2021 byr:1943
hgt:179cm

hcl:#623a2f byr:1987 eyr:2023 iyr:2019 hgt:152cm
pid:473569020
ecl:grn

pid:953968630
hgt:175cm
byr:1971 ecl:blu hcl:#623a2f iyr:2017 cid:336 eyr:2030

ecl:grt hgt:74cm byr:2022 eyr:2024 pid:39114027
iyr:2026 hcl:4b5675

pid:#492988
eyr:2032 hgt:63cm iyr:2006
ecl:#817211 byr:2019

pid:800367032 hcl:#341e13
ecl:#765111 iyr:2012 byr:2006 hgt:166cm cid:291 eyr:2027

eyr:2021 iyr:2012 pid:876581393 ecl:amb hcl:#866857
hgt:64in byr:1993

iyr:2017 byr:1996 ecl:hzl pid:038990744
eyr:2028
hgt:177cm
hcl:#c0946f

hcl:#4214a6
eyr:2021
iyr:2019 cid:72 byr:1939
ecl:hzl pid:783071912 hgt:187cm

eyr:2020 hgt:158cm
pid:274060737 cid:277
iyr:2015 hcl:#bf9b5e byr:1950 ecl:brn

byr:1921 hcl:#7d3b0c cid:329 hgt:155cm eyr:2030 pid:718399669 iyr:2011 ecl:brn

cid:147 eyr:2021 hgt:167cm iyr:2010 ecl:grn byr:1975 hcl:#6b5442
pid:285479783

hgt:187cm
byr:2004 eyr:2025 hcl:bb331b
pid:851189955 iyr:2016
ecl:amb

hcl:#94007d pid:361561551 byr:1927 eyr:2026 iyr:2020
ecl:gry hgt:158cm

byr:1993 pid:#24c4af iyr:2023 hgt:175cm eyr:2028
hcl:z ecl:hzl cid:308

byr:1985 hcl:#c0946f eyr:2034 hgt:172cm
cid:300 iyr:2013 ecl:gry pid:389455676

eyr:2030 iyr:2017 byr:1956 hgt:178cm
pid:864401853 hcl:#6b5442

pid:836559549
iyr:2011
hgt:167cm
ecl:amb hcl:#c0946f
eyr:2026 byr:1981

pid:111085991 iyr:2011
ecl:blu eyr:2026 cid:311
byr:1920 hgt:182cm hcl:#602927

ecl:oth pid:284436132
byr:1929 cid:121
eyr:2027
iyr:2010
hgt:75in
hcl:#6b5442

byr:1987
hcl:#7d3b0c iyr:2018 hgt:180cm
ecl:blu eyr:2029 pid:878348021

hgt:183cm cid:98
byr:1953 hcl:#866857 eyr:2021 iyr:2012 pid:158898193

eyr:2030 pid:039638764 ecl:hzl hgt:190cm byr:1926
cid:294 hcl:#b6652a iyr:2017"#;

// #endregion

// #region day 5
pub const DAY_FIVE: &str = r#"FBBFFBBLLL
FFBFFFBRLL
FFBBBBFRRL
FBFBBBBRLL
BFBBBBFLLR
FFFBBBBLRR
BFFFFFBLLL
BBFFFBFRRL
FFBFFFFLLR
BFFFBBBRRL
FBFBFFFLRL
FFFBBFBLRR
FBFBFBFLRR
FBBBBFBRRL
BFFBFFBRRR
FBBBFBBRLL
FBFFBFBRLR
BBFBFFFLRL
FFBFFFFRLR
FFBBFBFRRR
BFBBBFBLRR
FFBBFFFLRL
FBBBBFFRLR
FBBBBBBRLR
FFBFBFBLLL
BBFBFBBLLL
FFFFFBBRRL
FBFFBFBRRR
FFFBFFFRLL
BFBFBFFLLL
BFBFFBFLLL
FFFFBFFRRL
FBFFFFFRLR
FBBFFBBLLR
BFFFFFBLRL
BFBFFFBLLR
FBBBBBBLLL
BBFBFBFLLL
FFBFFBFLRR
BBFFFBBRLL
FFBFFFFLLL
FBBFFBFLLL
FFFBBBFRRR
BFBBFBFRLL
FBBFBFFRRL
FBFBBFBLRR
FFBBBBBRLR
FFBFBFFLLL
FBFFFBFRRL
BFFBBFFRLL
BFFBBFFLLR
BBFFBFFRRL
FBFBFBFLLR
BBFFBFBLRL
BFBFBFBLRR
FFBBFBBRRL
BFBBBFBRLR
FBFFFFBLLR
BFBFFBBLRR
BBFBFBFRRR
FFBFBBBRRL
BFBFBBFRLR
FBFBBBBRLR
BFFFFBBRRR
BBFFFBFLLR
FBFFFFBRRL
BBFFBFBLLL
BFFFBBBLLL
FFBFFFBLRR
BFBFBBBRRR
FBFFBFFRRR
BFFBFFBLLL
BFBBFFFLRL
BBFBFFBRLR
FBBFFFFLLL
FBFFFFBLLL
BBFFFFBRLR
BBFBFFBRRL
FBBFBFBRRR
FBFFBFFLRR
FFBBBFBLLR
FFFFFBFRRR
BFBBBBBLLR
BBFBFFFRLL
BFFBFBBLRR
BFFBBFBRLL
FBFFBFFRLR
FBFBBFFLLR
FFFFFBFRRL
FFBFFBFRLR
FBFBBBFLLL
FFBFBBFLLR
FBFFBBBRLR
FFFBFBFLLR
BFBBFBBLRL
BFFFFBFLLL
BFBFFBFLRL
BBFFBFBLLR
BFFFBFBRRL
BBFBFFBLLR
FFBBFFFRLL
FFFFFBBRLR
FFBFBFBRLL
FBBBBFBLRR
FFFBFFFRRL
FFFFBFFLRL
FFFFBFFRLR
BFFFFFFRRR
BBFFBFBRRL
BBFBFFFLRR
FFBBBFFLLR
FBFBFBBRLR
FBBFBBBLRR
BFFBBBFRLL
BFBBFBBLLL
FBFFFBBLLR
FBBBBFFRRL
BFBBFFFLLR
BFBBBFFLRL
FFBFFBBRLL
FBFBFFBRRR
FBBFBBBLRL
BFFFBFFLRL
BFFBFFBLRR
BFBFBBFLRL
FBFBBFFRLL
BFFFFBBRRL
FFBBBBBLRL
FBBFFBFRRL
BBFFBFBRLL
FBFFFFFLRL
FFFBFBFRLR
FFBBBBFLLR
FFFBBBFRLR
FFBBFFBRRR
FFFBFFBLRL
FBFBFBBLLL
BFFFFBBLRR
BBFFBBFLLR
FFFFFFBRLL
FBBBBBBLRR
FFFFBFBLLR
BBFFBBFRRL
BFBBBBBLRR
BFBBFBBLRR
BBFFFFBRRR
BFBBBFBLLR
BBFFBBBRRR
FBBFBFBRLL
FFFFFBBLRR
BBFBFBBLLR
FBFFFBFRLR
BBFFBFFLLR
BBFBFFFRLR
FFFFBFBLRL
FFBBBBBLRR
FBBBFFBLRR
FBBFBBFRLL
FBBFBFBRLR
FFBFBBBLRL
FBFFBBBLLL
FFFBBBBRRR
FFBFFBFLRL
BFFFBBFRLL
BFFFFBBLLR
BFFFFFBRRL
BFFBFBBRRL
FFBFFFBRRL
BFFBFFFLRL
BFFBBBFLLL
FFBBFFFLRR
FBBBFFBRLL
FFBBFFFRLR
BFBFBFFRLL
FFFFBBFRRR
BFBFBBFRRL
FBFBBBFLRL
FFFBFBBRLL
FFBBBFFLLL
BBFFFBBLRL
FBBBFFBRRR
BFBFBFBLLL
FFBFFBBRLR
FFBFBBBRLR
FFFFFFBRLR
BFFBBFFLRL
FFBFFBFRRR
FFBFBFFRLL
FBFBFFBLLR
FBFBFBBRRR
FFFBBFBRRR
FBBBBFBLLR
FFFFBFBRLR
FFFBBBFRLL
FBFFBFBLLL
FBFBFFFLRR
FFFBBFFLLL
BFBFFFFLLL
BBFFFBFLLL
FFFFBFBRRL
FFBBBFBRRL
BFBFFBBRLR
FFFBBFFLRL
FFFBFBBLLR
FFBBBFFRLR
FBBBBFBLRL
BFFFBBFLLL
BFFBFBFLLR
FBFFBBFLRR
BFFBBBBLLR
FBBBFFFRRR
FBFFBFBLRL
BFFFFFFRLR
FBBBBFBRRR
FFBBBBFLRL
BBFBFFBLRL
BFBFBBBLRR
FFFFFBFLLR
FBBFFBFLLR
BFFFFBBRLL
FFBFBBBLRR
FFFFBFFLLR
BBFFFFFRLR
FBBBBFFRLL
BFFFBFBLRR
FBFBBBBLLR
BFFBFFBRLL
BFBFBFFLRR
BFFFFBFLRR
FBBFFBFLRR
BFFBBFBLLR
FBBFFFBLRL
FBFBFBBRRL
FBFFBFFRRL
BFBBFBFLLR
FFFFBFFLRR
BFBFFFBLRL
BFFBBBBRRR
FFFFBBBLRR
FFFBBFBLLL
FFBBFBFLLR
BFBBFBFLLL
FBBFBFFLLR
FFFBBFFRRR
FFBBBBBRRR
FFFBFFFRLR
BFFBFBFLRR
FBFFBFBRLL
FFFFBFFLLL
FFBBBFFRLL
FBFBFFBLLL
FBFBBBFLLR
BFBBBFFRLL
BFBBBBBRRL
FFFFFFBRRL
FBBFBBFLRL
BFFBFBBRLL
FFFFFBFLRL
BFBFFFBLRR
BBFFFBFLRR
BFBFBBBLRL
FBFFFFBRLR
BFBFBBBLLL
BFFFFFFRRL
FBBFFFBRRR
FBBFBFFLRL
BBFFFFFLRL
BFFFFBBLRL
FBFBBFFRRR
FBBFFFFLRR
FFFBFFFLRR
FBFBBFFRRL
BFFBFFBRRL
BFFBFBFRRL
FFBBFBBLLR
FBFFBFFRLL
BFBFFBBLLL
FFBBFFBLLL
BBFFFBBRRL
BFFFFFFLLL
FFBBBBFRLL
BFBBBBFRRR
BFFBBBBRLR
FBFFBBFRLL
BBFBFFBLRR
FBFFBBBLRR
BBFBFFFRRL
BFBFFFFLRR
BFBFFBBRRR
FBBFFFBLLL
BFFBBBFRLR
BFFFFBFRLL
FBFBBBBRRL
BBFFBBFLRL
FBBBBFFLLR
BFBBFFBRRR
FBBFFBBLRR
FBBBBBBRRL
BBFFFBBLLR
BFBBBBFLRL
BFFBFFFRLR
FFFFBFBRRR
FFFFBFBRLL
BFFFFFFRLL
BFFFBBBRLL
FFFFFBBLLL
FBFFFBFLRR
FBFBFBFLLL
BFBBFFBRLL
BFFFBBFRLR
BFBBFFFRLL
FFFBBFBLRL
BBFFFFFLRR
BBFFFBFRLR
BFBBBBFLRR
FBFFFBBLLL
BBFFBFBRRR
FBFBFBBLRR
FBBFBFFRLR
FBFBBBBLLL
FFFBBFBLLR
FFFFBBBRRL
FFBBBBFRLR
FFFBFFBRRL
BFBBFBBLLR
FFBFFFFLRL
BFFBFBFRLR
FFBBFBBLLL
BFFBBBFLRL
FFFFFBFRLL
BFFBFFBRLR
BFFFBFFRRL
BFBFBBBRLL
BFFFFFBLRR
FBBFFBBRLL
FFFFBBBLRL
BBFFFFBLRR
BFBFBBBLLR
BFFFBFFRLR
BFBBFBFRRR
BFFBBBFLRR
FFBFBBFRRR
FBBBFBFRLR
BFBFFBBRLL
BBFFBFFRLL
BFFFBFBLLR
FBFBBFFLRL
BFFFFBBLLL
BFBBFBBRRL
BFFFBFFLLL
BFBBBFBLLL
FFBBFBFLRR
FFBBFBFRLR
BFBBBFFRRR
BBFBFFFLLL
BFFFFFBRLL
FBBFBBFRRR
BFFFBBFLRR
BFFBBBFRRL
FFBFFFFRRL
FBFFBFBLRR
FFBFBFBRLR
BBFFBFFLRR
BFFBBFFLLL
FFBBBFBRRR
BBFFBFFRRR
FBBFFFFRRR
FBBBBFFLRR
FBBFBBBRRR
FFBFBFBLRL
FBBFFFBLRR
FBFFBFFLRL
BFBBFFFRRR
FBFBBBFRLR
BFFFFFBRRR
FBFBFFFRRR
BBFFFBFLRL
FFBFBFBLLR
FBFFFBBRRL
FBBBFFBLLL
BFBFBFFRRR
FBFFBBBRLL
FFBFFBBLLR
FFBBFBBRLL
BFBFBBFRLL
BFBBFFBLLL
BFFBBBFRRR
BFBFBFBRRL
BFFFFFFLLR
BFFFBBBRRR
FBBBFBFRRR
FFFBFFBRLL
BFFFFBFLRL
FBFBBFFRLR
FBBBBBFLRL
FFFBBBFRRL
BFBFFFFLLR
FBFFFBBLRR
FBBFBBBRRL
FFFBBBFLRR
BFBFFBFRRL
BFFFBBFRRL
FBBBFFBLLR
FFBBFBBLRL
BBFBFBFRLR
FFBFFBFLLL
FFFFBFBLLL
BBFFFFBRLL
FBFFFBBRLR
FFBBBFFRRR
FFFBFFFRRR
FFBFBBBLLR
FFBFFFBRLR
FFBBBFBLRR
BFFBBBBLRL
BFFFFBFRRR
BBFBFBFRLL
FBFBBBFLRR
FFFFFBFLLL
BFBBBFFLLR
BFFBFBFRRR
BFBBFFBLRR
BBFFFBBRLR
FBBBBBBLRL
BFFBBBBRRL
FFBFBFFRLR
BFFFFFBLLR
BBFFFFBRRL
FFFBBFFLRR
BFFFFBFRRL
BBFFBFFLRL
BBFBFBFLRR
FBFBBBBLRR
BFFBBFBRRR
FFFBBBBRLL
FBFBBBFRRL
FFFFFFBRRR
BFFFFFFLRL
FFBBFFFLLR
FBBFBFBLLL
FBBFBBFRLR
BFFBFFBLLR
FBFBBFBRLL
FBFBFBFLRL
FFBFFFBLLR
FBBFFBBLRL
FFFBBBBLLR
BFFBBBBLRR
BFFBFBBLLR
FFBBFFBRLR
FBBBBFFLLL
BFFFFFBRLR
BFBBBBFRLL
FFFBBFFRLR
BFFFBFBLLL
BBFFFFFRRL
FBFFFFBRLL
BFBFBFBLRL
FBBFBBFLRR
FFBFBBFLRR
FFBFBFFLRR
FFBFFFFRLL
BFBFBFBRRR
FBBBBFFLRL
FFBFFFBLLL
BFBBBFFRLR
FBFBFFBRLL
FBBBBBFRRL
FBFFBFFLLL
BBFFBBBLLR
BFFFBFFRRR
BBFFBBBRLL
BBFFFBBLRR
FBBFBBBRLR
BFBFBBFRRR
FBFFBBFLRL
FFFFBBFLRR
BFFFBFBRLL
BBFFFFFRLL
FFFBFBFRRR
BFBBBBFLLL
FFBBBFBRLR
FFFFBBBRRR
BFBFFBFRLR
FBFFBBFLLL
FFBFFBBLRR
FBFBFFBLRR
FFBFFBBRRL
FFFBBBBRRL
BFFBFBBLRL
BBFFBBBLRR
FFFBBFBRLL
FFFBFBBLRL
BFBFFFBRRL
FBBFFFFLLR
BBFBFFBLLL
BFBFBBFLLR
BFFBFBBRRR
FBFFFFFLLR
FFBBBFFLRL
FBFFBBFRLR
BFFFBFBLRL
FBFBBBFRRR
FFBFFBFRRL
FBFBBFBRLR
BFBFBFFRLR
FBFFBBBRRR
BBFBFFFLLR
FBBFBBBLLL
FFBBBFBLLL
FFBFFFFRRR
BFFBFBFLLL
BFFBBBFLLR
FBBBBBBRLL
FBBFFBFRLL
FFFBFFBLLR
FFBBFFBLRL
BFBBFBFLRR
BFBFFFBLLL
FBFFFBBRRR
FFBFBFFLLR
FBFFBBBRRL
FBBBBFBLLL
FFFFFBBLLR
FBBFBFBLRL
BFBBFBBRLL
FFFFBFBLRR
BBFFBBFLLL
FBBBFBFRLL
FBFBBFBLLL
BBFBFBFLLR
BFBFBFBRLR
FFFFFBFRLR
FFFFFBBRLL
FBFBFBFRRR
FBBBFBFLRL
BBFFBBBLRL
FFBBBBBRLL
FBBBFBBRRR
BFFFBBBLLR
BFBBBBBRLL
FFFFFBBLRL
BBFFBFFRLR
BFBFBFBRLL
FFFFBBFLLL
BFBFBFFRRL
FFBBFFBLLR
BFBBFFFLLL
BFBBFFBRLR
FBFBFFBRLR
FBFFFBFRLL
FBBFBFFLRR
FBBFFBFRRR
BFBFFFFRLL
BFFFBBBLRL
FBFBBBBRRR
BBFFBBFLRR
FBBBBFBRLL
BBFFBBBRRL
BFBBBFFLLL
FBFFFFBLRL
FBFFFFFRRL
BFBBBFFLRR
FBBFBFBLRR
FBFBFBBLRL
FFBFBFFRRL
BFFBBFBRLR
FFBFFBBLLL
FBBBFBBLLL
FBBBFBBRRL
FBBBFBBLLR
FBFFFBFLLL
FBBBFFBRLR
FBBFBBBRLL
FBBFFFBLLR
FFFBFBBRRR
FBBFFFFLRL
BFFFFBFRLR
FBBBFFFLLL
FFBBFBFLLL
BBFFFFFRRR
BBFFFBFRRR
FFBFFBBLRL
FBBBFFBLRL
FFBBFBFRRL
FBFFFBFLRL
FBBBFFFLRL
FFFFBBBRLR
FFBBFBBRLR
FBFBFFBRRL
BFBFFBFRRR
BFBFBFFLRL
BFFBFFFLLL
FBBFBFBRRL
FFBBBFBLRL
BFFFFBFLLR
FBBBFBFLLL
FBFFFBBLRL
FBBFFFBRRL
FBBBBBFLLR
FBBBFBFRRL
FFFBBFBRRL
FBBBFFBRRL
FBFBFBBRLL
BFBBBBBLLL
FBFBFFFLLR
FFFBFFBRLR
BBFFBFBLRR
BFFFBFFLLR
FBBFBBFRRL
FFBBBBBLLL
FBBFBFFRLL
FFFBBBFLRL
FFBBBBFLRR
FBFFFFFRLL
BBFFFFBLLR
FFFFBBFRRL
FFBBBBFRRR
FBBBFBBLRL
BFFBBFBLRL
BFBBFFFRLR
FFFBFBBRRL
BBFBFBFRRL
FFFBBBFLLR
BFFFFBBRLR
BFFBFFFLLR
BBFBFFFRRR
FFFFBBFRLR
BFBBBFBRRR
FBFBBBBLRL
FFFFBBFLLR
FBFBBBFRLL
BFBBFBBRLR
FFBBFFFRRR
FFBBBFBRLL
FBBFFBFLRL
BFBFFFBRRR
BFFFBBBRLR
FFFFBFFRRR
FFBBFFBLRR
FBBFBFBLLR
FFFBBBFLLL
FBBBBBFLLL
FFFFBBBLLL
FFFBFFBLLL
BFBBBBBRLR
BFBFBBFLRR
FBFBBFBRRR
BFBBFFFLRR
FFBBFFFLLL
BFFFBBFLLR
FFBBFBFRLL
FBBBFFFRLL
FFFBBFBRLR
BFBFFBFLRR
FBFBBFBLLR
FBFFFFBRRR
FFFBFBBLLL
FBFBFFFRRL
FFFBFBFLRR
FFBBBBBRRL
FFFBFFFLLR
BFFFBFFRLL
BFBBBBBLRL
FBBBFBFLRR
FFFFBBBLLR
BFFFBFFLRR
FBBBBBFLRR
FBBBBBFRLR
BFFBFFFRRR
FFBFBBFLRL
FFBFBBFRRL
FFBFFBBRRR
FFFFBBFRLL
BBFBFBFLRL
FBFBBFFLLL
BBFFFFBLLL
FBBBBFFRRR
BFFBFBFRLL
BFBBFBFRRL
FFBFBBFLLL
FBFFFBFRRR
FFBFFBFLLR
BFBBFFBRRL
FBBFFBBRRR
FFBFBBBRLL
FBBBBBBLLR
FBFBBFBRRL
BFFBFFFRLL
BBFFBBFRRR
FBBBBBFRLL
FBBFFFFRLR
FBBFFBBRLR
BFBFBFBLLR
FBFFBFBRRL
FFFBBFFRRL
FFBFBBFRLR
BFFBFBBRLR
BFFFBFBRRR
BFBBBFFRRL
FBBBFBBLRR
FFFBFBFRLL
FFBBFFBRLL
FFFBBBBRLR
FBBBFBFLLR
BFBFBBBRRL
FBBBFFFRLR
FBFFFFFLRR
FFBFBFFRRR
FFFBFBBLRR
FBBFBFFLLL
FFBFFBFRLL
FFBBBFFLRR
FFBFBFBRRR
FFFBBBBLRL
FFBFBFFLRL
FBFBBFBLRL
FFFBFBFLRL
BFBFFBFLLR
BBFFFBFRLL
FFBBBBBLLR
BBFFFBBLLL
BBFBFFBRRR
BFBFFFBRLL
FFFBFBFRRL
FFBFFFFLRR
BBFFFFFLLL
FBBBFFFLRR
BFFFBBFRRR
FBBFBBBLLR
BFBFFFFRRL
FFBBFFBRRL
BFBFFBBLRL
BBFFFBBRRR
FFFBBFFRLL
BFFBBFBRRL
BFFFFFFLRR
BFBBBBFRLR
FBBFFBFRLR
BFFBBFFRRL
BFFBFFFRRL
FFFFBBBRLL
FBFBFFFRLR
BBFFBBFRLR
BBFFBBFRLL
BFFFBBFLRL
FBFFBFFLLR
BFBBFBFLRL
FBFBFBFRLL
BFBFBBBRLR
BFBFFBBLLR
FBBBBFBRLR
FBFFFFBLRR
FBFFBBFRRR
BBFFBFFLLL
FFFBFBFLLL
BFFBBFFRRR
FBBFBFFRRR
BBFFBBBRLR
FFFBFFBLRR
FBFBFBFRLR
BFBBBBBRRR
BFFBFBFLRL
BBFFBBBLLL
BBFFFFFLLR
BFBFFBFRLL
BFBBBBFRRL
BFBBFFBLRL
BFFBBFBLLL
BFFBBBBLLL
BFBFBBFLLL
FBFBFFFRLL
FBFBFBFRRL
BFFBFBBLLL
FFBBFBBRRR
FFFBFFBRRR
FBFFFBFLLR
FBFFFFFRRR
FBBFBBFLLL
FFFBFBBRLR
FBFFFFFLLL
FBBFFFFRRL
BFBBFFBLLR
BBFBFBBLRL
FBBFBBFLLR
BFBFFFFLRL
FFBBFBBLRR
BFBBBFBLRL
BFFBBFFRLR
FFFFBFFRLL
BFFBBFFLRR
FBBBFBBRLR
FBFBFFFLLL
FFBBBFFRRL
BFBBFBFRLR
FBFFFBBRLL
FFFBFFFLRL
BFBBBFBRRL
FBBFFFBRLR
FFBBBBFLLL
FBBBFFFLLR
FBFFBBFLLR
FFBFBBFRLL
BBFFBFBRLR
FBFFBBBLLR
FFBFBFBRRL
BFFBFFBLRL
FBFFBFBLLR
FFBFBFBLRR
FFBBFBFLRL
FFBBFFFRRL
BFBFBFFLLR
FBBBBBBRRR
BFBFFFFRLR
BFFBBBBRLL
FFFFFBBRRR
FFBFFFBRRR
FFFBFFFLLL
BFBFFFFRRR
FBBBBBFRRR
FBBBFFFRRL
FBBFFFFRLL
FFFFFBFLRR
FFBFBBBLLL
BFBFFBBRRL
FFFFBBFLRL
BFBBBFBRLL
FFBFFFBLRL
FBBFFFBRLL
BFFBBFBLRR
FFFBBFFLLR
FBBFFBBRRL
FFBFBBBRRR
FBFBFFBLRL
BFBBFBBRRR
BBFFFFBLRL
FBFBFBBLLR
BFFFBBBLRR
BFBFFFBRLR
FFFBBBBLLL
BFFBFFFLRR
BFBBFFFRRL
FBFFBBBLRL
FBFBBFFLRR
FBFFBBFRRL
BBFBFFBRLL"#;
// #endregion

// #region day 6
pub const DAY_SIX: &str = r#"xtpmjeuayzkflcdo
zdaeyxlpftkmojc

ifnkhzalvprjtyus
lmihuzrytjns
uilmsotygbnhrzj
uwslqctjnirzxhy

lhuydwqxaempbisrnfcjtvz
rvtcegbhljadfpxzsiuyqwnm
rzyvtefjdsplucmqbinahxw

whlxgytuoaidpfrsbvmkjqez
iwfghlytovbpdemzanjq
mpejclwhqziatybogvdf

qsmuehbvfiwlkx
gtmdsbrqfwhk

lagwjpd
dpglwja
qgdawplj

faqjpheg
acnfgtkzj
msjdfox

gsqhdrcvyxbp
yhvgpsqxcbdr
drqvgxhyspcb
cgqrypdhbxvs
hbdxgyscjqprv

bgahiyplewc
qaprike
aojpszixe
airpksoe
mpzaiek

ihyudtfbjqnxop
poyiuqjdlhmtxb

gw
bg
g

zxjgytpsnol
tfuexgspvdkb

zmisajphc
paifmh
ipnhdma
dbacphmi
omhpwailkyv

ind
jd
olxtyd
daylkbtox

hjmdrgevkztwfsx
gsm
gsapioqm
ngoaqms
omlqgys

qfxwksiyluhvgnaotd
hqnrauzcdifeoglsyt
dgnfqsaytlihmou

brmeuyxlagqsz
zfkiyxnloagjeqwh

whqgiduclktrebyfp
kcirhydwebpfqgtul
tfclebygmdnuriqkwhp
ytfipklgubqwdrehc

yubh
dhu

sncvemkuwhi
wizhecnxlvbkms
ykvshenmicwu

dlfbyscp
oruaxtg
rowqu
gze
i

hgtejbrcxdolv
lbmhepjxsvdrqcgo
jelrbohcdxvngf

wrqxeocjni
lnirqcoxjwk

xmshq
mhqxs
shxqm
mxsqh
hmqsx

rbwdsfcgute
oa
lqizp
kxvjyz

gxikepca
cigaekx
xkgacei
kceigxa
ecxikag

ltpzifo
ozitrslp

tixaqjd
qtw
ctrlqfznmg
wqtbk

uyi
nuiy
iyu

nwid
zkmf

oqerdmwk
dmorkew
meorkwd

gjvkzlbrxiyotpfqedu
xtqfgluozkvimrydpjeb
rzbfjotiuqkpgxvldey
ixkptzqdouvbgrlyefj
pvxqydniuegjwbartfkolz

rkhexctuiopsjlzw
thxscuzrdlpjekiwo
sjihrzuwkcoelxtp
rtjcusoipxewklhz
sokecrwuzpxltihj

sdqlwfuo
uqsdlwfo
slqdufow
oafswqdlu
wfquosld

qksticygoejwnvhf
lnkipymvstjqzgfcaorxw
fwqgcyovjteisdkn

fjcywhvqdirxblzkaegpumnsto
qjigrmzsfexaholvnutcbywpdk

qgbaphtxuf
khpug
ipuzshgky
pmgshu

fjpvbdtlcugxizhwsrokmaqy
fgznwthadpvkxrsbqcyom

oqet
toeq

mrfhtwenxykibpsoj
qlucvsmroghwzpbtadx

aspnfmxi
ivlgfmy

mznti
tnzmi
itnmz

hlmx
xhlmp
lmxh
lmxh
lxguhm

fhmcgwtsj
jtomcfzwklh
wfybihtjcm

pkwqvtgzmjsenudxifbc
gwdfhjicypnmltroeqavk

euklmi
iumerlh
uimkle
iunzmelcv

fkumqizsynacr
nqzxycskufaih
asfpckruyinqz

ibpehrotv
pehovrntbi
voehitbpqr

cvqsfartyxkw
snvwkazcfqritxy
frayckvwtqxus

preot
ezj
eju
uezcd

qvwmxcnhpzjyadkbisotu
hnoxjqyudkgapbiwcmtsz
qklozhpyvrcxwjsitudbnma

gmspxjbwnray
wypmxsgjr
drlpsjgmxy
xykgesrjpvhm
xjmsypgro

ncexy
nycex
xneyc

ej
kre
lesxzt
gphfre
e

oribjg
bu
kvq

vnfpsmwc

qd
qd
dq
dq

rsc
rpaszc
ecrsom

bofltugqahsjyzew
osahjqwtlzevb
ojxtvlnqhrswzeab
bajepotzqhvwsl

elyhw
lwytrh
ljwyh

cboxndurmi
hjzslvq

wiozfcehptsxjrqmlbky
fzgilbuhktasdmvo
zileftmnhpcskbo

qeabixvh
awgkoqejh

ncrshioxklvumztdqfewpbg
vekofqmgclxzsdbuhwipnt
nlzxouwtmkfsgqcdbehipv
ibpwlqnmzfhkosuvtcxgd
wphavfyloszidgjbmxqtukcn

zat
cilkbqxr
nzwsgy

z
d
d
d

v
m
v
v

fjeayswno
jefwuxnoc

iugjno
jugmenoi
ncuiogj
ojfguin
jiugno

mcr
cmuq
cm
cm

rvwgteoxlnqpu
onubqretvwxgpl
agitsumplxnzrkoeqvw
oleurpvnxcgtqdw

ajedfrwstqn
pmnw
cwn
gcnbw
uvnw

l
zqxij
prlu
ek
l

rpht
whk
okhcbfw
whomk

hlkyo
kp

dnl
ldn
dnl
nld

bgor
dlou
mdvn
cyzkw
vjentu

trxvafpwesciqombhlkgjndy
oamdvntbsgcfphlkrwzyeqjx
kovjselaqpfnmyrxbzcthdguw

van
ltexgv
dfcjoskzyriw

jusk
a
lxg
hvnwf
exus

fawkveqgsrbjzlncxyhot
rklfzsvbwhxetjcaoqg
rakjszvgtxbfhwqodlec
sxpzekgwjdocbqtrvlhaf
fzrlsavojxmcbktgeuhwq

eitrvamwxfckzgqdoljyhnsb
yiptlazkxdwnmohujsecr

xoyvn
noyx
fotnyax
noxy
noyqx

nfvogds
fnsgvy
sqjvgfn

mjcekdpwxr
xkuphcmrg

by
by
bytda
yb

mefouswzphqrdvn
cdbyjmintfpgxlk

lemysjcrknvfpzwoxhqaig
yodnmfwpkjurscbtezqv

zw
saqv
kbermi
lwvo

oi
io

zdyk
ckdy
jyu

eldnugikcpxvwm
gcxdpukeln
ucnkdhxegljp
rdkegcnupxl

uxokmrefyw
fyrwucohsnxem
rmbfeyuwxjoth
lquwdiyaropmexf
uweyxofmr

jhoufbsc
juodbchs
bcuwljhos
sbujhcfoz
ubosjdhce

nuyhefxwclzsobmjvrgt
uvyswnqfgxemobrclzjth
cydtgefhwnisjorumvlzbx
nbcghyjtsvkwzroefuxmla
leitjyscbuzrxmvowhgfnd

c
sc

fc
fc
cf
fc
cf

iupatydhxkvmbjrl
hbptrxjliaymud
udxeyjrkbphamilt
yuspxdmtiajlqrbh

hjgkstroxypi
melkwdhiqbypjxc
isxfgjyohpk

jcshpztfgoynimedwxrqba
izjtbypodrxfwgcqahmsen
mqwgbpzhijdonfryxetcas
nfywrtdoejsxzaipghbmcq
mgrazowyxfecspntdhqjbi

pzctfawmn
hzinstwv
zntipwh
wjvtzfubnm
zknyqrltgxow

bxtvgjkreoupqzayh
jezpkhrovugaqtxby
htxbojraqzpuvnykegs

jakdyhimbtwxlncsg
bjnkcmatgwyxidlhs
mycwkdighnablsxjt

smokt
eunfogxytclshm

r
r

pdasnreog
uyljkar

whriktcn
ujewkhdfrs
mxdfrhgkw
fwhkrp

h
hw
h

xmvjkhoebzqs
osvxbzheqmkj
szekvxjobmhq
hezjmxqbsokv
voekqxjbshzm

kzysiheptuxgob
zhyxepbwsugikot
goxkzsebyphitu
hsuxotzbpikgey
sbgtoeyizkhupdx

gdrqitano
rqdiaotgn
daogqtirn
atgoqdrni

dluy
fm
dry

nyseo
jenyo
ovyne
nyeox

zawiflvhpjgomecdrk
iloejmrghpkwfzcadv
hfwejokzirldvpcgma
dihcwlmgoefkrazjvp
ephokwjzicdflvgmra

gyubiopxhflqkrvcwtszmj
cfmwxogquzyhtsbvpkjil
qxmgoflbyjzwucphksitv
fjzixwusycvbhgqomtpkl
hgmtcbszywpvukjqoxifl

jkvthqorywsux
qwukhroxytjvs
wktonyqjvuxi

esmpgqur
rspqm
sqmpr
sprqm
rspmq

e
e

vkamztjcdqhroblnyw
tmoqwyzharjvbdnlc
ojynamchbztdlqvsrw
whjrocnadetmlqyzbv
jlmqcyxbdvzhoawrntu

yljxnwtpirgqe
rtwyxij
iywjtxr
yrjwzxit

qovbwdzhlif
dbuvhifyzqw

yaeigdrovcqmnbxfw
vpwnimfaexocyqgdr
nivqcoemawydxrgf
wmxfiornakcgvtuqdhye

ctywzurhk
rtyzwuchk
wrycuhmtkz

z
vz
zma

gcrivmshxknuwot
hqgwunvkitsorfx

ulmkvwz
qlkfmczovysuw
dumwaljrvzk
nuwzkmilv
wluntvmkzx

ydw
qwstbd
jhd
idxfn

defbuaptricmwk
nylgehpvsu

zkelcmpdatj
tacjldxkem

hjvmubrtyzpwnskexflod
bdgfcwkmtzxrqoupnljhiva

eubykwhqdxivgmfpzjcnors
qcrexkjnmibzhsvfoydgpuw
rejovgknxuwymhfcpidzqsb
qgzkjwiepyhosmvrxdubncf

jfqsrwzg
csyuoabhvikmd
nejsz
eqxs
jsgwl

vkfjpozcxm
qsephmdo
aormp
eqponhym

tdhaizeyqfcvm
yvzgctnph
vkojgzbhtypcs
wbcyhxnvtzs
hztvcyp

yehaqkotbnzlj
qohzbyjltnieak
lhkyqzjbeotans

msvofdluxnpht
otmnflzvds
slmtnzodvf
tflsjivnomd
tdonmsfvl

hyqoneaujczkmwlx
ynkheqzwfaolc
nhkzqalyewco
wznkfcaeyohlq
yechalzpoqsknw

mnebgrhqjlkpod
ecxgvlwdfzhiabstj

rgjhy
dqgh

inel
heniy
ndei
edin

yomqetfwnpixh
lqsvgkdcizjyfb

qksm
qms
smq
sqm

xpigmrvtc
mdcxgrip
rmzgxcip
xigmcrp
ugflmyircxbp

uaibcnygjvroqxtmkz
aunrkmxqbzjgwyeod
qxmogsazkujrcnby
qojxaybszlrcuknfmg

recwyhnda
nchryaw

gi
ig

tzq
bkj

rz
r
qr

bkdftxjvrioq
xkodfbivrtq
biqtovdxkfr

zxybwjcqvkmt
swaeftrnhd

vk
vkw
kv
vk

jmpli
jpgim
jmoipw

uliwvnpmk
lwvinmk
mvnwleik

h
h
h

aizpyuhrsm
rmiuzdbhy
iruhmtzy

fsc
c
c
zuag

kinylrdqtf
yilkfrdwxqno
rynlkfdiqp

ruskgfcylowvmjiahezb
gsicyhqnxfpjrlt

awgx
gawx
xagw
axgw
awxg

iu
u
u
u
xsu

xr
d

z
z
z
z

epdf
d
d

vdlrbif
cjsotfdwexh
qdrfa

qwfkdupgvszibt
smpzetkihb

slvfkubejdcowqr
ruwedaqcvolbfpsigj
jblrauwseokvdcf
vxhntcrfzubwdosjel
oslwdmvcjebfru

wnkaszlfjgupvbxmhocrt
nbohjymstlfrvxckgu

ampvjczwtn
rchznpfjvaw
zpcjnwtva

slqon
ados
losn

mvuy
uxogtvmcqp

ql
yopbdme
swhkg
iwlcq

pdfuvyqokgxsbzhacirjlm
pgdakovmurfcyisqbljzhx
ypifcvlojsagrxdqzubhkm
pjyfozrlcxhvbgdiukasqm
lphfyuskrqioacxgdvjzbm

vjq
oqjv
jqv
jqvb
qjv

u
u
u
u

mife
eaxz

wukfghacoqi
kjcphli

elmf
xogzbpwcmds

udrqzjglkefhcimptvanyx
fnczjgaekihwyuvldqsrxtmp
banxpouvchgikqemtdfzjrl

bdzvtqygucirm
ibyzgcvtumd

poxgflmjweuqhktcazb
ugzfcjeqkwnbmoatph
qyjtwbfgouzhcpamek

o
ov
o
o
o

actbd
czose
zcpys

btwhfmpi
umhtbfiw
imfwhbt

afolhyqvxeg
qohvcregl
rvhclosqge
hcoeqglv

ozqibfhewgpndavmly
wnlzepyoivkxqfbgamdh
oiztbvmlfnaqyhdgewp
egvlmrwpqdiyozanfbh
zkpnwmfydlheaqivbogu

jnl
njl
ljn

a
qtn
ntzr

eohmibyp
yokm
owumy
kojmy
nomay

ebx
dapbe
dbex
erb

btrkcljqxdm
dtbrck
dkrucbt
tbrcdk

igvhpejrmxkcofznuyb
oygpxuvbjzrkcnhiem

mj
my
mj
m
dmw

ljxhmbda
ahdjxblm

umqytpknxl
uondmpyblqt
abndmyuphtq
nwgmiujyszqeptf
nbprlqvycutm

ebxpwykcgiuhv
yxgwehcpvbkui

fpalrvgksjn
pxchrbnkfajvdg

dsunk
gtwuse
xvlbcmhufi
sozgqpue

trn
e
e
ey
a

vwdazum
mauvgzw
fuzwpvmiaq

ihowsqrbxdetuvcgajf
sdojqeaiwygfubxrkv

abzi
fbajiu
baicf
rsbyeliad
azib

nscxriyzoapflkwj
ptrnscyfjdxazoiwlk
zxraykosfjlpcinw
zoplikjyxwcvnsgraf
zcspbroxlikfawynj

vqefizng
ix
dcs
apoujwtylk
nbg

thvu
utvh
vuth

cb
qb
znisjautvpo
e
rmk

yps
ps
sp
pds

htldayejcm
qgwmjit
tmsj

qw
qnzpei
tjq

gmri
rimg
gqeimrt

nvoqdje
nvojeqd
nojevqd
njeoqdv
qndovje

mdhvwfnarbpksjt
dftsmnvrjpkwbha
dmtnhbrypzvjsfakw

xnctqbezompvi
tcivzqxjemo
trmizegvcqxo
yeozqitcjxmv

rpvezm
pmre
reopimut
vmpezr
hpermz

dshourgilv
hovgluedis
ugvsohldi
dshvuiglo

xufdhyst
tuh
htkbuwv

uyhmkzgprft
lptqenbofxwk

klhnprixqc
ptyfsz

gbdilsxjvyuceqnw
vguqmicyjlnoxstkzdb

ajbvdz
jbdavz

qwoaltfyjecup
bikznrxehdsvgl

il
il
lai

hnxbpeudogywqjtzf
xrfwsypilokbghmn

lnprticjufgk
tkgifnrcpluj
jkptrinlcgfu
ntruckifpjgl
kjutlfcrpnig

pjxn
d

cehjgirskzdfupnayqbtx
tfcrbgqoxipjnshyu
ircpljyuqgbnthxmsf

elknv
kjle

rlfimhpjtxcbgqysnkev
fxmpjhcvnqgtsykbrlie
geilpnftyhdqrmvbckxsj

iwrhjuxaylt
rutxwihlja

xgjqs
jlgbxz
gjsxb
jxnhtg

rojtvpudx
rdpoxtjv
vjrpxdot
doxtvjpr
xdtrojpv

hwpaqudromg
pfqarht
pnqhear
pxrenahq
eapqthvr

nrhfdzetwixcjsqlbupko
btijqrefckosadwlnxyhu
hievkwjcoqtrdbxunsfml

odhzjpkrxueqymavltbsc
vchxyqjdsmfrupoanezltbk

xsdmczyvhojibkarqeplgfn
cjrzmybhfvdakqexngsplio
yogeivplzxmqskfcdrnhajb

jhtogzx
zoxjght
xtjhozg
ghtzoxj
ztoxhgj

edvbanpitmhjgwxrofklyu
cpemyxlabkdjinzrg
yadxlqkepbrgnjim

yq
mj

gbylw
zuc

glbsezpfrdxu
krzfoxgcbdsp
jfrgxsdbzpo
gzsrpbdfx
rzxfgspdb

srwzajgvukpflidxn
sdialkfnjqvrxwzp
fakwsjmnbldtrvopyxzi
zrifwvjnacxspedkql

qfbszxo
zqsbfox
bozqfxs
fosbxzq
xofbzsq

lfgirdtnsbmuhx
fuirqanhzmtdwxcs
pxhinjvuflrdsmt

kd
bdkc
dk

ysakvxpc
xascbykpz
ygkstxp
ikfspxyeh

sxewnt
anwezrxstqkgp
weysxnftij
stnxhwe
tesxwnv

ozpbw
plyobt
bjcdnoavuh

vuhczmygikjqtnwelxsap
ljhsbinurypwxmkqgca

ktlucpm
clupktf

lfx
uqrmedh

tdbnijescrzghomwvkfuxq
nsriukmjqeavxlzwgtofd
ekyoupxdzflqntrimjwvsg

jhdtvxrg
vtgjrdhx
txrgdvjh
txrdhvgj
rgtvjdxh

rfjl
bl

rp
pr
pr
qpr
pr

qatusrmofjiv
kovrwcgijz
jivolnry
tjylrnmoiv

bscyonuaed
eckjonawsbz

sl
ls
ls

dcypgbktwevhju
fhlopxgejscqziambnt

glwuep
welpug
lguewmp
wugelp
lgwpue

cbnprvmgujfeahkod
gcjinupbmrehvakdfo
upyjvnhcamfokdrbeg
oueagfpkmljrvnchtbd

ngoqjkxyeztcar
tawokezjqyxn
qetojylkxaznvdfp
qzcanobyxesthkj
oyzetagnqukjx

oluzdqyevricxgpbjh
jczoidqgpuhbrylvex
eoawvlqpzidybgcusx

gbkhxr
xgbkvhr

gynjrlazuhdsiebpm
zwgmvhr
xtfgrmhzq
wgtzrkcmh
mohrzg

ips
eiop
spi
blcipw
pie

m
mg
rmib

brelhau
aerulhb
eularhb

ntjcb
hlpfq
mhlfp

gkbzfmjinp
rwfq
wcftl
cotf

pcklviqyxumegznob
gsqecynpzlimoxukbv

hu
uhl
hu

mzktplnxfqwrd
dplwmzfrtqknx
frqpmkdxtnlziw
kyxwlztrjdsfqnmp
twzlqxkprfdmne

afblckugpyzwvh
chfuvgklywz
pxkwuvlychgdf
igshyklvecwfur
qcvtwgylunkfhj

jvbsuodzptghycqkx
thjxozkvudqsgby
yeqodxvpbukthzjgs
jutnksfqbvodzglxyh

amodbsxrzg
apbcgrskzinm
awmgtzhbsr
qvsgzbeamjlrx

rwaisemcuqxo
lrojvqxczigbsmay
facqkrmiusxo
mxdsoaqcuri

rbvzikynfoxctwghdupaem
bytwuofvzknrqagmixdpchse
vxgtnrukcidoepfbyzahwm
dzwgutvhaocryeinmkpbxf

fbp
c
auyk

lbmyuotpcnsikvgrx
ycvhpkibmtsflxegunr
gpibcmtsulxarknzjv
gpkimtovlxnrscub
dxkbghlnctswevrimpu

qok
iqz
qetpgrwyd
qnsco

ioud
udiopc
mzsfhdiu

ueikvmsr
hkvmrjuiase
evsxikplfrom
krzmgisev
iesgmvkur

kdwbteoufy
wbdfokuy

dbsj
dbfqgjek
dhbjp
pdbjo
jbd

ywnmpicadove
vawkoyice
vyaocqwie
aeiryxvwco
tecoiywav

grqoevckijwfmspahltuz
zwgcvraqfimotjlhsekp
okeahcrpmjsigtqwlzvf
rmcielqkwpogfthsvzja
iotprlamgjqfecwsvzhk

pdwxajyokg
jrsdkwgxbfae

dqlsnwreptfjxcybv
ujlpmzqrikgyantvse

wjc
cj
cj

tvzla
wacevhg

bq
bq
qub
qb
fqdbs

yks
sbyeuf

mgntlwyv
tqzvgn
gnktipzv

apkwmxjluocsvzi
ixzqpcsalkmowvj

bajuygnrwt
yubjgatrwn
gtjuwnbary
wajbgntyru
yrnbgwjuta

bmcwie
cibjwme
bweimc
eiwcmb

dbkylhiwzusm
ujkistmfwcldh
vrsplkinguhmwqxd
wiysuchkzolmd

dxkrutihaz
uiarkhtdxz
uzdksihtxarb
krihuztxad
rakhjxiztud

vhfsnzep
fsckleqdhirjyp
fshwpaeuogmx
behftsp

gxotrha
karogh
dohgnaqjm

mvs
my

fbgqisx
sfilvgax

gcvwlreubdmo
mrfcwgblvudho
vdicaompgulwtrn
cvglyduohwbjrem

axfqlgezschyiubm
xeyabhzilqusfgc
fzixqebhyslacgu
fqyucihzxgblpeas
cxqvtwrkfhbzauliysjge

lnxj
alwoycs
uqedvgz

ztkagdloihcmvjen
ovcdmjekgazhtnli

mpohrwbesqfvd
kdehobasjwrq
suebmqvdrowh
hdewrbpusoqf

fnebwpxqjzrukgo
alfwgtnhzrejdvmu

con
on

tamhynfpc
mcrhflnpujxtk
pmvicnhgsdwot

leiyrvjncqphbftdo
ldorbvncyhpfiqej
oqhnyaipjrlegcvfdb
clibfjpnyedovhqr
idfrhbevkjloycnqp

bfchelm
lbhf

b
qst

vpngb
gvbj
gnavbcu
nlgvb
sidbvg

tkpwsnedf
bxrvmltfaonhkezwyp
etkjwupgcnifq

ypvk
ypk
pwyk
kyp

farmkwguisvzbljdcqtnoe
irtcfvmbndwejgaoqzuks
omjvzkdnugcesafiqtwbr
bocntzgfuwksarjmivqed
wrtznsgijemqdkfuoabvc

a
epn

xcpqb
xszwbeujqv
bxdzejv
bq
iyrltkbf

vpolaxmcutnk
ovctkpumnl

od
tuod
od

okcewuzdtplshgxb
czkpdgohbwlseuxt
epbzodlgtuwkschx
wtbgdmzjohpxulckse
stbuhgdokewxclzp

oubmwaygzsvrpeiktq
nuaqotbmgcvrejdshkizwpy

kxw
kwd
wk

afysdhlriocjxq
yafqcrdxlohsjiz
sofqizhlxypdcjra
sjmqylwcahrofibedux

fkwbvap
jgdkzrwfvo
wifnkev
kbncvwxfp

gkhstvwcadxfrp
atfrwdksmphbvgx
rcdhgkvspawtfx

vbmzqp
r

ewluxs
mosuqlxwg
rwuclxsi
xsilwu

qpdjbnyi
mackzh

vwmsy
dctoz

xgidfke
dqxfik
davf

gwluyxi
ihoxnbazsmj
qypwitvkxcdeflr

yibzorjh
hbijqrzf

xwednlczvukoftbmajphg
wlvjfuzkapbnghdmcoex
noajcfhzgmvkweupdblx

dvhytk
mljydszthqk
cyftakh

zrhupwkt
hgrlusqofkyvipztdbn
hpcrzkumta
kerutxhpaz
pkzucwthr

wmyaf
lmhgou

rhwitl
eztgnli
dlti

svzxbogmhnteu
smzypvbelngjoucx
zebvognsmxuid
obuveznmgxs

djcqa
qadj
ajqd

lvpe
epl

nxevifrzshgutcqdm
rqvxcgnutiesdzmhf
rteshnfuvizglxwcj
fuchnevszixtrg

vmnhsx
mwhp
smheg

k
k
k
k
kj

cmki
dkacm

jhomeuldz
drozch
dsziqwxph
bhmzdtrl

umxh
xhp
hqx
xh
xh

vfmrzqltn
qtzlrv
lzavurtq
vzlrqtf
vzrtql

jflmxkwv
xlmwkvfj
wilkxjvfm

tyrsw
fsajp

pkchzmtelv
tcezkhvmpl
ptevlczmkh

ifxnytesvzbgwqhc
thbgxsqwyfiznemv

j
wi
hen
i

lgjzaswrpcvednutqko
newpozcqlkjsvrdaug

izmhfde
dghestzmi
ivhuwdzmej

icsjrlghtvayp
chysgvpletijr
pijtvryshglc
hijscpgvqrtly

nyrtchmao
cboztuhpflkq
exjgsvidw

z
z
z
z
z

azvph
tedzxmvh
yuzvh

vfqzjhp
jzqbflhvp
pzqkvdjihf
vjzfqexhaynp
qvjotzuhfp

kawzqvehnldpfyjms
qwgjsvmlyfedhpnkza
lqmwjfpsknhvyaezd

plrjckxaihbn
xbcnfhpauktir
hxbekmqwzidnaoc
sgynjacxhkib

elstkqybopx
qpldszxenu
philxrvacem

zfqvwapib
vyuwqtaizlfmr
fwxaqizoknc

fcxkg
kgucf
cfgwk

bifpnhqodgzy
qeudfpyozihng
ihqgyofndzp

gbdlhztckxvfmp
bdflhegkvmtapxz
gkvdtbfhxlzcpm
fblkphjzxvgmdt

mbkqegcvysdat
dgamlysetcbq
gcyedbmsqat
stdcmgbqeya
mdgaseytbqc

mbanitlspwd
idohstranwluby
tlfihanjszbwdq
csvwnatkibeldx
gdlnbawsit

koc
aklh

nwforsix
rfoinxgsq
ponu

frkqweaplmyjtszc
evqlhgdnofpbsx

xmjafe
nasx
gvtoirb

fh
ahcvf
fubh
hf

cnziuowlvd
wdulnczo
clnuwozid
usnolwdcz
lzuodcnw

qzvo
ozqv
ozqv
ovqz

petyqas
ysap
ypsjia

as
jrd

vczyxagnhlik
axhcklzgviny
rnlayxcgvzkhi
hcnizvgklyxa

bhzyoer
odrnzth
bdrzpxohf
onrzh
ozmrwgh

dwbikvq
qbdkivw
kvwbdiq
bqitkvdw

zaiwc
nzdipc

hupmeoy
feonhpym
pojmytcehkz
mhpoey
pehoyfmn

iexqrjdunpmhkzsvcy
tzjhxenvdqmpbcuykris
xyenuidspclvmkhjzrq

eputvsadhcwmxilyzrf
xhldizuwfpryamsctev

vruljzbpqwkimgcdnhox
dvrhjzwlubxgqmpnokci
rogckbmzwlnjphxdvqiu

mflxyrdnaiojwh
hvxdjufmlwgryioq
hialwfmxdoryj

zqjyx
qyjxz
guyrjxczq

jkptbc
ctpnbk

djoxvsbktw
zhryojfgxnpamtbiqc
keloxbwjtu

waeixkdunptl
atejnixwk
yaiwnebzktjx
nraxetkwi

srvltzm
artzpiml
zmfrutld
trmzl
rzfltm

upews
uwesp
ewusp
upwes
uwsep

gmqadcilxpustz
zmdaputbsglxcki
gtudmizpcaxsl
audigpclmsxzt
zcxasluogptmdin

mjxvndbfygluqh
vbpdxunmh

thlqwmnfyvxkcjaibro
wnohvfbukmtcjrqxlyai

iauh
dh
pqsnwke
td
t

k
k
k
k

tnirszcf
tprnchfolzi

vyf
vftay
pvof

nrzbpfdae
ofanrjbep
avnprbemfo
bgaufphxern
tbncrqeafjzidp

wbosukfnidm
bptmsnodiuef

fop
pfo

ztfplvdymqciuarewxgkn
vwufqxyerpcoitnadlmgzk
ndlwgymuirapzxketqvfhc
gwecvyqdmlraunxjiktpzf

ozjs
tmqezknf
rmvcn
hydpigwulb
es

umlvzepcx
vuxmeclzp
vlzuexcpm
zmecpvulx
mpzvulcex

oeixhj
iexhoj
jexoih
iohvejxnk
ihexjo

uzw
zsb
zu
zp

mrqanvodfx
rnqmdvfa
abdptrgzflyhnvs

pjivzyacou
vnapjyzuwbf
ajpuvzy

yrbptzgsevaomukldqxwicnj
ksatveqludrmbjngoixwycz
zdrigcyauxvmlsektbqojnw
lrwdgumyojnqcaizxsebkvt

ipsc
picl
ptcasgi
pgvci

u
e
j
e

gz
ezj
ibolpzf
zg
z

ihsemv
ocaj
gu
kidw
vmultwkr

xka
x
xq

vdyzsapbwntmxojfqiclek
dfhscbwpqxltmjzoga

aiswlquvcmhy
cusylqwiav

jmowrpbflkcysdzt
fvaxlbkmdiowjtzrqp
lpuwrjkohfdtmsbzge

jgrqosxzyuki
skhoquigzrypjbx
zysixgukrqjo

tesvcrwaqzmfnyb
fbeciswqrzmta
wmqerlfczatbps

hx
xvjh
xh
xurcwzh
hvx

kzh
kzh
khz
khz
zhk

rezwblynpvodigmj
ltzreviupnybgqdjkm
zjervlnixmydgspb

abfcgeymizjdnxklrtqs
czqemlanjpfsohryxg
xjfnqyeagrsuczlmh

myrqdea
mdtkp
wjudvohlmn
pmdc
mkd

x
n
n
n

obpuqvwmnx
mwnqbxvupo
oqwpnuxvmb
uxmpwbqvno
qmpuxbvnow

xzesnocrmaqtfvlkby
ltzxykcnbfaqvmesor
cstoenkyfmplzqxgvab

qwkuhcmvrlg
hlkrmuygwqxv

rkgvmfyz
sqnc
dohxti

pkufhtwxvnjqd
fshdqpizmvtoewculnb
phgduwftvqn
dqajtyhprvgufnw

qwzkrhs
zulqk
qvozjkyga
ntcbqlkz

ucgmdbajz
xtg

nx
xn

kajgbcyoetfiplx
twloixgaekbjpfcy
jlifxpbtoycagek

mrspqwdnuye
yujewnrasqmp

mafzxkliquynsropbtgejhcw
mrxfnicjwelukgzqtspahybo
xryozujfakinebpqwslhtmgc
poschfkrtxjwegimuzqyalnb
omtswhzuijaxlpgeybknrfqcv

qs
sq
lqs
sq
sqpi

apfgtns
ryigfnsq

vjz
ql

pftkg
dyncoulih
bzpg
f
g

qhc
ehxdzqc
qgb
qjrvlsf
cdqtb

rusxpwgebjtqc
lgdzuinwvje
lfjwmgoeuadz

dafvyi
iylefg
flciy

echxoit
ytup
thxasiod

singvkzjohmct
njvcmzthkigps
gmnjsdctizvhk
psmvjkhzogictn

xjgukmfnls
sukxjf
xfudbsejnkv
ubhmjksfzx
stxpqfucjkioaw

tq
tq

bleyvp
ebypv
yewvpdb

xsjbancl
asq
as

inl
ny
vn
sbtf

ihgvopwqzydftlxcbjm
xpdhgjtqybocekl
uhogsrjblpdxcyqtn
rqbltpahkjgcodyx

tzagysvdrebjhnlmu
djugelzvrsnabt
brsculznevdjtag
udbzxjgtievarnsl

icalhfwu
uafcliw
waulzfic
auilcwf
filcuwma

qetcajuhmpbkvsdi
khpwjutmvcyesbadqi
bpvucajhidemqtsk

f
m
m

bxqtkhwynlezfp
moptexhzfabwcy
dzkryhtpbvqxefw

gyznickf
ws
o
abd
shj

moqsnucx
mnucxqso
qoxcsnum
mjcsnqxuo

yad
cday
dya
ady

acsl
la
ila
lavwj
al

eq
eq
teq
qe

ypgtbcqnio
vwj
jwu
rk

b
b
b
uab
b

vpnjtyldieokwc
dlyifcjxzmvgbhwnper

cpxasmqgytkbl
lqstpbgmkiza
bgamkdtsplq
jatqcxlsyubkpmgrw
ksgqpdltafnbcm

vikupeozmxgqw
kexgjzvmwuqoip
wzgvoimkpexqu
kuwgizqxompev

dbctojnhsz
mpochelwauvbjdqrn
bhdjonscz

tmxgo
ogxmt
ogmtx
gtxom

ogdzpe
dzpgeo
zopdeg

ajit
rintad

xtjghyrzcfl
atfzxcyrljq

dl
arl
arlg
pl
mwoklu

w
k
w
w

tzycrupmsh
trucmzsphy
husmrczpty
mryuczptsh
zuyjtpmrhsc

txbpilyem
lbtzex
xgehlbtw
eclmbxt

aiwbpzesk
yugvlxojqd
ichwa
inz

bdes
sedb
sueqfbd
dbes
rbeds

xbzefntus
snypzbfmeku

skjrbtqewpyg
iqjkyxaswe

hdegjifr
jwxrahfmbei
irfqolet
eairzfnhs

p
sp
p

g
x
xv
ia

jozvgcpk
vpo
opfv
pov

owgfclzraqikb
rkbqalfico

djsyxnkw
ws
wgs
wlsg
svpfwu

oyjlxstrebvpfhmd
opfrjctbnemyvd

lpqebxykmfdsnt
emfnsdxptbkyq
mdobqptnkjxfeys

waktzxvocjruilhgym
uvtakjblqwsyoghzp
yklvjwniutehzgoa

fhqgy
yqhfg

brlsfdopmqyng
fnlrdpbosqymg

lyksz
rzyk

jamvlpcydqzu
azdcpmvquylj
zlqmpydvuacj
cydluaqmzvjp
lmqzdvajuycp

nuixfj
ixozbutyj
isxlpkqhwrmvj
jincxga
jenidx"#;

// #endregion

