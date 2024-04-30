import React from "react";
import Box from '@mui/material/Box'; 
import TaskHeader from "./TaskHeader";
import TaskContainer from "./TaskContainer";

const TaskList = () => {
  return (
    <Box 
      backgroundColor='rgba(150,150,150,0.1)' 
      height='100%'
      display="flex"
      flexDirection="column"
    >
      <TaskHeader />
      <TaskContainer />
    </Box>
  );
};

export default TaskList;