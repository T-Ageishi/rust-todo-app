
/**
 * PATCH /api/v1/tasks
 */
export async function updateTask({id, title, description, status}) {
    const res = await fetch("http://127.0.0.1:8080/api/v1/tasks", {
        method: "PATCH",
        headers: {
            "Content-Type": "application/json",
        },
        body: JSON.stringify((() => {
            const ret = {id};
            if (title !== "") {
                ret.title = title;
            }
            if (description !== "") {
                ret.description = description;
            }
            if (1 <= status && status <= 3) {
                ret.status = status;
            }
            return ret;
        })()),
    });

    const json = await res.json();
    return json.data;
}

