export const fetchTasks = () => {
  return fetch("/api/tasks")
    .then((res) => {
      if (!res.ok) {
        throw new Error("Failed to fetch tasks");
      }
      return res.json();
    })
    .catch((error) => {
      console.error("Error fetching tasks:", error);
      throw error; // エラーをリジェクトする
    });
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
