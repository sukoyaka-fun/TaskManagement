import React from "react";
import Box from '@mui/material/Box'; 

const TaskContainer = () => {
  return (
    <Box backgroundColor='rgba(100,100,100,0.1)' height='100%'>
      <div>TaskContainer Component</div>
      <div>This component contains as many task components as there are records in the tasks table.</div>
      <div>Task1</div>
      <div>Task2</div>
      <div>Task3</div>
    </Box>
  );
};

export default TaskContainer;