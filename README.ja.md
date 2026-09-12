# herdr

<p align="center">
  <img src="assets/logo.png" alt="herdr" width="100" />
</p>

<p align="center">
  <a href="https://herdr.dev">herdr.dev</a> · <a href="#install">インストール</a> · <a href="https://herdr.dev/docs/quick-start/">クイックスタート</a> · <a href="https://herdr.dev/docs/">ドキュメント</a>
</p>

<p align="center">
  <a href="README.md">English</a> · <a href="README.zh-CN.md">简体中文</a> · 日本語
</p>

<p align="center">
  <a href="LICENSE"><img src="https://img.shields.io/badge/license-Apache--2.0-666666?labelColor=333333" alt="Apache 2.0 license" /></a>
  <a href="https://github.com/herdrdev/herdr/releases"><img src="https://img.shields.io/github/downloads/herdrdev/herdr/total?labelColor=333333&color=666666" alt="total GitHub release downloads" /></a>
  <a href="https://github.com/herdrdev/herdr/stargazers"><img src="https://img.shields.io/github/stars/herdrdev/herdr?labelColor=333333&color=666666&logo=github" alt="GitHub stars" /></a>
  <a href="https://github.com/herdrdev/herdr/releases/latest"><img src="https://img.shields.io/github/v/release/herdrdev/herdr?label=release&labelColor=333333&color=666666" alt="latest stable release" /></a>
  <a href="https://formulae.brew.sh/formula/herdr"><img src="https://img.shields.io/homebrew/v/herdr?label=homebrew&labelColor=333333&color=666666" alt="Homebrew version" /></a>
  <a href="https://x.com/herdrdev"><img src="https://img.shields.io/badge/follow-%40herdrdev-000000?logo=x&logoColor=white" alt="follow @herdrdev on X" /></a>
</p>

---

https://github.com/user-attachments/assets/043ec09f-4bdd-41d5-aee0-8fda6b83e267

**コーディングエージェントが動作するためのランタイム。**

- **常に実行中** — herdrはバックグラウンドサーバーであり、ターミナルはその内部で動作します。ノートパソコンを閉じても、ネットワークが切れても、マシンを再起動しても、エージェントは働き続け、セッションは復元されます。どのターミナルからでも、あるいはSSH経由でも再接続可能です。
- **行き詰まったエージェントを探す必要はありません** — すべてのペインには「working（作業中）」「blocked（ブロック中）」「idle（待機中）」のいずれかがマークされます。エージェントが停止し、回答が必要な場合はherdrが知らせてくれます。
- **エージェントネイティブ** — エージェントはCLIやソケットAPIを通じてherdrを操作します。ペインを生成し、お互いにプロンプトを送り、他のエージェントが本当にブロックされるまで待機することができます。[エージェントスキル →](https://herdr.dev/docs/agent-skill/)
- **既存のツールをそのまま実行** — claude code、codex、cursor、opencode、grokなど。herdrはそれらをラップしたり置き換えたりするのではなく、それらのターミナルを所有するだけです。
- **キーボードとマウス、両方が第一級クラス** — tmuxスタイルのプレフィックスキー**と**、クリック、ドラッグ、分割機能。ツールごとではなく、その時々に合わせて使い分けられます。
- **プラグイン** — ペインやワークフローを拡張します。[マーケットプレイスを見る →](https://herdr.dev/plugins/)
- **1つのRustバイナリ、Electron不要** — 現在使っているどのターミナル環境でも動作します。

---

## インストール

```bash
curl -fsSL https://herdr.dev/install.sh | zsh
```

または `brew install herdr` · `mise use -g herdr` · Windows: `powershell -ExecutionPolicy Bypass -c "irm https://herdr.dev/install.ps1 | iex"` · [エンドポイント保護されたWindows](https://herdr.dev/docs/windows-beta/) · [バイナリ](https://github.com/herdrdev/herdr/releases)

その後、作業場所で起動します：

```bash
herdr
```

エージェントを実行し、ペインを分割して、そのままにしておけます。`ctrl+b q` でデタッチし、`herdr` で再アタッチします。[クイックスタート →](https://herdr.dev/docs/quick-start/)

## ドキュメント

すべては [herdr.dev/docs](https://herdr.dev/docs/) にあります: [クイックスタート](https://herdr.dev/docs/quick-start/) · [コンセプト](https://herdr.dev/docs/concepts/) · [サポートされているエージェント](https://herdr.dev/docs/agents/) · [キーボード](https://herdr.dev/docs/keyboard/) · [設定](https://herdr.dev/docs/configuration/) · [セッション状態](https://herdr.dev/docs/session-state/) · [リモート](https://herdr.dev/docs/persistence-remote/) · [インテグレーション](https://herdr.dev/docs/integrations/) · [プラグイン](https://herdr.dev/docs/plugins/) · [ソケットAPI](https://herdr.dev/docs/socket-api/)

## 感謝

過去のスポンサーと支援者の皆様は [SPONSORS.md](./SPONSORS.md) に記載されています — ありがとうございます 🐑

エンタープライズ / パートナーシップ: hey@herdr.dev

## エージェントへの指示

あなたがこのリポジトリの改善を手伝うAIエージェントである場合、変更を加える前に [`AGENTS.md`](./AGENTS.md) を読み、IssueやPRを開く前に [`CONTRIBUTING.md`](./CONTRIBUTING.md) を読んでください。

## 開発

```bash
git clone https://github.com/herdrdev/herdr
cd herdr
cargo build --release

just test        # ユニットテスト
just check       # フォーマット、テスト、およびメンテナンスチェック
```

## ライセンス

Herdr は [Apache License 2.0](LICENSE) の下でライセンスされています。
