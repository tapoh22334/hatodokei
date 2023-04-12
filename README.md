# 鳩時計時報

シンプルな時報アプリ。一時間ごとに時間を読み上げます。

 - [ダウンロード](https://booth.pm/ja/items/4187459?BOOTH-APP-CLIENT-VERSION=%2Bnvvurdsi%3D%2B%2Fsearch%2Fhair)
 - [旧バージョンダウンロード](https://github.com/iwase22334/hatodokei/releases)

![キャプチャ1](https://raw.githubusercontent.com/iwase22334/hatodokei/main/capture/Capture1.png)
![キャプチャ2](https://raw.githubusercontent.com/iwase22334/hatodokei/main/capture/Capture2.png)

## 機能

 - [x] 時報
 - [x] 特定の時報をミュート
 - [x] 全体の音量調整
 - [x] タスクトレイ在中
 - [x] 設定の保存
 - [x] 音声の選択
 - [ ] 時間の追加・編集


## 動作確認環境

 - Windows 10
 - Mac

## 開発

開発言語: Rust + React

GUIおよび音声再生に以下のライブラリを使用。

Third party libraries

 - Tauri: https://github.com/tauri-apps/tauri
 - rodio: https://github.com/RustAudio/rodio

### 環境構築

```
cargo install tauri-cli --version "^1.0.0"
yarn add -D @tauri-apps/cli
```

### Build

- debug
```
cargo tauri dev
```

- release
```
cargo tauri build
```

- lint, formatt

```
npx prettier --write src/*.ts src/*.tsx
(cd src-tauri; cargo fmt)
(cd src-tauri; cargo clippy -- -D warnings)
```

- license file generation


```
(cd src-tauri; cargo about generate -o ../resource/THIRD-PARTY-NOTICES-cargo.txt about.hbs)
(yarn licenses generate-disclaimer | tee resource/THIRD-PARTY-NOTICES-yarn.txt)
```


### ソースコード

本ソフトウェアはフリーソフトであり、
ソースコードはMITライセンスで公開しています。 
含まれている音声ファイルは別ライセンスになります。
注意してください。

### 音声について

以下のソフトを使用して開発しています。

■無料AIトークソフトCOEIROINK: https://coeiroink.com
■ VOICEVOX: https://voicevox.hiroshiba.jp/

実際に使用している音声については、
実行ファイルの「License」->「使用している音声」の項目を参照ください。
