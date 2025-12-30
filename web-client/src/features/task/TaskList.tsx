import { type FormEvent, useEffect, useRef, useState } from "react";
import {
  type Task,
  TASK_STATUS,
  type TaskCreatePayload,
  type TaskEditorProps,
  type TaskEditorSubmitSource,
  type TaskStatus,
  taskStatusMap,
  type TaskUpdatePayload,
} from "./define.ts";
import { deleteTask, listTasks, registerTask, updateTask } from "./api.ts";

export default function TaskList() {
  const [tasks, setTasks] = useState<Task[]>([]);
  const { open, close, TaskEditor } = useTaskEditor();

  useEffect(() => {
    (async () => {
      setTasks(await listTasks());
    })();
  }, []);

  const handleRegisterTask = async (payload: TaskCreatePayload) => {
    const task = await registerTask(payload);
    setTasks((prev) => [task, ...prev]);
  };

  const handleUpdateTask = async (payload: TaskUpdatePayload) => {
    const task = await updateTask(payload);
    setTasks((prev) => prev.map((t) => (t.id === task.id ? task : t)));
  };

  const handleDeleteTask = async (id: Task["id"]) => {
    const result = await deleteTask(id);
    if (result) {
      setTasks((prev) => prev.filter((t) => t.id !== id));
    }
  };

  const handleEdit = async (source: TaskEditorSubmitSource) => {
    if (source.mode === "register") {
      await handleRegisterTask(source.payload);
    } else {
      await handleUpdateTask(source.payload);
    }
    close();
  };

  return (
    <>
      <div className="tasks-actions">
        <button onClick={() => open({ type: "register" })} className="tasks-actions__register">
          <span className="tasks-actions__register-icon">+</span>
          Register
        </button>
      </div>

      <table className="tasks">
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
                  <button
                    onClick={() => handleDeleteTask(id)}
                    className={"tasks__body-cell-delete"}
                  >
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
  const [dialogOpen, setDialogOpen] = useState(false);

  const dialogRef = useRef<HTMLDialogElement>(null);
  const modeRef = useRef<"register" | "update">("register");
  const initialValueRef = useRef<Task>({
    id: "",
    title: "",
    description: "",
    status: TASK_STATUS.TODO,
  });

  function open(source: { type: "register" } | ({ type: "update" } & Task)) {
    if (source.type === "register") {
      modeRef.current = "register";
      initialValueRef.current = {
        id: "",
        title: "",
        description: "",
        status: TASK_STATUS.TODO,
      };
    } else {
      modeRef.current = "update";
      initialValueRef.current = source;
    }

    setDialogOpen(true);
    dialogRef.current?.showModal();
  }

  function close() {
    setDialogOpen(false);
    dialogRef.current?.close();
  }

  function TaskEditor({ onEdit }: TaskEditorProps) {
    const [title, setTitle] = useState("");
    const [description, setDescription] = useState("");
    const [status, setStatus] = useState<TaskStatus>(TASK_STATUS.TODO);

    useEffect(() => {
      if (!dialogOpen) return;

      setTitle(initialValueRef.current.title);
      setDescription(initialValueRef.current.description);
      setStatus(initialValueRef.current.status);
    }, [dialogOpen]);

    const handleSubmit = async (e: FormEvent<HTMLFormElement>) => {
      e.preventDefault();

      if (modeRef.current === "register") {
        await onEdit({
          mode: "register",
          payload: { title, description, status },
        });
      } else {
        await onEdit({
          mode: "update",
          payload: {
            id: initialValueRef.current.id,
            title,
            description,
            status,
          },
        });
      }
    };

    return (
      <dialog ref={dialogRef} className={"task-dialog"}>
        <form onSubmit={handleSubmit} className={"task-dialog__form"}>
          <h3 className={"task-dialog__title"}>
            {modeRef.current === "register" ? "Register Task" : "Update Task"}
          </h3>

          <div className={"task-dialog__field"}>
            <label className={"task-dialog__label"}>
              Title
              <input
                value={title}
                onChange={(e) => setTitle(e.target.value)}
                className={"task-dialog__input"}
              />
            </label>
          </div>
          <div className={"task-dialog__field"}>
            <label className={"task-dialog__label"}>
              Description
              <textarea
                value={description}
                onChange={(e) => setDescription(e.target.value)}
                className={"task-dialog__textarea"}
              />
            </label>
          </div>
          <div className={"task-dialog__field"}>
            <label className={"task-dialog__label"}>
              Status
              <select
                value={status}
                onChange={(e) => setStatus(Number(e.target.value) as TaskStatus)}
                className={"task-dialog__select"}
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
            <button type="button" onClick={close}>
              Cancel
            </button>
            <button type="submit" className={"task-dialog__save"}>
              {modeRef.current === "register" ? "Register" : "Update"}
            </button>
          </menu>
        </form>
      </dialog>
    );
  }

  return { open, close, TaskEditor };
}
