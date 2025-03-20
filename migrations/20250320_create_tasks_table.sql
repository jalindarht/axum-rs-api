CREATE TABLE tasks (
    id BIGINT IDENTITY(1, 1) PRIMARY KEY,
    title VARCHAR(255) NOT NULL,
    description TEXT,
    status VARCHAR(50) NOT NULL
);