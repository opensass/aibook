<div align="center">

# 📖 AIBook 🤖

[![made-with-rust](https://img.shields.io/badge/Made%20with-Rust-1f425f.svg?logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![Rust](https://img.shields.io/badge/Rust-1.79%2B-blue.svg)](https://www.rust-lang.org)
[![Maintenance](https://img.shields.io/badge/Maintained%3F-yes-green.svg)](https://github.com/wiseaidev)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

[![Open SASS Discord](https://dcbadge.limes.pink/api/server/dGCPR6bq)](https://discord.gg/dGCPR6bq)

| 🐧 Linux `(Recommended)` | 🪟 Windows |
| :------: | :------: |
| <video src="https://github.com/user-attachments/assets/aaa3b858-8f81-4c92-9cdc-c18e6300b48d"></video> | <video src="https://github.com/user-attachments/assets/9b593e2b-6c22-406e-93b7-d620448f82e7"></video> |
| [**Download Executable**](https://github.com/opensass/aibook/releases/download/v0.0.1/dist.zip) | [**Download Executable**](https://github.com/opensass/aibook/releases/download/v0.0.1/dist.rar) |
| [**Set Environment Variables**](https://github.com/opensass/aibook#-setting-up-env-vars) | [**Set Environment Variables**](https://github.com/opensass/aibook#-setting-up-env-vars) |
| unzip files | unzip files |
| execute `./dist/aibook` | execute `.\dist\aibook.exe` |

</div>

## 📐 Architecture

![Arch](https://github.com/user-attachments/assets/b5af3f0b-1855-4510-853a-f4258e81cccd)

## 🖥️ For the `.exe` Enjoyers

So, you're the kinda person who'd rather download an `.exe` than spend 20 minutes watching code compile? No worries; I gotcha! 🎉 Each release comes with pre-compiled binaries. Just download, set env vars, run a command, and boom.

> [!NOTE]
> - 📸 **Unsplash API**: Limited to 50 requests per hour.
> - 💎 **Gemini credits**: Unlimited!
> - 🗄️ **MongoDB Storage**: Capped at around ~512MB.

Now, navigate to the [🔑 Setting Up Env Vars](https://github.com/opensass/aibook#-setting-up-env-vars) section.

## 🤓 For the Hardcore Nerds

Aight, if you're, just like me, one of those brave souls who **wants** to compile everything themself, this section is for you. 🛠️ No shortcuts, just raw code and dedication. Grab your favorite terminal, fire up those dependencies, and let the adventure begin!

### 🛠️ Pre-requisites:

1. Install [`rustup`](https://www.rust-lang.org/tools/install):

    ```bash
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    ```

1. Install [`Dioxus CLI`](https://dioxuslabs.com/learn/0.5/getting_started):

    ```bash
    cargo install dioxus-cli
    ```

1. Fork/Clone the GitHub repository.

	```bash
	git clone https://github.com/opensass/aibook
	```

## 🔑 Setting Up Env Vars

Before you can start running AIBook, you'll need to configure a few environment variables. These are essential for connecting to external services like MongoDB, Unsplash, and the Gemini AI, so let's get you set up! Here's a quick guide:

### Create an `.env` File

Inside the project root, copy and create a file named `.env` from `.env-example`. This file will securely store all your environment variables.

```bash
cp .env.example .env
```

> [!NOTE]
> Replace the following values with your actual credentials.
>
> ```bash
> MONGODB_USR=
> MONGODB_PWD=
> MONGODB_CLSTR=your-cluster.mongodb.net
> MONGODB_DB_NAME=aibooks
> JWT_SECRET=
> GEMINI_API_KEY=
> UNSPLASH_API_KEY=
> ```
>
> If you're missing any of these keys, check the service's developer portal to generate them.

### 🥑 Set Up MongoDB

Follow [our quick guide](./MongoDB.md) to set up your MongoDB database and connect it to your project!

### 🔐 Generate JWT Secret Key

Generate a secret key using OpenSSL and update its env var in the `.env` file.

```bash
openssl rand -hex 128

d8d0b35856c6fa90a8f3f818fa1c71785d63181945077a4c81e28f731de406c94acad5e864fc85604c520cd67e4977a06656eee081d2d0a897415bb42d8dca167662ae53078084ce70feaee104a3428797078c5bb359db277b26182114bb6b6f4e50d34dcce1ab2ed952912f5783ca89138d508f41bc2d56e60ef2480f501819
```

### ✨ Gemini AI API

To obtain your API key, navigate to [Google AI Studio](https://aistudio.google.com/app/apikey) and generate it there. This key allows aibook to communicate with Gemini API.

### 📸 Unsplash API

AIBook uses Unsplash which provides a powerful API to search for and retrieve high-quality images. To communicate with this api you will need a [Secret key](https://unsplash.com/oauth/applications). If you don't already have one, sign up for a free account at Unsplash, create a new app, and copy the Secret key at the bottom of the page after creating the app.

### 🚀 Building and Running

- Run the client:

	```sh
	dx serve --port 3000
	```

Navigate to http://localhost:3000 to explore the landing page.

> [!WARNING]
> This might take a few minutes (yes, seriously). But hey, good things take time, right?

Happy compiling! 😄

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

## 🗂️ Project Structure

This project is packing 81 files! 😅 But don't worry, it's all organized with love, care, and the principles of SoC and DRY in mind (peak engineering, ngl). Each file has a job to do, and it does it well; like little code ninjas in their own modular worlds.

Here's what the structure looks like:

<details>
<summary><code>❯ cd src && tree</code></summary>

```sh
❯ cd src && tree
.
├── ai.rs
├── components
│   ├── common
│   │   ├── header.rs
│   │   ├── logo.rs
│   │   └── server.rs
│   ├── common.rs
│   ├── dashboard
│   │   ├── analytics.rs
│   │   ├── books
│   │   │   ├── create.rs
│   │   │   ├── edit.rs
│   │   │   ├── list.rs
│   │   │   └── read.rs
│   │   ├── books.rs
│   │   ├── chat
│   │   │   ├── panel.rs
│   │   │   └── sidebar.rs
│   │   ├── chat.rs
│   │   ├── fields
│   │   │   ├── input.rs
│   │   │   ├── number.rs
│   │   │   └── select.rs
│   │   ├── fields.rs
│   │   ├── navbar.rs
│   │   ├── profile.rs
│   │   └── sidebar.rs
│   ├── dashboard.rs
│   ├── features
│   │   ├── grid.rs
│   │   └── item.rs
│   ├── features.rs
│   ├── footer
│   │   ├── bottom.rs
│   │   ├── contact.rs
│   │   ├── icon.rs
│   │   ├── links.rs
│   │   ├── logo.rs
│   │   └── support.rs
│   ├── footer.rs
│   ├── hero.rs
│   ├── navbar
│   │   ├── btns.rs
│   │   └── links.rs
│   ├── navbar.rs
│   ├── pricing.rs
│   ├── spinner.rs
│   ├── testimonial
│   │   ├── author.rs
│   │   ├── card.rs
│   │   └── rating.rs
│   ├── testimonial.rs
│   ├── toast
│   │   ├── manager.rs
│   │   └── provider.rs
│   └── toast.rs
├── components.rs
├── db.rs
├── lib.rs
├── main.rs
├── pages
│   ├── book.rs
│   ├── dashboard.rs
│   ├── home.rs
│   ├── login.rs
│   └── signup.rs
├── pages.rs
├── router.rs
├── server
│   ├── auth
│   │   ├── controller.rs
│   │   ├── model.rs
│   │   └── response.rs
│   ├── auth.rs
│   ├── book
│   │   ├── controller.rs
│   │   ├── model.rs
│   │   ├── request.rs
│   │   └── response.rs
│   ├── book.rs
│   ├── common
│   │   ├── request.rs
│   │   └── response.rs
│   ├── common.rs
│   ├── conversation
│   │   ├── controller.rs
│   │   ├── model.rs
│   │   ├── request.rs
│   │   └── response.rs
│   ├── conversation.rs
│   ├── subscription
│   │   ├── controller.rs
│   │   ├── model.rs
│   │   ├── request.rs
│   │   └── response.rs
│   └── subscription.rs
├── server.rs
├── theme.rs
└── unsplash.rs

19 directories, 81 files
```

</details>

### 🛠️ What's Inside?

- **Components**: All modular components live here, following the DRY principle. From `navbar` to `footer`, each feature has its own place, making it easy to find and tweak when needed.
- **Server**: Adheres to the **MVC** pattern, making the backend as clean as a freshly minted Linux distro. You'll find models, controllers, and response handlers for each feature, organized and ready for action.
- **Pages**: Each page of the app (e.g., `dashboard.rs`, `home.rs`) is set up here, so you know exactly where to go to update views.

With this structure, the project stays manageable and maintainable, despite those 81 files. Let's be honest, though: it's probably going to keep growing. 😅

## 👨‍💻 Data Models

![MongDB Models](https://github.com/user-attachments/assets/a2f430c3-3d5a-491d-9fc9-b833a555cbc1)

AIBook is powered by **MongoDB** storage, with each model carefully structured to keep the app humming along smoothly. Here's a closer look at the data models and how they connect:

- **User** 🧑‍💼: Stores user credentials, profiles, and role information. This model ensures each user enjoys secure, authenticated access.
- **Book** 📚: Contains details like title, type, topics, and handy timestamps for creation and updates, essentially, everything about a book except the content itself!
- **Chapter** 📖: Houses the content for each chapter, stored in both markdown and HTML formats for flexibility.
- **Conversation** 💬: Logs chats between users and the Gemini AI, so each interaction has a place in history.
- **Message** 📝: Tracks individual messages within each conversation, capturing the ebb and flow of the AI interaction.
- **Subscription** 💳: Manages subscription plans, payment methods, and active status, essentially the gatekeeper for access levels and perks.

> [!NOTE]
> MongoDB allows us to embed entire documents within another document, bypassing the need for an `ID` relationship (though it does add one more DB call if we want to fetch the data separately). For now, we're not hitting any performance bottlenecks, but this option keeps things flexible as we scale.

Each model is designed to keep data tightly organized, minimize dependencies, and allow for easy scaling. So whether it's a quick query for a single user or a deep dive into chat history, these models keep AIBook streamlined and ready to grow! 🚀

