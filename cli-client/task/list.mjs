/**
 * GET /api/v1/tasks
 */
export async function listTasks() {
    const res = await fetch("http://127.0.0.1:8080/api/v1/tasks", {
        method: "GET",
    });

    const json = await res.json();
    console.log(json);
}