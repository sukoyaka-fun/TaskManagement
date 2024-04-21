import React, { useEffect, useState } from "react";
import logo from "./logo.svg";
import "./App.css";

function App() {
  const [message, setMessage] = useState();
  const [tasks, setTasks] = useState([]);

  useEffect(() => {
    // GET /users リクエスト
    fetch("/api/users")
      .then((res) => res.json())
      .then((res) => setMessage(`Hello with ${res.length} users`))
      .catch(console.error);
  }, [setMessage]);

  useEffect(() => {
    // GET /tasks リクエスト
    fetch("/api/tasks")
      .then((res) => res.json())
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

    fetch("/api/tasks", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify(postData),
    })
      .then((res) => res.json())
      .then((data) => console.log("Response:", data))
      .catch((error) => console.error("Error:", error));
  };


  return (
    <div className="App">
      <header className="App-header">
        <img src={logo} className="App-logo" alt="logo" />
        <p>{message || "Loading..."}</p>
        <button onClick={handlePostTask}>Send POST Request</button> {/* POSTリクエストを送信するボタン */}
        {/* タスク一覧を表示 */}
        <u1>
          {tasks.map((task) => (
            <li key={task.id}>
              {task.name} - {task.description} - {task.status}
            </li>
          ))}
        </u1>
        <p>
          Edit <code>src/App.js</code> and save to reload.
        </p>
        <a
          className="App-link"
          href="https://reactjs.org"
          target="_blank"
          rel="noopener noreferrer"
        >
          Learn React
        </a>
      </header>
    </div>
  );

}

export default App;
