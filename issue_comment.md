### 最終実装計画

Issue #5 の実装計画と、提供された設定管理に関する詳細な技術レポート（serde, config クレートの活用）を基に、以下の最終実装計画を提案します。この計画は、堅牢性、保守性、拡張性を重視しています。

#### 1. 設定管理の統一戦略

すべての設定値（コマンドライン引数、設定ファイル、環境変数）を、単一の `Settings` 構造体で一元管理します。

- **CLI引数**: `clap` クレートの `derive` マクロを用いてパースします。
- **設定ファイル (`.castnowrc`)**: `config` クレートを用いて TOML 形式のファイルを読み込みます。
- **環境変数**: `config` クレートの `Environment` ソースを用いて、プレフィックス付き（例: `CASTNOW_`）の環境変数を読み込みます。
- **シリアライズ/デシリアライズ**: `serde` クレートを用いて、上記ソースから `Settings` 構造体への変換を型安全に行います。

#### 2. 設定の優先順位

以下の優先順位で設定をマージし、ユーザーが柔軟に設定を上書きできるようにします。

1.  **コマンドライン引数** (最優先)
2.  **環境変数**
3.  **ユーザー設定ファイル** (例: `~/.config/castnow/config.toml` または `.castnowrc`)
4.  **コンパイル時に埋め込まれたデフォルト値** (最下位)

#### 3. 実装ステップ

**ステップ 1: 依存関係の追加**

`Cargo.toml` に以下のクレートを追加します。
```toml
[dependencies]
clap = { version = "4.0", features = ["derive"] }
config = { version = "0.14", features = ["toml"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
anyhow = "1.0"
# ... その他必要なクレート
```

**ステップ 2: `Settings` 構造体の定義 (`src/settings.rs`)**

アプリケーションのすべての設定を保持する `Settings` 構造体を定義します。`clap` と `serde` のアトリビュートを併用して、CLI、ファイル、環境変数のすべてに対応させます。

```rust
// src/settings.rs
use clap::Parser;
use serde::Deserialize;

#[derive(Parser, Deserialize, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Settings {
    #[arg(long, short)]
    pub device: Option<String>,

    #[arg(long)]
    pub subtitles: Option<String>,

    #[arg(long)]
    pub quiet: bool,

    // ... 他のすべてのオプションを同様に定義 ...
}
```

**ステップ 3: 階層的設定読み込みの実装 (`src/config.rs`)**

`config` クレートを利用して、ファイルと環境変数から設定を読み込み、マージするロジックを実装します。

```rust
// src/config.rs
use config::{Config, File, Environment};
use crate::settings::Settings;

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let settings = Config::builder()
        // 1. デフォルト値 (コード内で設定する場合)
        // .set_default("quiet", false)?

        // 2. .castnowrc ファイルから読み込み (存在しなくてもOK)
        .add_source(File::with_name(".castnowrc").required(false))
        
        // 3. 環境変数から読み込み (プレフィックス: CASTNOW)
        .add_source(Environment::with_prefix("CASTNOW").separator("__"))
        
        .build()?;

    settings.try_deserialize()
}
```

**ステップ 4: `main.rs` での設定のマージと適用**

起動時に `clap` でCLI引数をパースし、`config.rs` で読み込んだ設定とマージして、最終的な `Settings` オブジェクトを構築します。

```rust
// src/main.rs
mod settings;
mod config;

use clap::Parser;
use settings::Settings;

fn main() -> anyhow::Result<()> {
    let cli_settings = Settings::parse();
    let file_and_env_settings = config::get_configuration()?;

    // 手動で設定をマージ
    // (より洗練されたマージ戦略を検討する可能性あり)
    let final_settings = Settings {
        device: cli_settings.device.or(file_and_env_settings.device),
        subtitles: cli_settings.subtitles.or(file_and_env_settings.subtitles),
        quiet: cli_settings.quiet || file_and_env_settings.quiet,
        // ... 他のすべてのオプションをマージ ...
    };

    println!("最終的な設定: {:?}", final_settings);

    // ここから final_settings を使ってアプリケーションの主処理を開始
    // run_application(final_settings)?;

    Ok(())
}
```

この計画に沿って実装を進めることで、Issue #5 で要求されている豊富なコマンドラインオプションを、メンテナンスしやすく、かつユーザーにとって使いやすい形で提供できると考えています。