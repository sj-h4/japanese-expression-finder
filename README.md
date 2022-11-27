## Shift-JIS から UTF-8 への変換

`nkf` がインストールされていれば

```=bash
bash convert_shift_jis2utf8.sh example.txt
```

で `example.txt` を UTF-8 に変換したファイルが `output/` 以下に置かれます（`output/` フォルダがない場合は作成してください）。

## 方針

その１

- 一行に一文となるようにテキストを処理
- 対象となるテキストを形態素解析し、単語リストを作成（ついでに書字形基本形に変換したら検索が楽？）
- 単語リストから述語を抽出し、対象となるテキストに含まれる述語リストを作成
- 述語リストにある述語を反復して譲歩の意味を表す構文を検索

その２

- 一文（一行？）ずつ形態素解析して、「には」か「は」や「も」の前後数語以内に同じ語が使われているかを調べる
  - 「は」か「が」「けれど」「けど」とかが入っているものに絞ったほうが効率良さそう

### 問題点

- 「やるにはやる」という文は形態素解析の結果、「やる」「に」「はやる」となる
