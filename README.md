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
(cd src-tauri; cargo about generate -o ../resource/THIRD-PARTY-NOTICES-cargo.txt about.hbs)
(yarn licenses generate-disclaimer | tee resource/THIRD-PARTY-NOTICES-yarn.txt)


### ソースコード

本ソフトウェアはフリーソフトであり、ソースコードはMITライセンスで公開しています。 
本ソフトウェアに含まれるOSSのライセンスは
 - https://github.com/iwase22334/hatodokei/blob/main/THIRD-PARTY-NOTICES.txt
を参照してください。

### 音声について
以下のボイスを使用しています。

■無料AIトークソフトCOEIROINK: https://coeiroink.com

 ■COEIROINK:つくよみちゃん
 ■フリー素材キャラクターつくよみちゃん: https://tyc.rei-yumesaki.net

 ■COEIROINK:MANA
 ■音声合成キャラクターMANA: https://cf-mk.jimdofree.com/mana/

 ■COEIROINK:KANA
 ■音声合成キャラクターKANA: https://cf-mk.jimdofree.com/kana/

 ■COEIROINK:おふとんP
 ■おふとんPの作業部屋: http://mohumohuton.web.fc2.com

■ VOICEVOX: https://voicevox.hiroshiba.jp/

 ■VOICEVOX:ずんだもん
 ■VOICEVOX:四国めたん

