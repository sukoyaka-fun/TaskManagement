import React from "react";
import Box from '@mui/material/Box';
import TaskCheckBoxContainer from "./TaskCheckBoxContainer";
import TaskIndexContainer from "./TaskIndexContainer";

const TaskHeader = () => {
  return (
    <Box 
      backgroundColor='rgba(255,150,50,0.3)' 
      display="flex"
      flexDirection="row"
      justifyContent="space-between"
    >
      <div>TaskHeader Component.</div>
      <div>TaskIndex component.</div>
    </Box>
  );
};

export default TaskHeader;