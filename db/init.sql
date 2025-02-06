CREATE TABLE tasks (
    id INT AUTO_INCREMENT PRIMARY KEY,
    description TEXT NOT NULL,
    is_completed BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
);

INSERT INTO tasks (description, is_completed) VALUES ('task1', FALSE);
INSERT INTO tasks (description, is_completed) VALUES ('task2', TRUE);
INSERT INTO tasks (description, is_completed) VALUES ('task2', FALSE);