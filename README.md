## テキストの事前処理

`nkf` がインストールされていれば

```=bash
bash convert.sh example.txt
```

で `example.txt` を UTF-8 に変換したファイルが `output/` 以下に置かれます（`output/` フォルダがない場合は作成してください）。

## 実行

`example.txt` というテキストファイル内を検索する場合

```=bash
cargo run -- example.txt
```

で検索できます。
