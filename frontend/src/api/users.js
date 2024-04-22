export const fetchUsers = () => {
  return fetch("/api/users")
    .then((res) => res.json())
    .catch((error) => console.error("Error fetching users:", error));
};
  