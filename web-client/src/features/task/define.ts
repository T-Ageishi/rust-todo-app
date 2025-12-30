export type TaskStatus = (typeof TASK_STATUS)[keyof typeof TASK_STATUS];

export type Task = {
  id: string;
  title: string;
  description: string;
  status: TaskStatus;
};

export type TaskCreatePayload = Omit<Task, "id">;

export type TaskUpdatePayload = {
  id: Task["id"];
  status: TaskStatus;
} & Partial<Omit<Task, "id" | "status">>;

export type TaskEditorSubmitSource =
  | {
      mode: "register";
      payload: TaskCreatePayload;
    }
  | {
      mode: "update";
      payload: TaskUpdatePayload;
    };

export type TaskEditorProps = {
  onEdit: (source: TaskEditorSubmitSource) => Promise<void>;
};

export const TASK_STATUS = {
  TODO: 1,
  DOING: 2,
  DONE: 3,
} as const;

export const taskStatusMap: Record<TaskStatus, string> = {
  [TASK_STATUS.TODO]: "ToDo",
  [TASK_STATUS.DOING]: "Doing",
  [TASK_STATUS.DONE]: "Done",
};
