-- Add up migration script here

CREATE TABLE events
(
  id         uuid        NOT NULL,
  created_at timestamptz NOT NULL,
  updated_at timestamptz NOT NULL,
  name       text        NOT NULL,
  started_at timestamptz NOT NULL,
  ended_at   timestamptz NOT NULL,
  PRIMARY KEY (id)
);

COMMENT ON TABLE events IS 'イベントテーブル';

COMMENT ON COLUMN events.id IS 'Event ID';

COMMENT ON COLUMN events.created_at IS '作成日時';

COMMENT ON COLUMN events.updated_at IS '更新日時';

COMMENT ON COLUMN events.name IS 'イベント名';

COMMENT ON COLUMN events.started_at IS '開始日時';

COMMENT ON COLUMN events.ended_at IS '終了日時';

CREATE TABLE locations
(
  id         uuid        NOT NULL,
  created_at timestamptz NOT NULL,
  updated_at timestamptz NOT NULL,
  name       text        NOT NULL,
  type       text        NOT NULL,
  lat        float8      NOT NULL,
  lng        float8      NOT NULL,
  PRIMARY KEY (id)
);

COMMENT ON TABLE locations IS 'Locationテーブル';

COMMENT ON COLUMN locations.id IS 'Location ID';

COMMENT ON COLUMN locations.created_at IS '作成日時';

COMMENT ON COLUMN locations.updated_at IS '更新日時';

COMMENT ON COLUMN locations.name IS 'Location 名';

COMMENT ON COLUMN locations.type IS 'campsite, home, other, store';

COMMENT ON COLUMN locations.lat IS '緯度';

COMMENT ON COLUMN locations.lng IS '経度';

CREATE TABLE purchases
(
  id         uuid        NOT NULL,
  created_at timestamptz NOT NULL,
  updated_at timestamptz NOT NULL,
  user_id    uuid        NOT NULL,
  event_id   uuid        NOT NULL,
  name       text        NOT NULL,
  price      integer     NOT NULL,
  PRIMARY KEY (id)
);

COMMENT ON TABLE purchases IS '購入履歴';

COMMENT ON COLUMN purchases.id IS 'Purchase ID';

COMMENT ON COLUMN purchases.created_at IS '作成日時';

COMMENT ON COLUMN purchases.updated_at IS '更新日時';

COMMENT ON COLUMN purchases.user_id IS 'User ID';

COMMENT ON COLUMN purchases.event_id IS 'Event ID';

COMMENT ON COLUMN purchases.name IS '購入品目';

COMMENT ON COLUMN purchases.price IS '金額';

CREATE TABLE transports
(
  id         uuid        NOT NULL,
  created_at timestamptz NOT NULL,
  updated_at timestamptz NOT NULL,
  name       text        NOT NULL,
  type       text        NOT NULL,
  PRIMARY KEY (id)
);

COMMENT ON TABLE transports IS '移動手段テーブル';

COMMENT ON COLUMN transports.id IS 'Transport ID';

COMMENT ON COLUMN transports.created_at IS '作成日時';

COMMENT ON COLUMN transports.updated_at IS '更新日時';

COMMENT ON COLUMN transports.name IS '移動手段の名称';

COMMENT ON COLUMN transports.type IS 'car,motorcycle,rental';

CREATE TABLE users
(
  id         uuid        NOT NULL,
  created_at timestamptz NOT NULL,
  updated_at timestamptz NOT NULL,
  name       text        NOT NULL,
  PRIMARY KEY (id)
);

COMMENT ON TABLE users IS 'Userテーブル';

COMMENT ON COLUMN users.id IS 'User ID';

COMMENT ON COLUMN users.created_at IS '作成日時';

COMMENT ON COLUMN users.updated_at IS '更新日時';

COMMENT ON COLUMN users.name IS 'ユーザー名';

ALTER TABLE purchases
  ADD CONSTRAINT FK_events_TO_purchases
    FOREIGN KEY (event_id)
    REFERENCES events (id);

ALTER TABLE purchases
  ADD CONSTRAINT FK_users_TO_purchases
    FOREIGN KEY (user_id)
    REFERENCES users (id);
