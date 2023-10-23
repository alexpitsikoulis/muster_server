CREATE TABLE users (
    id uuid NOT NULL,
    PRIMARY KEY(id),
    email VARCHAR(50) NOT NULL UNIQUE,
    name VARCHAR(50) NOT NULL,
    password TEXT NOT NULL,
    profile_photo TEXT,
    bio TEXT,
    is_locked BOOLEAN NOT NULL DEFAULT false,
    failed_attempts SMALLINT NOT NULL DEFAULT 0,
    created_at timestamptz NOT NULL DEFAULT now(),
    updated_at timestamptz NOT NULL DEFAULT now(),
    deleted_at timestamptz
);

CREATE TABLE servers (
    id uuid NOT NULL,
    PRIMARY KEY(id),
    name VARCHAR(50) NOT NULL,
    owner_id uuid REFERENCES users(id),
    description TEXT,
    photo TEXT,
    cover_photo TEXT,
    created_at timestamptz NOT NULL DEFAULT now(),
    updated_at timestamptz NOT NULL DEFAULT now(),
    deleted_at timestamptz
);

CREATE TABLE server_members (
    server_id uuid REFERENCES servers(id),
    user_id uuid REFERENCES users(id),
    PRIMARY KEY(server_id, user_id),
    is_admin BOOLEAN DEFAULT false,
    is_banned BOOLEAN DEFAULT false,
    joined_at timestamptz NOT NULL DEFAULT now()
);

CREATE TABLE group_chats (
    id uuid NOT NULL,
    PRIMARY KEY(id),
    server_id uuid REFERENCES servers(id),
    name VARCHAR(50) NOT NULL,
    description VARCHAR(255),
    photo TEXT,
    voice_enabled BOOLEAN,
    created_at timestamptz NOT NULL DEFAULT now(),
    updated_at timestamptz NOT NULL DEFAULT now(),
    deleted_at timestamptz
);

CREATE TABLE posts (
    id uuid NOT NULL,
    PRIMARY KEY(id),
    user_id uuid REFERENCES users(id),
    server_id uuid REFERENCES servers(id),
    content TEXT NOT NULL,
    quoted_post_id uuid REFERENCES posts(id),
    created_at timestamptz NOT NULL DEFAULT now(),
    updated_at timestamptz NOT NULL DEFAULT now(),
    deleted_at timestamptz
);

CREATE TABLE post_likes (
    post_id uuid REFERENCES posts(id),
    user_id uuid REFERENCES users(id),
    PRIMARY KEY(post_id, user_id),
    created_at timestamptz NOT NULL DEFAULT now()
);

CREATE TABLE comments (
    id uuid NOT NULL,
    PRIMARY KEY(id),
    post_id uuid REFERENCES posts(id),
    user_id uuid REFERENCES users(id),
    comment TEXT,
    created_at timestamptz NOT NULL DEFAULT now(),
    updated_at timestamptz NOT NULL DEFAULT now(),
    deleted_at timestamptz
);

CREATE TABLE comment_likes (
    comment_id uuid REFERENCES comments(id),
    user_id uuid REFERENCES users(id),
    PRIMARY KEY(comment_id, user_id),
    created_at timestamptz NOT NULL DEFAULT now()
);
