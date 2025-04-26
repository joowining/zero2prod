-- Add migration script here
-- migrations/{timestamp}_create_subscriptions_table.sql
-- subscripttions 테이블을 생성한다.
CREATE TABLE subscriptions(
    id uuid NOT NULL,
    PRIMARY KEY (id),
    email TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    subscribed_at timestamptz NOT NULL
);
