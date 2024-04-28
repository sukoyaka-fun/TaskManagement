import React, { useEffect, useState } from "react";
import { fetchTasks, createTask, fetchUsers } from "./api";
import  MainPageContainer  from "./components/MainPageContainer.js";

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
    <MainPageContainer />
  );

}

export default App;
