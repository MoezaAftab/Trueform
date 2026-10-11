const promptInput = document.getElementById("prompt");
const generateButton = document.getElementById("generate-btn");
const appStatus = document.getElementById("app-status");

const imagePlaceholder = document.getElementById("image-placeholder");
const generatedImage = document.getElementById("generated-image");
const resultContent = document.getElementById("result-content");
const resultMessage = document.getElementById("result-message");
const requirementsList = document.getElementById("requirements-list");

console.log("TRUEFORM NEW SCRIPT LOADED");


generateButton.addEventListener("click", async () => {

    const prompt = promptInput.value.trim();

    if (!prompt) {
        appStatus.textContent = "Please enter a prompt.";
        return;
    }

    generateButton.disabled = true;
    generateButton.textContent = "Generating...";
    appStatus.textContent = "Generating your image...";

    try {

        const response = await fetch("/api/generate", {
            method: "POST",
            headers: {
                "Content-Type": "application/json"
            },
            body: JSON.stringify({
                prompt: prompt
            })
        });

        const data = await response.json();

        console.log("API DATA:", data);
        console.log("IMAGE URL:", data.image_url);

        if (!response.ok) {
            throw new Error(data.message || "Generation failed.");
        }

        // THIS IS THE IMPORTANT LINE
        generatedImage.setAttribute("src", data.image_url);

        generatedImage.hidden = false;
        imagePlaceholder.hidden = true;

        resultContent.hidden = false;

        resultMessage.textContent =
            data.message || "Image generated successfully.";

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

        appStatus.textContent = "Image generated successfully.";

    } catch (error) {

        console.error("TRUEFORM ERROR:", error);

        appStatus.textContent =
            `Error: ${error.message}`;

    } finally {

        generateButton.disabled = false;
        generateButton.textContent = "Generate image →";
    }
});