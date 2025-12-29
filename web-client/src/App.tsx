import "./App.css";
import { type FormEvent, useEffect, useRef, useState } from "react";

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
  onRegister: (source: {
    title: Task["title"];
    description: Task["description"];
    status: Task["status"];
  }) => Promise<void>;
  onDelete: (id: Task["id"]) => Promise<void>;
};

function App() {
  const [tasks, setTasks] = useState<Task[]>([]);

  useEffect(() => {
    (async () => {
      setTasks(await listTasks());
    })();
  }, []);

  const handleRegisterTask = async (source: {
    title: Task["title"];
    description: Task["description"];
    status: Task["status"];
  }) => {
    console.log(source);
    const task = await registerTask(source);
    setTasks([task, ...tasks]);
  };

  const handleDeleteTask = async (id: Task["id"]) => {
    const result = await deleteTask(id);
    if (result) {
      setTasks(() => tasks.filter((t) => t.id !== id));
    }
  };

  return (
    <main>
      <Tasks tasks={tasks} onRegister={handleRegisterTask} onDelete={handleDeleteTask} />
    </main>
  );
}

function Tasks({ tasks, onRegister, onDelete }: TaskProps) {
  const [title, setTitle] = useState("");
  const [description, setDescription] = useState("");
  const [status, setStatus] = useState<Task["status"]>(TASK_STATUS.TODO);

  const dialogRef = useRef<HTMLDialogElement>(null);
  const openDialog = () => {
    dialogRef.current?.showModal();
  };
  const closeDialog = () => {
    dialogRef.current?.close();
  };

  const taskStatusMap = {
    [TASK_STATUS.TODO]: "ToDo",
    [TASK_STATUS.DOING]: "Doing",
    [TASK_STATUS.DONE]: "Done",
  };

  const handleSubmit = async (e: FormEvent<HTMLFormElement>) => {
    e.preventDefault();

    await onRegister({
      title,
      description,
      status,
    });

    setTitle("");
    setDescription("");
    setStatus(TASK_STATUS.TODO);

    closeDialog();
  };

  return (
    <>
      <div className={"tasks-actions"}>
        <button onClick={() => openDialog()} className={"tasks-actions__register"}>
          <span className={"tasks-actions__register-icon"}>+</span>Register
        </button>
      </div>
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
              <td className={"tasks__cell tasks__body-cell-status"}>
                {taskStatusMap[task.status]}
              </td>
              <td className={"tasks__cell"}>
                <button onClick={() => onDelete(task.id)} className={"tasks__body-cell-delete"}>
                  Delete
                </button>
              </td>
            </tr>
          ))}
        </tbody>
      </table>
      <dialog ref={dialogRef} className="task-dialog">
        <form className={"task-dialog__form"} method="dialog" onSubmit={handleSubmit}>
          <h3 className={"task-dialog__title"}>Register Task</h3>

          <div className={"task-dialog__field"}>
            <label className={"task-dialog__label"}>
              Title
              <input
                className={"task-dialog__input"}
                value={title}
                onChange={(e) => setTitle(e.target.value)}
                required
              />
            </label>
          </div>

          <div className={"task-dialog__field"}>
            <label className={"task-dialog__label"}>
              Description
              <textarea
                className={"task-dialog__textarea"}
                value={description}
                onChange={(e) => setDescription(e.target.value)}
              />
            </label>
          </div>

          <div className={"task-dialog__field"}>
            <label className={"task-dialog__label"}>
              Status
              <select
                className={"task-dialog__select"}
                value={status}
                onChange={(e) => setStatus(Number(e.target.value) as Task["status"])}
              >
                <option value={TASK_STATUS.TODO}>ToDo</option>
                <option value={TASK_STATUS.DOING}>Doing</option>
                <option value={TASK_STATUS.DONE}>Done</option>
              </select>
            </label>
          </div>

          <menu className={"task-dialog__actions"}>
            <button className={"task-dialog__button task-dialog__button--primary"} type="submit">
              Register
            </button>
            <button
              className={"task-dialog__button task-dialog__button--secondary"}
              type="button"
              onClick={() => closeDialog()}
            >
              Cancel
            </button>
          </menu>
        </form>
      </dialog>
    </>
  );
}

async function listTasks(): Promise<Task[]> {
  const res = await fetch("/api/v1/tasks", {
    method: "GET",
  });
  const json = await res.json();
  return json.data;
}

async function registerTask({
  title,
  description,
  status,
}: {
  title: Task["title"];
  description: Task["description"];
  status: Task["status"];
}): Promise<Task> {
  const res = await fetch("/api/v1/tasks", {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify({
      title,
      description,
      status,
    }),
  });

  const json = await res.json();
  return json.data as Task;
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
