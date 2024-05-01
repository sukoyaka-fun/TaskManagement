import React from "react";
import Grid from "@mui/material/Grid";
import Header from "./Header";
import MenuBar from "./MenuBar";
import TaskList from "./TaskList";


const MainPageContainer = () => {
  return (
    <Grid container height={'100vh'}>
      <Grid item xs={12} height={'15vh'}>
        <Header />
      </Grid>
      <Grid item xs={2} height={'85vh'}>
        <MenuBar />
      </Grid>
      <Grid item xs={10} height={'85vh'}>
        <TaskList />
      </Grid>
    </Grid>
  );
};

export default MainPageContainer;