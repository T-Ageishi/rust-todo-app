// await registerTasks(10);
await listTask();

// await updateTask();

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

/**
 * register multiple tasks
 * POST /api/v1/tasks
 */
async function registerTasks(count) {
    for (let i = 0; i < count; i++) {
        const res = await fetch("http://127.0.0.1:8080/api/v1/tasks", {
            method: "POST",
            headers: {
                "Content-Type": "application/json",
            },
            body: JSON.stringify(newTask(i)),
        });

        const json = await res.json();
        console.log(res.status, json);
    }

    function newTask(i) {
        const min = 1;
        const max = 3;
        const status = Math.floor(Math.random() * (max - min + 1)) + min;

        return {
            title: `Sample Task${i === 0 ? "" : `(${i})`}`,
            description: `This is a test task${i === 0 ? "" : `(${i})`}`,
            status,
        };
    }
}

/**
 * PATCH /api/v1/tasks
 */
async function updateTask() {
    const res = await fetch("http://127.0.0.1:8080/api/v1/tasks", {
        method: "PATCH",
        headers: {
            "Content-Type": "application/json",
        },
        body: JSON.stringify({
            id: "78c45347-f332-4269-9115-6a5fa2bb1995",
            title: "Sample Task(9) (modified)",
        }),
    });

    const json = await res.json();
    console.log(res.status, json);
}