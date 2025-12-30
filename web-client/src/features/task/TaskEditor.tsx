import { type FormEvent, useEffect, useRef, useState } from "react";
import {
  type Task,
  TASK_STATUS,
  type TaskEditorProps,
  type TaskStatus,
  taskStatusMap,
} from "./define.ts";

export function useTaskEditor() {
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
