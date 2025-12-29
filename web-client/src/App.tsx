import "./App.css";
import { useEffect, useState } from "react";

const TASK_STATUS = Object.freeze({
  TODO: 1,
  DOING: 2,
  DONE: 3,
});

type Task = {
  id: string;
  title: string;
  description: string;
  status: (typeof TASK_STATUS)[keyof typeof TASK_STATUS];
};
type TaskProps = {
  tasks: Task[];
  onDelete: (id: Task["id"]) => Promise<void>;
};

function App() {
  const [tasks, setTasks] = useState<Task[]>([]);

  useEffect(() => {
    (async () => {
      setTasks(await listTasks());
    })();
  }, []);

  const handleDeleteTask = async (id: Task["id"]) => {
    const result = await deleteTask(id);
    if (result) {
      setTasks(() => tasks.filter((t) => t.id !== id));
    }
  };

  return (
    <main>
      <Tasks tasks={tasks} onDelete={handleDeleteTask} />
    </main>
  );
}

function Tasks({ tasks, onDelete }: TaskProps) {
  const taskStatusMap = {
    [TASK_STATUS.TODO]: "ToDo",
    [TASK_STATUS.DOING]: "Doing",
    [TASK_STATUS.DONE]: "Done",
  };

  return (
    <table className={"tasks"}>
      <thead>
        <tr>
          <td className={"tasks__cell tasks__header-cell"}>ID</td>
          <td className={"tasks__cell tasks__header-cell"}>Title</td>
          <td className={"tasks__cell tasks__header-cell"}>Description</td>
          <td className={"tasks__cell tasks__header-cell"}>Status</td>
          <td className={"tasks__cell tasks__header-cell"}></td>
        </tr>
      </thead>
      <tbody>
        {tasks.map((task) => (
          <tr key={task.id}>
            <td className={"tasks__cell tasks__body-cell"}>{task.id}</td>
            <td className={"tasks__cell tasks__body-cell"}>{task.title}</td>
            <td className={"tasks__cell tasks__body-cell"}>{task.description}</td>
            <td className={"tasks__cell tasks__body-cell-status"}>{taskStatusMap[task.status]}</td>
            <td className={"tasks__cell"}>
              <button onClick={() => onDelete(task.id)} className={"tasks__body-cell-delete"}>
                Delete
              </button>
            </td>
          </tr>
        ))}
      </tbody>
    </table>
  );
}

async function listTasks(): Promise<Task[]> {
  const res = await fetch("/api/v1/tasks", {
    method: "GET",
  });
  const json = await res.json();
  return json.data;
}

async function deleteTask(id: Task["id"]) {
  const res = await fetch("/api/v1/tasks", {
    method: "DELETE",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify({
      id,
    }),
  });

  return res.status === 200;
}

export default App;
