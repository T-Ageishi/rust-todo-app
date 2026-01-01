/**
 * register multiple tasks
 * POST /api/v1/tasks
 */
export async function registerTasks(count) {
    for (let i = 0; i < count; i++) {
        const res = await registerTask(newTask(i));
        const json = await res.json();
        console.log(res.status, json);
    }

    function newTask(i) {
        const min = 1;
        const max = 3;
        const status = Math.floor(Math.random() * (max - min + 1)) + min;

        return {
            title: `Sample Task${i === 0 ? "" : `(${i})`}`,
            description: `This is a sample task${i === 0 ? "" : `(${i})`}`,
            status,
        };
    }
}

/**
 * POST /api/v1/tasks
 */
async function registerTask(body) {
    const res = await fetch("http://127.0.0.1:8080/api/v1/tasks", {
        method: "POST",
        headers: {
            "Content-Type": "application/json",
        },
        body: JSON.stringify(body),
    });

    const json = await res.json();
    console.log(res.status, json);
}
