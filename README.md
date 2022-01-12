
# MIGU-Music-dl   

![Publish Action](https://github.com/swim2sun/migu-music-dl/actions/workflows/publish.yml/badge.svg)

Download Migu Lossless Music

## Installation

```shell
$ pip install migu-music-dl
```
    

Usage
-----

```shell
$ migu-music-dl [OPTIONS] SEARCH_KEYWORD OUTPUT_DIR
```


For example:

```
➜  migu-music-dl '周杰伦' .

+-----+-----------------------------------+--------+---------------+
| No. | Title                             | Artist |     Album     |
+-----+-----------------------------------+--------+---------------+
|  1  | 花海                              | 周杰伦 |     魔杰座    |
|  2  | 我是如此相信 (电影《天火》主题曲) | 周杰伦 |  我是如此相信 |
|  3  | 七里香                            | 周杰伦 |   Initial J   |
|  4  | 反方向的钟                        | 周杰伦 | Partners 拍档 |
|  5  | 晴天                              | 周杰伦 |     叶惠美    |
|  6  | 一路向北 (电影《头文字Ｄ》插曲)   | 周杰伦 |  十一月的萧邦 |
|  7  | 明明就                            | 周杰伦 |               |
|  8  | 稻香                              | 周杰伦 |     魔杰座    |
|  9  | 夜曲                              | 周杰伦 |  十一月的萧邦 |
|  10 | 爱在西元前                        | 周杰伦 | Partners 拍档 |
|  11 | 搁浅                              | 周杰伦 |     七里香    |
|  12 | 半岛铁盒                          | 周杰伦 |    八度空间   |
|  13 | 兰亭序                            | 周杰伦 |     魔杰座    |
|  14 | 枫                                | 周杰伦 |  十一月的萧邦 |
|  15 | 给我一首歌的时间                  | 周杰伦 |     魔杰座    |
|  16 | 以父之名                          | 周杰伦 |   Initial J   |
|  17 | 轨迹(电影《寻找周杰伦》主题曲)    | 周杰伦 |               |
|  18 | 等你下课(with 杨瑞代)             | 周杰伦 |    等你下课   |
|  19 | 蒲公英的约定                      | 周杰伦 |     我很忙    |
|  20 | 夜的第七章                        | 周杰伦 |   依然范特西  |
+-----+-----------------------------------+--------+---------------+
input No. to download (split with , for download multiple songs, for example: 1,3,5): 14,16
selected: 枫, 以父之名
枫.flac  [####################################]  100%
以父之名.flac  [####################################]  100%

```
