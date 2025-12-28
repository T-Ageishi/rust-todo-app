// await registerTask();

await listTask();

/**
 * GET /api/v1/tasks
 */
async function listTask() {
    const res = await fetch("http://127.0.0.1:8080/api/v1/tasks", {
        method: "GET",
    });

    const json = await res.json();
    console.log(json);
}

/**
 * POST /api/v1/tasks
 */
async function registerTask() {
    const res = await fetch("http://127.0.0.1:8080/api/v1/tasks", {
        method: "POST",
        headers: {
            "Content-Type": "application/json",
        },
        body: JSON.stringify({
            title: "Sample Task",
            description: "This is a test task",
            status: 1,
        }),
    });

    const json = await res.json();
    console.log(res.status, json);
}
