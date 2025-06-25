# TowTruck Manager Pro - パフォーマンスチューニングプロジェクト

## 概要

このプロジェクトは、42TokyoとDreamArts主催の「Tuning the backend Contest 2024」で使用されたレッカー車配車管理システムです。レッカー車の配車要請を効率的に処理するためのWebアプリケーションで、パフォーマンス最適化を目的としたコンテスト課題として提供されています。

## アプリケーション構成

### アーキテクチャ
- **フロントエンド**: Next.js 14 (TypeScript, React)
- **バックエンド**: Rust (Actix Web)
- **データベース**: MySQL
- **Webサーバー**: Nginx
- **インフラ**: Docker Compose

### 主要機能
- ユーザー認証（顧客、ディスパッチャー、ドライバー）
- 配車依頼管理
- レッカー車の最適配車アルゴリズム
- リアルタイムダッシュボード
- グラフベースの位置情報管理

## プロジェクト構造

```
.
├── benchmarker/     # パフォーマンステスト・スコアリングツール
├── document/        # 競技ドキュメント・API仕様書
├── webapp/          # メインアプリケーション
│   ├── backend/     # Rust APIサーバー
│   ├── frontend/    # Next.js Webアプリ
│   ├── mysql/       # データベース設定・初期データ
│   └── nginx/       # Webサーバー設定
├── init.sh          # 環境初期化スクリプト
└── run.sh           # アプリケーション起動スクリプト
```

## セットアップ方法

### 環境要件
- Docker & Docker Compose
- Git

### 初期セットアップ
```bash
# リポジトリクローン
git clone <repository-url>
cd 42Tokyo-Tuning-2407

# 環境初期化
./init.sh

# アプリケーション起動
./run.sh
```

### アクセス情報
- **フロントエンド**: http://localhost
- **ログイン情報**: 
  - ユーザー名: `dispatcher1_1` (ディスパッチャー例)
  - パスワード: `password`

## 開発・運用

### スクリプト
- `init.sh`: 初期環境構築
- `run.sh`: アプリケーション起動
- `restart_container.sh`: コンテナ再起動
- `get_test_status.sh`: テスト状況確認

### ベンチマーク実行
```bash
cd benchmarker
./run_k6_and_score.sh
```

## 技術仕様

### バックエンド (Rust)
- **フレームワーク**: Actix Web 4.6
- **データベース**: SQLx (MySQL)
- **認証**: Argon2パスワードハッシュ
- **レート制限**: Actix Governor

### フロントエンド (TypeScript)
- **フレームワーク**: Next.js 14
- **UIライブラリ**: Material-UI
- **HTTP通信**: Axios
- **日時処理**: Day.js

### データベース設計
- ユーザー管理（顧客、ディスパッチャー、ドライバー）
- 配車依頼管理
- レッカー車情報管理
- グラフ構造（ノード、エッジ、エリア）

## パフォーマンス最適化ポイント

### 実装済み最適化
- SQLクエリの最適化
- 最短経路探索アルゴリズム（ダイクストラ法）
- 画像リサイズ・圧縮
- データベースインデックス最適化

### 改善可能領域
- キャッシュ戦略
- 非同期処理最適化
- フロントエンドバンドルサイズ削減
- データベース接続プール調整

## ドキュメント

詳細な仕様書・手順書は以下を参照：
- [競技ドキュメント](./document/README.md)
- [API仕様書](./document/api-specs/openapi.yaml)
- [アプリケーション詳細](./document/md/app/)

## ライセンス

MIT License
