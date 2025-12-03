# UI

This crate contains all shared components for the workspace. This is a great place to place any UI you would like to use in multiple platforms like a common `Button` or `Navbar` component.

> [!IMPORTANT]
> **コンポーネントの構成について**
>
> 汎用的なUIコンポーネントやDioxus公式コンポーネントは `packages/ui/src/components` に格納します。
> `packages/ui/src` 直下には、**アプリ専用のコンポーネント**（特定のビジネスロジックを含むものや、アプリ専用のデザインが強く適用されたもの）を格納してください。
> 可能な限り `src/components` にあるものを使用することを推奨します。

## コンポーネントのデモ

各コンポーネントの使用例は `packages/ui/src/demo` ディレクトリに格納されています。
コンポーネントを使用する際は、対応するdemoファイルを参照してください。

利用可能なデモ:
- `demo/button.rs` - Buttonコンポーネントの各バリアント
- `demo/card.rs` - Cardコンポーネントの使用例
- `demo/input.rs` - Inputコンポーネントの使用例
- `demo/label.rs` - Labelコンポーネントの使用例
- `demo/navbar.rs` - Navbarコンポーネントの使用例
- `demo/switch.rs` - Switchコンポーネントの使用例
- `demo/tabs.rs` - Tabsコンポーネントの使用例

すべてのデモは `ui::demo::Demo` コンポーネントから確認できます。

```
ui/
├─ src/
│  ├─ lib.rs # The entrypoint for the ui crate
│  ├─ hero.rs # The Hero component that will be used in every platform
│  ├─ echo.rs # The shared echo component that communicates with the server
│  ├─ navbar.rs # The Navbar component that will be used in the layout of every platform's router
```

## Dependencies

Since this crate is shared between multiple platforms, it should not pull in any platform specific dependencies. For example, if you want to use the `web_sys` crate in the web build of your app, you should not add it to this crate. Instead, you should add platform specific dependencies to the [web](../web/Cargo.toml), [desktop](../desktop/Cargo.toml), or [mobile](../mobile/Cargo.toml) crates.

## UI Component Library

### 使用するライブラリ

基本的なUIコンポーネント(Button, Toggle, Modalなど)には **[Dioxus Components](https://github.com/DioxusLabs/dioxus-std)** を使用します。これにより、統一感のあるUIを効率的に構築できます。

カスタムコンポーネントは、アプリケーション固有の機能を実装するために作成します。

---

## 必要なUIコンポーネント

このアプリケーションを実装するために必要なUIコンポーネントのリストです。

### 🎯 コアコンポーネント(優先度: 高)

#### 設定関連
- **`SettingsCard`** - アプリの基本設定を行うカード
  - 就寝時刻の設定
    - 時刻選択UI(時・分)
    - 有効/無効の切り替え
  - 通知方法の設定
    - 通知方法のチェックボックス(メッセージ、バイブレーション、音)
    - プラットフォーム別の利用可能な通知方法の表示
    - プレビュー機能
  - 設定の保存・読み込み

- **`BotSelector`** - ボットタイプを選択するコンポーネント
  - ボットタイプのリスト表示(機械タイプ、おかんタイプ)
  - 各ボットの説明表示
  - 選択状態の管理

#### 通知関連
- **`NotificationCard`** - 催促メッセージを表示するカード
  - メッセージ本文の表示
  - 閉じるボタン
  - アニメーション効果

- **`NotificationHistory`** - 通知履歴を表示するリスト
  - 日時とメッセージの表示
  - フィルタリング機能(日付、ボットタイプ)

### 🎨 レイアウト・ナビゲーション(優先度: 高)

- **`AppLayout`** - アプリ全体のレイアウト
  - ヘッダー
  - メインコンテンツエリア
  - フッター(必要に応じて)

- **`TabNavigation`** - タブナビゲーション
  - ホーム、設定、履歴などのタブ切り替え
  - アクティブタブの表示

### 📊 ダッシュボード(優先度: 中)

- **`SleepStatusCard`** - 現在の睡眠状態を表示
  - 現在時刻
  - 設定時刻までの残り時間
  - 今日の就寝予定時刻

- **`StatisticsCard`** - 統計情報を表示
  - 今週の就寝時刻の平均
  - 目標達成率
  - グラフ表示(オプション)

### 🤖 ボット個別設定(優先度: 中)

- **`BotConfigCard`** - 各ボットの詳細設定
  - 催促タイミングの設定(間隔、ランダム範囲など)
  - メッセージのカスタマイズ
  - ボット固有の設定項目

- **`MessagePreview`** - メッセージのプレビュー
  - 実際の通知メッセージの表示
  - サンプルメッセージの切り替え

### 🎭 UI共通パーツ(優先度: 低)

> **Note**: 以下のコンポーネントは基本的に **Dioxus Components** を使用します。
> カスタマイズが必要な場合のみ、独自実装を検討してください。

- **`Button`** - 汎用ボタンコンポーネント *(Dioxus Components使用)*
  - プライマリ、セカンダリ、危険などのバリエーション
  - サイズバリエーション

- **`Card`** - 汎用カードコンポーネント *(Dioxus Components使用)*
  - タイトル、本文、アクションエリア
  - シャドウやボーダーのスタイル

- **`Toggle`** - トグルスイッチ *(Dioxus Components使用)*
  - ON/OFF状態の切り替え
  - ラベル表示

- **`TimePicker`** - 時刻選択コンポーネント *(カスタム実装)*
  - 時・分の選択UI
  - 12時間/24時間表示の切り替え

- **`Modal`** - モーダルダイアログ *(Dioxus Components使用)*
  - 確認ダイアログ
  - 設定ダイアログ
  - オーバーレイ

### 🔔 プラットフォーム固有(優先度: 低)

> **Note**: これらのコンポーネントは各プラットフォームのcrateに配置します。

- **`AndroidNotification`** - Android固有の通知UI
  - バイブレーション制御
  - システム通知との連携

- **`DesktopTray`** - デスクトップのシステムトレイUI
  - トレイアイコン
  - クイック設定メニュー
