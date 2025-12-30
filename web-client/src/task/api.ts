import type { Task, TaskCreatePayload, TaskUpdatePayload } from "./define.ts";

const endpoint = "/api/v1/tasks";
const headers: HeadersInit = { "Content-Type": "application/json" };

export async function listTasks(): Promise<Task[]> {
  const res = await fetch(endpoint);
  const json = await res.json();
  return json.data;
}

export async function registerTask(payload: TaskCreatePayload): Promise<Task> {
  const res = await fetch(endpoint, {
    method: "POST",
    headers,
    body: JSON.stringify(payload),
  });
  const json = await res.json();
  return json.data;
}

export async function updateTask(payload: TaskUpdatePayload): Promise<Task> {
  const res = await fetch(endpoint, {
    method: "PATCH",
    headers,
    body: JSON.stringify(payload),
  });
  const json = await res.json();
  return json.data;
}

export async function deleteTask(id: Task["id"]) {
  const res = await fetch(endpoint, {
    method: "DELETE",
    headers,
    body: JSON.stringify({ id }),
  });
  return res.status === 200;
}
