import { useCallback, useEffect, useState } from "react";
import {
  type Task,
  type TaskCreatePayload,
  type TaskEditorSubmitSource,
  taskStatusMap,
  type TaskUpdatePayload,
} from "./define.ts";
import { deleteTask, listTasks, registerTask, updateTask } from "./api.ts";
import { useTaskEditor } from "./TaskEditor.tsx";

export default function TaskList() {
  const [tasks, setTasks] = useState<Task[]>([]);
  const { open, close, TaskEditor } = useTaskEditor();

  const handleRegisterTask = useCallback(async (payload: TaskCreatePayload) => {
    const task = await registerTask(payload);
    setTasks((prev) => [task, ...prev]);
  }, []);
  const handleUpdateTask = useCallback(async (payload: TaskUpdatePayload) => {
    const task = await updateTask(payload);
    setTasks((prev) => prev.map((t) => (t.id === task.id ? task : t)));
  }, []);
  const handleDeleteTask = useCallback(async (id: Task["id"]) => {
    const result = await deleteTask(id);
    if (result) {
      setTasks((prev) => prev.filter((t) => t.id !== id));
    }
  }, []);
  const handleEdit = useCallback(
    async (source: TaskEditorSubmitSource) => {
      if (source.mode === "register") {
        await handleRegisterTask(source.payload);
      } else {
        await handleUpdateTask(source.payload);
      }
      close();
    },
    [close],
  );

  useEffect(() => {
    (async () => {
      setTasks(await listTasks());
    })();
  }, []);

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
