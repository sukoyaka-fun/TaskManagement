CREATE TABLE task_categories (
  task_id INT NOT NULL,
  category_id INT NOT NULL,
  PRIMARY KEY (task_id, category_id), -- 複合主キーとして設定
  FOREIGN KEY (task_id) REFERENCES tasks(id), 
  FOREIGN KEY (category_id) REFERENCES categories(id)
);
