const API_URL = "http://127.0.0.1:3000";

const promptInput = document.getElementById("prompt");
const charCount = document.getElementById("char-count");
const generateButton = document.getElementById("generate-btn");
const appStatus = document.getElementById("app-status");
const imagePlaceholder = document.getElementById("image-placeholder");
const resultContent = document.getElementById("result-content");
const resultMessage = document.getElementById("result-message");
const requirementsList = document.getElementById("requirements-list");

promptInput.addEventListener("input", () => {
    charCount.textContent = `${promptInput.value.length} / 2000`;
});

async function checkBackend() {
    try {
        const response = await fetch(`${API_URL}/health`);

        if (!response.ok) {
            throw new Error("Backend returned an error.");
        }

        appStatus.textContent = "Backend connected. Image generation provider is not connected yet.";
    } catch {
        appStatus.textContent = "Backend offline. Start the Rust server with cargo run.";
    }
}

generateButton.addEventListener("click", async () => {
    const prompt = promptInput.value.trim();

    if (!prompt) {
        appStatus.textContent = "Please describe the image you want first.";
        promptInput.focus();
        return;
    }

    generateButton.disabled = true;
    generateButton.textContent = "Processing…";
    appStatus.textContent = "Sending your requirements to the Rust backend…";

    try {
        const response = await fetch(`${API_URL}/api/generate`, {
            method: "POST",
            headers: {
                "Content-Type": "application/json"
            },
            body: JSON.stringify({ prompt })
        });

        const data = await response.json();

        if (!response.ok) {
            throw new Error(data.error || data || "The request failed.");
        }

        imagePlaceholder.hidden = true;
        resultContent.hidden = false;
        resultMessage.textContent = data.note || data.message;

        requirementsList.replaceChildren();

        for (const item of data.requirements || []) {
            const li = document.createElement("li");
            li.textContent = item.requirement;

            const status = document.createElement("span");
            status.className = "requirement-status";
            status.textContent = `Status: ${item.status}`;

            li.appendChild(status);
            requirementsList.appendChild(li);
        }

        appStatus.textContent =
            "Request successful. This is a backend prototype, not a generated or verified image.";
    } catch (error) {
        appStatus.textContent =
            `Could not process the request: ${error.message}. Check that the backend is running.`;
    } finally {
        generateButton.disabled = false;
        generateButton.textContent = "Prepare image →";
    }
});

checkBackend();