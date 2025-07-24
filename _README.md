# castnow

Castnow is a command-line utility that can be used to play back media files on
your Chromecast-enabled device. It supports playback of any
[Chromecast supported media files
](https://developers.google.com/cast/docs/media),
videos on the web and torrents. If no media is passed as a parameter, castnow
will re-attach to a running playback session.

This version of castnow:

- Almost fixes the glaring omission of the Chromecast not using the metadata
and cover art present in music files to display. The almost part being to only
use a tiny version of the cover art to fit in the arbitrary maximum message
size of 64k.

- Supports playlist parsing. Common basic formats (CUE/M3U/PLS) are supported
and as a bonus HTML files are parsed for content as well. CUE format INDEX
is also supported for single CD files, but only for selecting tracks.


### Usage

```

// start playback of a local video file
castnow ./myvideo.mp4

// start playback of video and mp3 files in the local directory
castnow ./mydirectory/

// playback 3 videos after each other
castnow video1.mp4 video2.mp4 video3.mp4

// start playback of an mp4 file over the web
castnow http://commondatastorage.googleapis.com/gtv-videos-bucket/ED_1280.mp4

// start playback of a video over torrent
castnow <url-to-torrent-file OR magnet>

// start playback of a video over torrent with local subtitles
castnow <url-to-torrent-file OR magnet> --subtitles </local/path/to/subtitles.srt>

// transcode some other video format to mp4 while playback (requires ffmpeg)
castnow ./myvideo.avi --tomp4

// transcode only audio while playback (in case the video shows, but there's no audio)
castnow ./myvideo.mkv --tomp4 --ffmpeg-vcodec copy

// change the increment at which the volume steps up or down. A lower number
// is helpful if your speakers are very loud, and you want more precision over
// the change in volume
castnow ./song.mp3 --volume-step "0.01"

// re-attach to a currently running playback session
castnow

```

### Options
<table>
<tr>
<td>
<code>--tomp4 </code>
</td>
<td>
Transcode a video file to mp4 during playback. This option requires ffmpeg to
be installed on your computer. The play / pause controls are currently not
supported in transcode mode.
</td>
</tr>
<tr>
<td>
<code>--device "my chromecast" </code>
</td>
<td>
If you have more than one Chromecast on your network, use this option to
specify the device on which you want to start casting. Otherwise, castnow will
just use the first device it finds in the network.
</td>
</tr>
<tr>
<td>
<code>--address 192.168.1.4 </code>
</td>
<td>
The IP address or hostname of your chromecast. This
  will skip the MDNS scan and improve the initial response time.
</td>
</tr>
<tr>
<td>
<code>--subtitles <path/URL> </code>
</td>
<td>
This can be a path or URL to a vtt or srt file that contains subtitles.
</td>
</tr>
<tr>
<td>
<code>--subtitle-scale 1.5 </code>
</td>
<td>
Scaling factor for the size of the subtitle font. Default is 1.0.
</td>
</tr>
<tr>
<td>
<code>--subtitle-color #FFFFFFFF </code>
</td>
<td>
Foreground RGBA color of the subtitle font.
</td>
</tr>
<tr>
<td>
<code>--myip 192.168.1.8 </code>
</td>
<td>
Your main IP address. (Useful if you have multiple network adapters.)
</td>
</tr>
<tr>
<td>
<code>--quiet </code>
</td>
<td>
Hide the player timeline.
</td>
</tr>
<tr>
<td>
<code>--peerflix-&lt;option&gt; &lt;argument&gt; </code>
</td>
<td>
Pass options to peerflix.
</td>
</tr>
<tr>
<td>
<code>--ffmpeg-&lt;option&gt; &lt;argument&gt; </code>
</td>
<td>
 Pass options to ffmpeg.
</td>
</tr>
<tr>
<td>
<code>--type <type> </code>
</td>
<td>
Explicity set the mime-type of the first item in the playlist
(e.g. 'video/mp4').
</td>
</tr>
<tr>
<td>
<code>--seek <hh:mm:ss> </code>
</td>
<td>
Seek to the specified time on start using the format hh:mm:ss or mm:ss.
</td>
</tr>
<tr>
<td>
<code>--bypass-srt-encoding </code>
</td>
<td>
Disable automatic UTF-8 encoding of SRT subtitles.
</td>
</tr>
<tr>
<td>
<code>--loop </code>
</td>
<td>
Play the list of files over and over in a loop, forever.
</td>
</tr>
<tr>
<td>
<code>--shuffle </code>
</td>
<td>
Play the list of files in random order.
</td>
</tr>
<tr>
<td>
<code>--recursive </code>
</td>
<td>
 List all files in directories recursively.
</td>
</tr>
<tr>
<td>
<code>--volume-step </code>
</td>
<td>
Step at which the volume changes. Helpful for speakers that are softer or
louder than normal. Value ranges from 0 to 1. Default is 0.05.
</td>
</tr>
<tr>
<td>
<code>--metadata false</code>
</td>
<td>
Do not attempt to retrieve metadata from audio media.
</td>
</tr>
<tr>
<td>
<code>--showmetadata</code>
</td>
<td>
Show metadata.
</td>
</tr>
<tr>
<td>
<code>--showcover false</code>
</td>
<td>
Do not show cover art on console.
</td>
</tr>
<tr>
<td>
<code>--showoptions</code>
</td>
<td>
Show options.
</td>
</tr>
<tr>
<td>
<code>--command &lt;key1>,&lt;key2>,... </code>
</td>
<td>
Execute key command(s) (where each <code>&lt;key&gt;</code> is one of the keys
listed under *player controls*, below).
</td>
</tr>
<tr>
<td>
<code>--exit </code>
</td>
<td>
Exit when playback begins or <code>--command &lt;key&gt;</code> completes.
</td>
</tr>
<tr>
<td>
<code>--help </code>
</td>
<td>
 Display help message.
</td>
</tr>
</table>

Optionally, options can be preset by storing them in a file named `.castnowrc`
in the current user's home directory. For example:

```
--myip=192.168.1.8
--volume-step=0.01
```

### Player Controls
| Key                  | Action                                                        |
| -------------------: | :------------------------------------------------------------ |
| <kbd>**Space**</kbd> | Toggle between play and pause                                 |
| <kbd>**m**</kbd>     | Toggle mute                                                   |
| <kbd>**t**</kbd>     | Toggle subtitles                                              |
| <kbd>**Up**</kbd>    | Volume up                                                     |
| <kbd>**Down**</kbd>  | Volume down                                                   |
| <kbd>**Left**</kbd>  | Seek backward (keep pressed / multiple press for faster seek) |
| <kbd>**Right**</kbd> | Seek forward (keep pressed / multiple press for faster seek)  |
| <kbd>**p**</kbd>     | Previous item in the playlist (only supported in launch-mode) |
| <kbd>**n**</kbd>     | Next item in the playlist (only supported in launch-mode)     |
| <kbd>**s**</kbd>     | Stop playback                                                 |
| <kbd>**q**</kbd>     | Quit                                                          |


### YouTube Support

We had to drop direct YouTube support for now since google changed the
Chromecast YouTube API. However, there is a nice workaround in combination with
the tool [youtube-dl](https://github.com/rg3/youtube-dl):

`youtube-dl -o - https://youtu.be/BaW_jenozKc | castnow --quiet -`

Thanks to [trulex](https://github.com/trulex) for pointing that out.

### Non-Interactive

Castnow can also be used in cron jobs or via window-manager bindings; for
example:

```
// Play/pause.
castnow --command space --exit

// Louder.
castnow --command up --exit
```


### serve-mp4.js の仕様

`utils/serve-mp4.js` モジュールは、ローカルのビデオファイルをHTTP経由でChromecastデバイスに提供する役割を担うシンプルなNode.jsスクリプトです。ローカルメディアファイルを再生するための重要なコンポーネントです。

**機能:**

*   **HTTPサーバー:** 単一のファイルに対してシンプルなHTTPサーバーとして機能します。
*   **MIMEタイプ検出:** `mime`ライブラリを使用して、ファイルの拡張子に基づいて正しい`Content-Type`を自動的に決定します。
*   **Rangeリクエストのサポート:** HTTPのRangeリクエストを適切に処理します。これはビデオストリーミングにとって不可欠であり、Chromecastがビデオの異なる部分にシークしたり、コンテンツを効率的にバッファリングしたりすることを可能にします。`range-parser`ライブラリを利用して、リクエストヘッダーの`Range`フィールドを解釈し、指定された範囲のデータを返します。Rangeリクエストを受け取ると、ファイルの要求された部分のみを`206 Partial Content`ステータスコードで提供します。
*   **CORS:** すべてのレスポンスに `Access-Control-Allow-Origin: *` ヘッダーを含めることで、Chromecastが異なるオリジン（ローカルサーバー）からコンテンツにアクセスできるようにします。
*   **フルファイルサポート:** リクエストにRangeが指定されていない場合、ファイル全体を`200 OK`ステータスコードで提供します。

#### range-parser

`range-parser`は、HTTPの`Range`ヘッダー文字列を解析するためのNode.jsモジュールです。リソースのサイズと`Range`ヘッダー（例: `bytes=0-499`）を受け取り、範囲オブジェクトの配列を返します。これにより、`serve-mp4.js`は指定された範囲のビデオデータのみを効率的にクライアントに送信できます。

**詳細な仕様:**

*   **入力:**
    *   `size` (数値): リソースの合計サイズ（バイト単位）。
    *   `str` (文字列): HTTP `Range` ヘッダーの値。
    *   `options` (オブジェクト, オプショナル):
        *   `combine` (真偽値): `true`に設定すると、隣接または重複する範囲を結合します。
*   **出力:**
    *   成功した場合: 解析された範囲を示すオブジェクトの配列を返します。各オブジェクトは `start` と `end` プロパティを持ちます。
        *   例: `[ { start: 0, end: 499 } ]`
    *   範囲が不正な場合: `-1` を返します (例: `bytes=500-200`)。
    *   ヘッダー文字列の形式が正しくない場合: `-2` を返します。
*   **その他:**
    *   複数の範囲指定 (例: `bytes=0-10, 20-30`) もサポートします。
    *   `Content-Range`ヘッダーの生成には寄与しません。別途実装が必要です。

#### mime

`mime`は、ファイル拡張子からMIMEタイプを、またはその逆を決定するための、依存関係のないライブラリです。`serve-mp4.js`では、提供するファイルの`Content-Type`ヘッダーを正しく設定するために使用されます。

**主な機能:**

*   `mime.getType(path)`: ファイルパスまたは拡張子からMIMEタイプを取得します。
*   `mime.getExtension(type)`: MIMEタイプからデフォルトのファイル拡張子を取得します。

### grab-opts.js の仕様

`utils/grab-opts.js`は、特定のプレフィックスを持つオプションを抽出するためのユーティリティ関数です。`castnow`では、`--peerflix-`や`--ffmpeg-`といったプレフィックスを持つコマンドラインオプションを、それぞれのライブラリに渡すために使用されます。

**機能:**

*   **入力:**
    *   `options` (オブジェクト): すべてのコマンドラインオプションを含むオブジェクト。
    *   `prefix` (文字列): 抽出したいオプションのプレフィックス。
*   **出力:**
    *   プレフィックスに一致するキーを持つ新しいオブジェクトを返します。キーからはプレフィックスが削除されます。

**例:**

```javascript
const options = { '--ffmpeg-vcodec': 'copy', '--other-option': 'value' };
const ffmpegOptions = grabOpts(options, '--ffmpeg-');
// ffmpegOptions は { 'vcodec': 'copy' } となります。
```

### unformat-time.js の仕様

`utils/unformat-time.js`は、`hh:mm:ss`または`mm:ss`形式の文字列を秒に変換するユーティリティ関数です。`--seek`オプションで指定された時間を秒単位の数値に変換するために使用されます。

**機能:**

*   **入力:**
    *   `string` (文字列): `hh:mm:ss`または`mm:ss`形式の時間文字列。
*   **出力:**
    *   変換された合計秒数を数値で返します。

**例:**

```javascript
const timeString1 = '01:02:03';
const seconds1 = unformatTime(timeString1);
// seconds1 は 3723 となります。

const timeString2 = '05:30';
const seconds2 = unformatTime(timeString2);
// seconds2 は 330 となります。
```

### directories.js の仕様

`plugins/directories.js`は、プレイリストに含まれるディレクトリを再帰的に探索し、再生可能なメディアファイル（.mp3, .mp4, .flac）を抽出してプレイリストを平坦化するプラグインです。

**機能:**

*   `--recursive` オプションが有効な場合、ディレクトリを再帰的に探索します。
*   ディレクトリ内のファイルをフィルタリングし、許可された拡張子を持つファイルのみをプレイリストに追加します。

#### diveSync

`directories.js`は`diveSync`というライブラリに依存しています。`diveSync`は、ディレクトリツリーを同期的に走査するための非常に小さなNode.jsモジュールです。しかし、このライブラリは**10年以上更新されておらず(バージョン0.3.0)**、現在はメンテナンスされていない可能性が高いことに注意が必要です。

**`diveSync`の機能:**

*   指定されたディレクトリを同期的に（オプションで再帰的に）探索します。
*   `filter`オプションを使用することで、探索するファイルやディレクトリをフィルタリングできます。
*   各ファイルに対してコールバック関数を実行します。

### localfile.js の仕様

`plugins/localfile.js`は、ローカルのメディアファイルをChromecastで再生可能にするためのプラグインです。ローカルファイルを再生するために、HTTPサーバーを起動し、各ファイルに一意のURLを割り当てます。

**機能:**

*   **HTTPサーバーの起動:** ローカルファイルを提供するためのHTTPサーバーを起動します。ポートは`--localfile-port`オプションで指定でき、デフォルトは`4100`です。
*   **URLの割り当て:** プレイリスト内の各ローカルファイルに対して、`http://<ip>:<port>/<index>`という形式のURLを割り当てます。IPアドレスは`--myip`オプションで指定するか、`internal-ip`モジュールによって自動的に検出されます。
*   **メディアタイプの処理:** `mime`ライブラリを使用してファイルのMIMEタイプを判別します。オーディオまたはビデオでない場合は、`video/mp4`として扱われます。
*   **リクエストの処理:** `serve-mp4`モジュールを利用して、Chromecastからのリクエストに応じてファイルの内容を配信します。

### stdin.js の仕様

`plugins/stdin.js`は、標準入力からのデータをChromecastで再生可能にするためのプラグインです。`youtube-dl`など、他のコマンドラインツールと連携して使用することを想定しています。

**機能:**

*   **HTTPサーバーの起動:** 標準入力を提供するためのHTTPサーバーを起動します。ポートは`--stdin-port`オプションで指定でき、デフォルトは`4104`です。
*   **URLの割り当て:** 標準入力に対して、`http://<ip>:<port>`という形式のURLを割り当てます。
*   **データ転送:** 受け取ったHTTPリクエストに対して、`process.stdin`をパイプすることで、標準入力からのデータを直接ストリーミングします。

### subtitles.js の仕様

`plugins/subtitles.js`は、ビデオに字幕を追加するためのプラグインです。`--subtitles`オプションで指定された字幕ファイル、またはビデオファイルと同じディレクトリにある同名の`.srt`ファイルを自動的に読み込みます。

**機能:**

*   **字幕ファイルの自動検出:** `--subtitles`オプションが指定されていない場合、ビデオファイルと同じディレクトリにある同名の`.srt`ファイルを自動的に探します。
*   **SRTからVTTへの変換:** 字幕ファイルがSRT形式の場合、`srt2vtt`ライブラリを使用してVTT形式に変換します。`--bypass-srt-encoding`オプションでエンコーディングをバイパスできます。
*   **HTTPサーバーの起動:** 変換後のVTT字幕を提供するためのHTTPサーバーを起動します。ポートは`--subtitle-port`オプションで指定でき、デフォルトは`4101`です。
*   **字幕のスタイル設定:** `--subtitle-color`や`--subtitle-scale`といったオプションを使用して、字幕のスタイル（色、サイズなど）をカスタマイズできます。

### torrent.js の仕様

`plugins/torrent.js`は、torrentファイルまたはmagnetリンクからビデオをストリーミング再生するためのプラグインです。`peerflix`ライブラリを利用して、torrentのコンテンツをHTTPストリームに変換します。

**機能:**

*   **torrentのサポート:** `.torrent`ファイルへのパス、または`magnet:`で始まるmagnetリンクを認識します。
*   **peerflixの利用:** `read-torrent`でtorrentを読み込み、`peerflix`エンジンを起動してビデオのストリーミングを開始します。
*   **HTTPサーバーの起動:** `peerflix`が起動したHTTPサーバーのURLをプレイリストに追加します。ポートは`--torrent-port`オプションで指定でき、デフォルトは`4102`です。`--peerflix-port`オプションも利用可能です。
*   **オプションの引き渡し:** `--peerflix-<option>`という形式のオプションを`peerflix`に渡すことができます。

### transcode.js の仕様

`plugins/transcode.js`は、`--tomp4`オプションが指定された場合に、ビデオをMP4形式にリアルタイムでトランスコードするためのプラグインです。`ffmpeg`がシステムにインストールされている必要があります。

**機能:**

*   **リアルタイムトランスコード:** `stream-transcoder`ライブラリを使用して、ビデオストリームをh264コーデックのMP4コンテナにトランスコードします。
*   **HTTPサーバーの起動:** トランスコードされたストリームを提供するためのHTTPサーバーを起動します。ポートは`--transcode-port`オプションで指定でき、デフォルトは`4103`です。
*   **ffmpegオプションの引き渡し:** `--ffmpeg-<option>`という形式のオプションを`ffmpeg`に渡すことができます。
*   **制限事項:** トランスコード中はタイムライン表示とシーク機能が無効になります。

### xspf.js の仕様

`plugins/xspf.js`は、XSPF (XML Shareable Playlist Format) ファイルを解析し、その中のメディアファイルをプレイリストに追加するためのプラグインです。これにより、XSPF形式のプレイリストを`castnow`で利用できるようになります。

**機能:**

*   **XSPFファイルの検出:** プレイリスト内の`.xspf`拡張子を持つファイルを識別します。
*   **XSPFの解析:** `xspfr`ライブラリを使用してXSPFファイルを読み込み、その中のメディアエントリ（パスとタイトル）を抽出します。
*   **プレイリストの拡張:** 抽出されたメディアエントリを`castnow`のプレイリストに追加します。
*   **制限事項:** 現在、外部のXSPFリンク（ファイルパスがローカルでないもの）はサポートしていません。

## index.js の仕様

`index.js`は`castnow`アプリケーションのエントリポイントであり、コマンドライン引数の解析、Chromecastデバイスとの連携、メディア再生の制御、および各種プラグインの統合を担当します。

**主な機能:**

*   **コマンドライン引数の解析:** `minimist`ライブラリを使用して、コマンドラインから渡されたオプション（`--tomp4`, `--device`, `--subtitles`など）を解析します。`.castnowrc`ファイルからの設定も読み込みます。
*   **Chromecastとの連携:** `chromecast-player`ライブラリを介してChromecastデバイスを検出・接続し、メディアの再生、一時停止、音量調整、シークなどの操作を行います。
*   **プレイリストの管理:** コマンドライン引数で指定されたメディアファイルやURLをプレイリストとして管理します。`--loop`や`--shuffle`オプションによる再生順序の制御も行います。
*   **プラグインの統合:** `directories.js`, `xspf.js`, `localfile.js`, `torrent.js`, `transcode.js`, `subtitles.js`, `stdin.js`といったプラグインをロードし、それぞれの機能（ディレクトリの走査、XSPFの解析、ローカルファイルの提供、torrentストリーミング、トランスコード、字幕処理、標準入力からの再生）を統合します。
*   **メディアメタデータの処理:** `music-metadata`ライブラリを使用して、オーディオファイルのメタデータ（タイトル、アーティスト、アルバムなど）を抽出し、Chromecastに表示します。カバーアートの処理には`sharp`ライブラリを使用し、Chromecastの表示要件に合わせて画像をリサイズ・変換します。
*   **ユーザーインターフェース:** `playerui`と`keypress`を使用して、コンソール上での再生状況の表示とキーボード操作による制御（再生/一時停止、音量調整、シークなど）を提供します。
*   **エラーハンドリング:** 再生中のエラーや接続切断などの問題が発生した場合に、ユーザーに通知し、適切に終了します。

## 外部ライブラリ一覧

`castnow`プロジェクトでは、以下の外部ライブラリが使用されています。

**`index.js`で直接使用されているライブラリ:**

*   `chromecast-player`: Chromecastデバイスとの通信とメディア再生を制御するためのライブラリ。([GitHub](https://github.com/xat/chromecast-player))
*   `chalk`: ターミナル出力に色やスタイルを追加するためのライブラリ。([GitHub](https://github.com/chalk/chalk))
*   `keypress`: キーボード入力を検出するためのライブラリ。([GitHub](https://github.com/dmauro/Keypress))
*   `playerui`: ターミナルベースのシンプルなUIを提供するライブラリ。([GitHub](https://github.com/player-ui/player))
*   `array-loop`: 配列をループ処理するためのユーティリティ。([GitHub](https://github.com/xat/array-loop))
*   `array-shuffle`: 配列をシャッフルするためのライブラリ。([GitHub](https://github.com/sindresorhus/array-shuffle))
*   `debounced-seeker`: シーク操作をデバウンス（連続した操作をまとめる）するためのライブラリ。([GitHub](https://github.com/xat/debounced-seeker))
*   `mime`: ファイルのMIMEタイプを検出するためのライブラリ（既に詳細を記載済み）。
*   `btoa`: 文字列をBase64エンコードするためのライブラリ。([GitHub](https://github.com/guillaumevincent/btoa))
*   `get-uri`: さまざまなURIスキーム（http, fileなど）からストリームを取得するためのライブラリ。([GitHub](https://github.com/TooTallNate/proxy-agents#readme))
*   `lodash` (`_`): JavaScriptのユーティリティ関数を提供するライブラリ。([GitHub](https://github.com/lodash/lodash))
*   `music-metadata`: オーディオファイルのメタデータ（ID3タグなど）を読み取るためのライブラリ。([GitHub](https://github.com/Borewit/music-metadata))
*   `sharp`: 高速な画像処理ライブラリ。主にカバーアートのリサイズや変換に使用されます。([GitHub](https://github.com/lovell/sharp))
*   `minimist`: コマンドライン引数を解析するためのライブラリ。([GitHub](https://github.com/minimistjs/minimist))

**プラグインで間接的に使用されているライブラリ:**

*   `diveSync`: ディレクトリを同期的に走査するためのライブラリ（`directories.js`で使用、既に詳細を記載済み）。
*   `read-torrent`: torrentファイルを読み込むためのライブラリ（`torrent.js`で使用）。([GitHub](https://github.com/mafintosh/read-torrent))
*   `peerflix`: torrentコンテンツをHTTPストリームとして提供するためのライブラリ（`torrent.js`で使用）。([GitHub](https://github.com/mafintosh/peerflix))
*   `internal-ip`: ローカルのIPアドレスを取得するためのライブラリ（複数のプラグインで使用）。([GitHub](https://github.com/sindresorhus/internal-ip))
*   `router`: シンプルなHTTPルーター（`localfile.js`で使用）。([GitHub](https://github.com/pillarjs/router))
*   `srt2vtt`: SRT字幕ファイルをVTT形式に変換するためのライブラリ（`subtitles.js`で使用）。([GitHub](https://github.com/nwoltman/srt-to-vtt-converter))
*   `got`: HTTPリクエストを行うためのライブラリ（`subtitles.js`, `transcode.js`で使用）。([GitHub](https://github.com/sindresorhus/got))
*   `stream-transcoder`: メディアストリームをトランスコードするためのライブラリ（`transcode.js`で使用）。([GitHub](https://github.com/mafintosh/stream-transcoder))
*   `xspfr`: XSPFファイルを解析するためのライブラリ（`xspf.js`で使用）。([GitHub](https://github.com/vslinko/node-xspf))



### reporting bugs/issues

Please include the debug output in your issues. You can enable the debug
messages by setting the DEBUG environment variable before running the castnow
command like this: `DEBUG=castnow* castnow ./myvideo.mp4`. Some problems have
already been addressed in our wiki https://github.com/xat/castnow/wiki.

Please only report metadata-related issues here and general issues there.

### contributors

* [dennizzzz](https://github.com/dennizzzz)
* [tooryx](https://github.com/tooryx)
* [przemyslawpluta](https://github.com/przemyslawpluta)

## License
Copyright (c) 2015 Simon Kusterer

Licensed under the MIT license.