CREATE TABLE dependencies (
  preceding_task_id INT NOT NULL, -- 先行するタスクのid
  following_task_id INT NOT NULL, -- 従属するタスクのid
  PRIMARY KEY (preceding_task_id, following_task_id), -- 複合主キーとして設定
  FOREIGN KEY (preceding_task_id) REFERENCES tasks(id), 
  FOREIGN KEY (following_task_id) REFERENCES tasks(id)
);
