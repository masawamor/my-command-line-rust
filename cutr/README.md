
* 範囲には、開始と終了の両方を示す必要がある（両端を含む）
* ユーザーが指定した順番で範囲が表示される
* 範囲は、重複する値を含む場合がある
* 区切り文字を含むテキストファイルの解析では、エスケープされた区切り文字を考慮する

cutr 0.1.0
Ken Youens-Clark <kyclark@gmail.com>
Rust cut

USAGE:
    cutr [OPTIONS] [FILE]...

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -b, --bytes <BYTES>        Selected bytes
    -c, --chars <CHARS>        Selected characters
    -d, --delim <DELIMITER>    Field delimiter [default:  ]
    -f, --fields <FIELDS>      Selected fields

ARGS:
    <FILE>...    Input file(s) [default: -]