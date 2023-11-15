CREATE TABLE post (
    id SERIAL PRIMARY KEY,
    author_id INT NOT NULL REFERENCES account (id),
    thread INT NOT NULL REFERENCES thread (id),
    is_deleted BOOLEAN,
    post_content_id INT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE TABLE post_content (
    id SERIAL PRIMARY KEY REFERENCES post (id),
    post_id INT NOT NULL REFERENCES post (id),
    contents TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
ALTER TABLE post ADD FOREIGN KEY (post_content_id) REFERENCES post_content (id);