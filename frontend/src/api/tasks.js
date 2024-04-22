export const fetchTasks = () => {
  return fetch("/api/tasks")
    .then((res) => res.json())
    .catch(console.error);
};

export const createTask = (postData) => {
  return fetch("/api/tasks", {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify(postData),
  })
    .then((res) => res.json())
    .catch((error) => console.error("Error:", error));
};
