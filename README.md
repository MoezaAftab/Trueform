# Trueform

### Your vision, faithfully rendered.

Trueform is a constraint-aware AI image generation platform designed to help users create images that better follow specific visual requirements.

Instead of treating image generation as simply:

> Prompt → Image

Trueform explores a more reliable workflow:

> Requirements → Generate → Evaluate → Refine → Re-evaluate

The goal is to reduce the gap between what a user describes and what an image-generation model actually produces.

---

##  What is Trueform?

AI image generators can create visually impressive images, but they may ignore important details in a prompt.

For example:

> "A black perfume bottle, exactly three white flowers, a gold ribbon, and empty space in the upper-right corner."

A conventional image generator may produce a beautiful image but:

- generate four flowers instead of three
- change the ribbon color
- place objects incorrectly
- ignore spatial requirements
- omit an important object

Trueform is designed around these **visual constraints**.

The system breaks a user's description into individual requirements and provides a workflow for generating, checking, and refining the result.

---

##  Core Idea

Trueform focuses on **constraint-aware image generation**.

### Traditional workflow

```text
User Prompt
     ↓
Requirement Extraction
     ↓
Image Generation
     ↓
Requirement Evaluation
     ↓
Identify Failed Requirements
     ↓
Targeted Refinement
     ↓
Re-evaluation