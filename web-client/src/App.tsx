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
};

function App() {
  const [tasks, setTasks] = useState<Task[]>([]);

  useEffect(() => {
    (async () => {
      setTasks(await fetchTasks());
    })();
  }, []);

  return (
    <main>
      <Tasks tasks={tasks} />
    </main>
  );
}

function Tasks({ tasks }: TaskProps) {
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
        </tr>
      </thead>
      <tbody>
        {tasks.map((task) => (
          <tr key={task.id}>
            <td className={"tasks__cell tasks__body-cell"}>{task.id}</td>
            <td className={"tasks__cell tasks__body-cell"}>{task.title}</td>
            <td className={"tasks__cell tasks__body-cell"}>{task.description}</td>
            <td className={"tasks__cell tasks__body-cell-status"}>{taskStatusMap[task.status]}</td>
          </tr>
        ))}
      </tbody>
    </table>
  );
}

async function fetchTasks(): Promise<Task[]> {
  const res = await fetch("/api/v1/tasks", {
    method: "GET",
  });
  const json = await res.json();
  return json.data;
}

export default App;
