# 開発ガイドライン

hayonerobotプロジェクトへの貢献ありがとうございます!このドキュメントでは、開発を始めるための情報と、コーディング規約について説明します。

## 目次

- [開発環境のセットアップ](#開発環境のセットアップ)
- [プロジェクト構造](#プロジェクト構造)
- [開発ワークフロー](#開発ワークフロー)
- [コーディング規約](#コーディング規約)
- [テスト](#テスト)
- [コミットメッセージ](#コミットメッセージ)
- [LLMを使った開発](#llmを使った開発)

## 開発環境のセットアップ

### 必要なツール

1. **Rust** (最新の安定版)
   ```bash
   # Rustのインストール
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **Dioxus CLI**
   ```bash
   # Dioxus CLIのインストール
   curl -sSL http://dioxus.dev/install.sh | sh
   ```

3. **プラットフォーム別の追加ツール**

   **Android開発の場合:**
   - Android Studio
   - Android SDK
   - NDK

   **Windows Desktop開発の場合:**
   - Visual Studio Build Tools
   - Windows SDK

### プロジェクトのクローン

```bash
git clone https://github.com/yourusername/hayonerobot.git
cd hayonerobot
```

### 依存関係のインストール

```bash
# Cargoが自動的に依存関係を解決します
cargo build
```

## プロジェクト構造

```
hayonerobot/
├── packages/
│   ├── web/           # Webアプリケーション
│   ├── desktop/       # Desktopアプリケーション
│   ├── mobile/        # Mobileアプリケーション
│   ├── ui/            # 共有UIコンポーネント
│   └── api/           # 将来のサーバー機能用
├── docs/              # ドキュメント
│   ├── architecture.md
│   └── bot-specifications.md
├── README.md
├── CONTRIBUTING.md    # このファイル
└── AGENTS.md          # Dioxus 0.7リファレンス
```

詳細は[architecture.md](docs/architecture.md)を参照してください。

## 開発ワークフロー

### 1. ブランチ戦略

- `main`: 安定版
- `develop`: 開発版
- `feature/xxx`: 新機能開発
- `fix/xxx`: バグ修正
- `docs/xxx`: ドキュメント更新

### 2. 新機能の開発手順

```bash
# developブランチから新しいブランチを作成
git checkout develop
git pull origin develop
git checkout -b feature/your-feature-name

# 開発作業...

# コミット
git add .
git commit -m "feat: add your feature"

# プッシュ
git push origin feature/your-feature-name

# Pull Requestを作成
```

### 3. 開発サーバーの起動

```bash
# Web
dx serve --package web --platform web

# Desktop
dx serve --package desktop --platform desktop

# Mobile (Android)
dx serve --package mobile --platform android
```

### 4. ビルド確認

```bash
# リリースビルドを試す
dx build --package web --platform web --release
```

## コーディング規約

### Rustコード

#### 1. フォーマット

```bash
# コードフォーマット
cargo fmt

# フォーマットチェック
cargo fmt -- --check
```

#### 2. Linting

```bash
# Clippy実行
cargo clippy -- -D warnings

# すべてのワーニングを修正してからコミット
```

#### 3. 命名規則

- **関数・変数**: `snake_case`
  ```rust
  fn send_notification() { }
  let user_name = "Alice";
  ```

- **型・構造体**: `PascalCase`
  ```rust
  struct BotConfig { }
  enum BotType { }
  ```

- **定数**: `SCREAMING_SNAKE_CASE`
  ```rust
  const MAX_RETRY_COUNT: u32 = 3;
  ```

- **コンポーネント**: `PascalCase`
  ```rust
  #[component]
  fn SettingsPanel() -> Element { }
  ```

#### 4. コメント

- 複雑なロジックには必ずコメントを追加
- 公開API(pub)には必ずドキュメントコメントを追加

```rust
/// ボットの設定を保存します
///
/// # Arguments
/// * `config` - 保存するボット設定
///
/// # Returns
/// 成功時は`Ok(())`、失敗時はエラー
pub fn save_bot_config(config: &BotConfig) -> Result<(), Error> {
    // 実装...
}
```

### Dioxus特有の規約

#### 1. コンポーネント

```rust
// ✅ 良い例
#[component]
fn NotificationCard(message: String, timestamp: String) -> Element {
    rsx! {
        div { class: "notification-card",
            p { "{message}" }
            span { class: "timestamp", "{timestamp}" }
        }
    }
}

// ❌ 悪い例(propsに参照を使わない)
#[component]
fn NotificationCard(message: &str) -> Element { // &strは使わない
    // ...
}
```

#### 2. 状態管理

```rust
// ✅ 良い例
#[component]
fn Counter() -> Element {
    let mut count = use_signal(|| 0);
    
    rsx! {
        button { onclick: move |_| *count.write() += 1, "Increment" }
        p { "Count: {count}" }
    }
}

// ❌ 悪い例(古いDioxus 0.6以前の書き方)
fn Counter(cx: Scope) -> Element { // Scopeは使わない
    let count = use_state(cx, || 0); // use_stateは使わない
    // ...
}
```

#### 3. RSXのフォーマット

```rust
// ✅ 良い例(読みやすい)
rsx! {
    div { class: "container",
        h1 { "Title" }
        p { class: "description",
            "This is a description"
        }
        button { 
            onclick: move |_| handle_click(),
            "Click me" 
        }
    }
}

// ❌ 悪い例(読みにくい)
rsx! { div { class: "container", h1 { "Title" } p { class: "description", "This is a description" } button { onclick: move |_| handle_click(), "Click me" } } }
```

### ファイル構成

#### 1. モジュール構成

```rust
// lib.rs または main.rs
mod components;
mod utils;
mod types;

pub use components::*;
pub use types::*;
```

#### 2. コンポーネントファイル

```rust
// components/notification_card.rs
use dioxus::prelude::*;

#[component]
pub fn NotificationCard(message: String) -> Element {
    rsx! {
        div { class: "notification-card",
            "{message}"
        }
    }
}
```

## テスト

### ユニットテスト

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_bot_message_generation() {
        let bot = BotType::Robot;
        let message = bot.get_message(1);
        assert!(!message.is_empty());
    }
}
```

### テストの実行

```bash
# すべてのテストを実行
cargo test

# 特定のパッケージのみ
cargo test --package ui

# 詳細出力
cargo test -- --nocapture
```

## コミットメッセージ

[Conventional Commits](https://www.conventionalcommits.org/)形式を使用します。

### フォーマット

```
<type>(<scope>): <subject>

<body>

<footer>
```

### Type

- `feat`: 新機能
- `fix`: バグ修正
- `docs`: ドキュメントのみの変更
- `style`: コードの意味に影響しない変更(フォーマットなど)
- `refactor`: リファクタリング
- `test`: テストの追加・修正
- `chore`: ビルドプロセスやツールの変更

### 例

```bash
# 新機能
git commit -m "feat(bot): add Okan bot type with random intervals"

# バグ修正
git commit -m "fix(notification): resolve notification not showing on Android"

# ドキュメント
git commit -m "docs: update architecture diagram"

# リファクタリング
git commit -m "refactor(ui): extract notification logic to separate module"
```

## LLMを使った開発

このプロジェクトはLLM(ChatGPT、Claude、Geminiなど)を使った開発を想定しています。

### LLMに指示を出す際のベストプラクティス

#### 1. 関連ドキュメントを参照させる

```
「docs/bot-specifications.mdに従って、おかんボットの
メッセージ生成ロジックを実装してください」
```

#### 2. 具体的な要件を伝える

```
「packages/ui/src/に新しいコンポーネント NotificationCard を作成してください。
要件:
- propsとしてmessage(String)とtimestamp(String)を受け取る
- Dioxus 0.7のuse_signalを使用
- CSSクラスは notification-card を使用」
```

#### 3. プロジェクトの制約を明示

```
「Dioxus 0.7を使用しているので、use_stateやScopeは使わないでください。
AGENTS.mdを参照して実装してください」
```

### 参照すべきドキュメント

LLMに作業を依頼する際は、以下のドキュメントを参照させると効果的です:

- **[README.md](README.md)**: プロジェクト概要と基本情報
- **[docs/architecture.md](docs/architecture.md)**: 技術的なアーキテクチャ
- **[docs/bot-specifications.md](docs/bot-specifications.md)**: ボットの振る舞い仕様
- **[AGENTS.md](AGENTS.md)**: Dioxus 0.7のリファレンス
- **このファイル(CONTRIBUTING.md)**: 開発ガイドライン

### LLMへの指示例

```
「hayonerobotプロジェクトに新しい機能を追加したいです。

参照ドキュメント:
- docs/architecture.md
- docs/bot-specifications.md

タスク:
packages/ui/src/に、ボットアバターを表示するコンポーネントを作成してください。

要件:
1. コンポーネント名: BotAvatar
2. props: bot_type (BotType enum)
3. bot_typeに応じて異なるアイコンを表示
4. Dioxus 0.7の規約に従う(AGENTS.md参照)
5. コメントは日本語で記述

実装後、packages/ui/src/lib.rsに追加してください。」
```

## よくある質問

### Q: Dioxus 0.6以前のコード例を見つけたが、使えますか?

A: いいえ。このプロジェクトはDioxus 0.7を使用しており、APIが大きく変更されています。必ず[AGENTS.md](AGENTS.md)または[公式ドキュメント](https://dioxuslabs.com/learn/0.7)を参照してください。

### Q: 新しいボットタイプを追加したい

A: [docs/bot-specifications.md](docs/bot-specifications.md)の「今後の拡張予定」セクションを参考に、同じパターンで実装してください。

### Q: プラットフォーム固有の機能を追加したい

A: 該当するプラットフォームのパッケージ(`web`, `desktop`, `mobile`)に実装してください。共通化できる部分は`ui`パッケージに移動を検討してください。

## サポート

質問や問題がある場合は、GitHubのIssuesで報告してください。

## ライセンス

このプロジェクトのライセンスについては、LICENSEファイルを参照してください。

---

Happy Coding! 🚀
