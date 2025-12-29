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
  onUpdate: (source: {
    id: Task["id"];
    title: Task["title"];
    description: Task["description"];
    status: Task["status"];
  }) => Promise<void>;
  onDelete: (id: Task["id"]) => Promise<void>;
};

type TaskEditorSubmitSource =
  | {
      mode: "register";
      title: Task["title"];
      description: Task["description"];
      status: Task["status"];
    }
  | {
      mode: "update";
      id: Task["id"];
      title: Task["title"];
      description: Task["description"];
      status: Task["status"];
    };

type TaskEditorProps = {
  onEdit: (source: TaskEditorSubmitSource) => Promise<void>;
};

const taskStatusMap = {
  [TASK_STATUS.TODO]: "ToDo",
  [TASK_STATUS.DOING]: "Doing",
  [TASK_STATUS.DONE]: "Done",
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
    const task = await registerTask(source);
    setTasks([task, ...tasks]);
  };

  const handleUpdateTask = async (source: {
    id: Task["id"];
    title: Task["title"];
    description: Task["description"];
    status: Task["status"];
  }) => {
    const task = await updateTask(source);
    setTasks(() =>
      tasks.reduce<Task[]>((acc, t) => {
        if (t.id === task.id) {
          acc.push(task);
        } else {
          acc.push(t);
        }
        return acc;
      }, []),
    );
  };

  const handleDeleteTask = async (id: Task["id"]) => {
    const result = await deleteTask(id);
    if (result) {
      setTasks(() => tasks.filter((t) => t.id !== id));
    }
  };

  return (
    <main>
      <Tasks
        tasks={tasks}
        onRegister={handleRegisterTask}
        onUpdate={handleUpdateTask}
        onDelete={handleDeleteTask}
      />
    </main>
  );
}

function Tasks({ tasks, onRegister, onUpdate, onDelete }: TaskProps) {
  const { open, close, TaskEditor } = useTaskEditor();

  const handleEdit = async (source: TaskEditorSubmitSource) => {
    if (source.mode === "register") {
      await onRegister({
        title: source.title,
        description: source.description,
        status: source.status,
      });
    } else if (source.mode === "update") {
      await onUpdate({
        id: source.id,
        title: source.title,
        description: source.description,
        status: source.status,
      });
    }

    close();
  };

  return (
    <>
      <div className={"tasks-actions"}>
        <button onClick={() => open({ type: "register" })} className={"tasks-actions__register"}>
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
          {tasks.map(({ id, title, description, status }) => (
            <tr key={id}>
              <td className={"tasks__cell tasks__body-cell"}>{id}</td>
              <td className={"tasks__cell tasks__body-cell"}>{title}</td>
              <td className={"tasks__cell tasks__body-cell"}>{description}</td>
              <td className={"tasks__cell tasks__body-cell-status"}>{taskStatusMap[status]}</td>
              <td className={"tasks__cell"}>
                <div className={"tasks__body-cell-actions"}>
                  <button
                    onClick={() => open({ type: "update", id, title, description, status })}
                    className={"tasks__body-cell-edit"}
                  >
                    Edit
                  </button>
                  <button onClick={() => onDelete(id)} className={"tasks__body-cell-delete"}>
                    Delete
                  </button>
                </div>
              </td>
            </tr>
          ))}
        </tbody>
      </table>
      <TaskEditor onEdit={handleEdit} />
    </>
  );
}

function useTaskEditor() {
  const dialogRef = useRef<HTMLDialogElement>(null);
  const modeRef = useRef<"register" | "update">("register");
  const initialValueRef = useRef<{
    id: Task["id"];
    title: Task["title"];
    description: Task["description"];
    status: Task["status"];
  }>({ id: "", title: "", description: "", status: TASK_STATUS.TODO });

  const [dialogOpen, setDialogOpen] = useState(false);

  function open(
    source:
      | { type: "register" }
      | {
          type: "update";
          id: Task["id"];
          title: Task["title"];
          description: Task["description"];
          status: Task["status"];
        },
  ) {
    if (source.type === "register") {
      modeRef.current = "register";
      initialValueRef.current = {
        id: "",
        title: "",
        description: "",
        status: TASK_STATUS.TODO,
      };
    } else if (source.type === "update") {
      modeRef.current = "update";
      initialValueRef.current = {
        id: source.id,
        title: source.title,
        description: source.description,
        status: source.status,
      };
    }

    setDialogOpen(true);
    dialogRef.current?.showModal();
  }

  function close() {
    setDialogOpen(false);
    dialogRef.current?.close();
  }

  function TaskEditor({ onEdit }: TaskEditorProps) {
    const [title, setTitle] = useState<Task["title"]>(initialValueRef.current.title);
    const [description, setDescription] = useState<Task["description"]>("");
    const [status, setStatus] = useState<Task["status"]>(TASK_STATUS.TODO);

    useEffect(() => {
      if (!dialogOpen) return;

      setTitle(initialValueRef.current.title);
      setDescription(initialValueRef.current.description);
      setStatus(initialValueRef.current.status);
    }, [
      dialogOpen,
      initialValueRef.current.title,
      initialValueRef.current.description,
      initialValueRef.current.status,
    ]);

    const handleSubmit = async (e: FormEvent<HTMLFormElement>) => {
      e.preventDefault();

      if (modeRef.current === "register") {
        await onEdit({ mode: modeRef.current, title, description, status });
      } else if (modeRef.current === "update") {
        await onEdit({
          mode: modeRef.current,
          id: initialValueRef.current.id,
          title,
          description,
          status,
        });
      }

      setTitle("");
      setDescription("");
      setStatus(TASK_STATUS.TODO);
    };

    return (
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
                {Object.entries(taskStatusMap).map(([value, label]) => (
                  <option key={value} value={value}>
                    {label}
                  </option>
                ))}
              </select>
            </label>
          </div>

          <menu className={"task-dialog__actions"}>
            <button
              className={"task-dialog__button task-dialog__button--secondary"}
              type="button"
              onClick={() => close()}
            >
              Cancel
            </button>
            <button className={"task-dialog__button task-dialog__button--primary"} type="submit">
              Register
            </button>
          </menu>
        </form>
      </dialog>
    );
  }

  return { open, close, TaskEditor };
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

async function updateTask({
  id,
  title,
  description,
  status,
}: {
  id: Task["id"];
  title: Task["title"];
  description: Task["description"];
  status: Task["status"];
}): Promise<Task> {
  const payload: {
    id: Task["id"];
    title?: Task["title"];
    description?: Task["description"];
    status: Task["status"];
  } = { id, status };
  if (title !== "") {
    payload.title = title;
  }
  if (description !== "") {
    payload.description = description;
  }
  const res = await fetch("/api/v1/tasks", {
    method: "PATCH",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify(payload),
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
