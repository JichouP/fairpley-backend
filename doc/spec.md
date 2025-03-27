# Fairpley Backend 仕様書

## 概要

Fairpley は、[シャープレイ値](https://ja.wikipedia.org/wiki/%E3%82%B7%E3%83%A3%E3%83%BC%E3%83%97%E3%83%AC%E3%82%A4%E5%80%A4) を用いて、
貢献度を計算し、公平な割り勘を行うサービスです。

シャープレイ値の計算には [JichouP/shapley](https://github.com/JichouP/shapley) を、割り勘の計算には [JichouP/warikan](https://github.com/JichouP/warikan) を使用します。

このアプリケーションは、Fairpley のサービスの API を提供します。
想定する動作環境は AWS Lambda です。

## 定義

### User

ユーザーを表すオブジェクトです。

### Event

割り勘をしたいイベントを表すオブジェクトです。

### Purchase

購入した物品・サービスと購入者、購入金額を表すオブジェクトです。

### Transport

移動手段を表すオブジェクトです。
移動にかかった金額を計算するのに使います。

### Location

目的地や経由地などの場所を表すオブジェクトです。
移動にかかった金額を計算するのに使います。

## API

### ユーザー管理

ログイン方法は、Google アカウントの OpenID Connect のみサポートします。

#### ログイン

```
POST /login

{
  "idToken": "idToken"
}
```

#### ログアウト

```
POST /logout
```

#### ユーザー情報取得

```
GET /user
```

#### ユーザー情報更新

```
PUT /user
```

### イベント管理

#### イベント作成

```
POST /event
```

#### イベント取得

```
GET /event
```

#### イベント更新

```
PUT /event
```

### 購入管理

#### 購入作成

```
POST /purchase
```

#### 購入取得

```
GET /purchase
```

#### 購入更新

```
PUT /purchase
```

### 移動手段管理

#### 移動手段作成

```
POST /transport
```

#### 移動手段取得

```
GET /transport
```

#### 移動手段更新

```
PUT /transport
```

### 場所管理

#### 場所作成

```
POST /location
```

#### 場所取得

```
GET /location
```
