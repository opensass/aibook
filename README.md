<div align="center">

# 📖 AIBook 🤖

[![made-with-rust](https://img.shields.io/badge/Made%20with-Rust-1f425f.svg?logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![Rust](https://img.shields.io/badge/Rust-1.75%2B-blue.svg)](https://www.rust-lang.org)
[![Maintenance](https://img.shields.io/badge/Maintained%3F-yes-green.svg)](https://github.com/wiseaidev)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

[![GigaDAO Discord](https://dcbadge.limes.pink/api/server/dGCPR6bq)](https://discord.gg/dGCPR6bq)

| 🐧 Linux `(Recommended)` |
| :------: |
| [ ![Linux Banner](https://github.com/user-attachments/assets/9b895bcf-43f8-4839-842b-4ad51c8c7777)](https://github.com/opensass/aibook/releases/download/v0.0.1/dist.zip) |
| `./dist/aibook` |

</div>

## 🛠️ Pre-requisites:

1. Install [`rustup`](https://www.rust-lang.org/tools/install):

    ```bash
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    ```

1. Install [`Dioxus CLI`](https://dioxuslabs.com/learn/0.5/getting_started):

    ```bash
    cargo install dioxus-cli
    ```

## 🚀 Building and Running

1. Fork/Clone the GitHub repository.

	```bash
	git clone https://github.com/opensass/aibook
	```

1. Run the client:

	```sh
	dx serve --port 3000
	```

Navigate to http://localhost:3000 to explore the landing page.

## ✅ Supported Features

- Support for all Gemini models (e.g. Gemini Pro 1.5, Flash 1.5).

![Gemini Models](https://github.com/user-attachments/assets/58f531d0-c352-40eb-8bb2-aed7359fccbc)

- Built-in Dark and Light themes.

![Light Dark Themes](https://github.com/user-attachments/assets/71820497-efcc-4227-a906-e97cdf9aa45b)

- JWT authentication.

- Forms validations.

![Email validation.](https://github.com/user-attachments/assets/7b86a5b5-e5a1-44af-8da1-b442d9869afc)

- Instant toast notifications when submitting a form.

![Toast notification.](https://github.com/user-attachments/assets/6c5149c9-bb5d-4786-a51b-38c36b4ade0c)

- Sending and receiving text messages in real time.

![Sending and receiving text messages.](https://github.com/user-attachments/assets/d3ca3f38-41dc-4815-b7eb-35f8b5d10e36)

## 👨‍💻 Data Models

![MongDB Models](https://github.com/user-attachments/assets/a2f430c3-3d5a-491d-9fc9-b833a555cbc1)

AIBook is powered by a MongoDB storage. The following a closer look at the data models and how they interconnect:

- **User**: Manages user information, credentials, and roles, allowing for secure, authenticated access.
- **Book**: Stores book-specific details, including title, type, and main topics, as well as creation and update timestamps.
- **Chapter**: Holds chapter content in both markdown and HTML formats.
- **Conversation**: Logs conversations between users and the Gemini AI.
- **Message**: Captures individual messages within each conversation.
- **Subscription**: Manages user subscription plans, payment methods, and active status.

## 🤖 Gemini AI Integration

The Gemini AI integration is a core part of AIBook’s functionality. When users generate a chapter, the `generate_chapter_content` function utilizes the "gems" crate to send a structured prompt to the Gemini AI, requesting content in markdown format for editor use and HTML format for publishing. The AI’s responses are saved in MongoDB, allowing users to retrieve, update, or regenerate content as needed.

