CREATE TABLE tasks (
  id SERIAL PRIMARY KEY,
  name VARCHAR(50) NOT NULL,
  description TEXT NOT NULL DEFAULT '詳細なし',
  status VARCHAR(10) NOT NULL CHECK (status IN ('未着手', '進行中', '完了', '保留')),
  user_id INT,
  FOREIGN KEY (user_id) REFERENCES users(id)
);
