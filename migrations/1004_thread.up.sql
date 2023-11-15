CREATE TABLE thread (
    id SERIAL PRIMARY KEY,
    thread_content_id INT,
    category_id SERIAL REFERENCES category (id),
    author_id INT NOT NULL REFERENCES account (id),
    -- For moderation purposes, when the thread is edited or deleted it still remains in the database
    is_deleted BOOLEAN NOT NULL DEFAULT false,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE TABLE thread_content (
    id SERIAL PRIMARY KEY,
    thread_id INT NOT NULL REFERENCES thread (id),
    title VARCHAR(255) NOT NULL,
    body TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
ALTER TABLE thread
ADD FOREIGN KEY (thread_content_id) REFERENCES thread_content (id);