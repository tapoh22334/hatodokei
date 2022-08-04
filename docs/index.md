# 鳩時計時報

シンプルな時報アプリ。一時間ごとに時間を読み上げます。

 - [ダウンロード](https://github.com/iwase22334/hatodokei/releases)

![キャプチャ1](https://raw.githubusercontent.com/iwase22334/hatodokei/main/capture/Capture1.png)
![キャプチャ2](https://raw.githubusercontent.com/iwase22334/hatodokei/main/capture/Capture2.png)

## 機能

 - [x] 時報
 - [x] 特定の時報をミュート
 - [x] 全体の音量調整
 - [x] タスクトレイ在中
 - [x] 設定の保存
 - [ ] 時間の追加・編集


## 動作確認環境

 - Windows 10

## 開発

開発言語: Rust + React

GUIおよび音声再生に以下のライブラリを使用。

Third party libraries

 - Tauri: https://github.com/tauri-apps/tauri
 - rodio: https://github.com/RustAudio/rodio

### Build

- debug
```
cargo tauri dev
```

- release
```
cargo tauri build
```

## Lisence

ソースコード:

 - MIT

音声データ:

 - フリー素材キャラクター「つくよみちゃん」（© Rei Yumesaki）を使用しています。
 - ■音声合成ソフト：つくよみちゃん@COEIROINK
 - https://tyc.rei-yumesaki.net/
