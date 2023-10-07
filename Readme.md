# Analyticsサーバー

## このアプリは何をするものなのか

webアプリケーション上でユーザーの行動した内容を記録すること

## 起動方法

### コードの取得

```bash
git clone https://github.com/yashiro-ryo/analytics-server
```

### ビルド

```bash
cargo build
```

### 実行

```bash
cargo run
```

## エンドポイントの仕様(予定)

### イベント登録用

POST /api/v1/register

```rust
{
  uid: String,
  event_name: String,
  event_detail: String
}
```

### データ取得用

GET /api/v1/result

```rust
{
  access_token: String
}
```