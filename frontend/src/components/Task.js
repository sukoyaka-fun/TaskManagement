import React from "react";
import Box from '@mui/material/Box'; 
import Grid from '@mui/material/Grid'; 
import Typography from "@mui/material/Typography";

const Task = ({task}) => {
//   console.log(task); // タスクの内容をログに出力
  return (
    <Grid 
      container
      display="flex" 
      flexDirection="row" 
      alignItems="center"
      borderBottom={1}
      borderColor="grey.400" 
      paddingY={1}
    >
      <Grid item xs={1} sm={1}>
        <Typography variant="body1">{task.name}</Typography>
      </Grid>
      <Grid item xs={8} sm={8}>
        <Typography variant="body1">{task.description}</Typography>
      </Grid>
      <Grid item xs={1} sm={1}>
        <Typography variant="body1">{task.status}</Typography>
      </Grid>
      <Grid item xs={2} sm={2}>
        <Typography variant="body1">ユーザーid:{task.user_id}</Typography>
      </Grid>
    </Grid>
  );
};

export default Task;