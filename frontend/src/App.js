import React, { useEffect, useState } from "react";
import { fetchTasks, createTask } from "./api/tasks.js";
import { fetchUsers } from "./api/users.js";
import "./App.css";

function App() {
  const [message, setMessage] = useState();
  const [tasks, setTasks] = useState([]);

  useEffect(() => {
    // GET /users リクエスト
    fetchUsers()
      .then((res) => {
        setMessage(`Hello with ${res.length} users`); // メッセージを更新
      })
      .catch(console.error);
  }, []); // 第2引数は空の配列

  useEffect(() => {
    // GET /tasks リクエスト
    fetchTasks()
      .then((res) => {
        setTasks(res); // 取得したタスク一覧をstateにセット
        setMessage(`Loaded ${res.length} tasks`); // メッセージを更新
      })
      .catch(console.error);
  }, []); // マウント時にのみ実行するため、第2引数は空の配列

  // POST /tasks リクエスト
  const handlePostTask = () => {
    const postData = {
      name: "misawa",
      description: "test",
      status: "未着手",
      user_id: 1,
    };

    createTask(postData)
      .then((data) => console.log("Response:", data))
      .catch((error) => console.error("Error:", error));
  };

  return (
    <div className="App">
      <header className="App-header">
        <p>{message || "Loading..."}</p>
        <button onClick={handlePostTask}>Send POST Request</button> {/* POSTリクエストを送信するボタン */}
        {/* タスク一覧を表示 */}
        <ul>
          {tasks.map((task) => (
            <li key={task.id}>
              {task.name} - {task.description} - {task.status}
            </li>
          ))}
        </ul>
      </header>
    </div>
  );

}

export default App;
