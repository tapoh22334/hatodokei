# 鳩時計時報

シンプルな時報アプリ

## 機能

### 実装済み
 - [x] 時報
 - [x] 特定の時報をミュート
 - [x] 全体の音量調整

### 追加予定
 - [] 時間の編集
 - [] 音声ファイルの読み込み
 - [] タスクトレイ化


### ビルド方法

Make sure you are using the latest version of stable rust by running `rustup update`.

`cargo run --release`

On Linux you need to first run:

`sudo apt-get install libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev libspeechd-dev libxkbcommon-dev libssl-dev`

On Fedora Rawhide you need to run:

`dnf install clang clang-devel clang-tools-extra speech-dispatcher-devel libxkbcommon-devel pkg-config openssl-devel libxcb-devel`

For running the `build_web.sh` script you also need to install `jq` and `binaryen` with your packet manager of choice.


## 音声に関して

フリー素材キャラクター「つくよみちゃん」（© Rei Yumesaki）を使用しています。
https://tyc.rei-yumesaki.net/

■音声合成ソフト：つくよみちゃん@COEIROINK
