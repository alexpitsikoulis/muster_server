CREATE TABLE confirmation_tokens(
    confirmation_token TEXT NOT NULL,
    user_id uuid NOT NULL REFERENCES users(id),
    PRIMARY KEY(confirmation_token) 
);