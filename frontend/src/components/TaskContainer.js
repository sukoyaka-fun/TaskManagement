import React, {useEffect, useState } from "react";
import Box from '@mui/material/Box'; 
import { fetchTasks } from "../api";
import Task from "./Task";

const TaskContainer = () => {
  const [tasks, setTasks] = useState([]);

  useEffect(() => {
    // タスクデータを取得してstateにセットする
    fetchTasks()
      .then((res) => {
        console.log("debug:");
        console.log(res);
        setTasks(res);
      })
      .catch(console.error);
  }, []); // マウント時にのみ実行するため、第2引数は空の配列

  return (
    <Box backgroundColor='rgba(100,100,100,0.1)' height='100%' display="flex" flexDirection="column">
      {/* タスクデータをマップしてタスクコンポーネントを生成 */}
      {tasks.map((task) => (
        <Task key={task.id} task={task} />
      ))}
    </Box>
  );
};

// const TaskContainer = () => {
//   return (
//     <Box backgroundColor='rgba(100,100,100,0.1)' height='100%'>
//       <div>TaskContainer Component</div>
//       <div>This component contains as many task components as there are records in the tasks table.</div>
//       <div>Task1</div>
//       <div>Task2</div>
//       <div>Task3</div>
//     </Box>
//   );
// };

export default TaskContainer;