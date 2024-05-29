<p align="center">
  <img src="https://raw.githubusercontent.com/PKief/vscode-material-icon-theme/ec559a9f6bfd399b82bb44393651661b08aaf7ba/icons/folder-markdown-open.svg" width="100" alt="project-logo">
</p>
<p align="center">
    <h1 align="center">DELTA-NOTES-ENDPOINT</h1>
</p>
<p align="center">
    <em>Empowering APIs with Files, Fast.Connecting Data, Serving Presentations.Transforming Notes into Powerful Endpoints.</em>
</p>
<p align="center">
	<img src="https://img.shields.io/github/license/Zlatovlaska11/Delta-Notes-Endpoint?style=default&logo=opensourceinitiative&logoColor=white&color=0080ff" alt="license">
	<img src="https://img.shields.io/github/last-commit/Zlatovlaska11/Delta-Notes-Endpoint?style=default&logo=git&logoColor=white&color=0080ff" alt="last-commit">
	<img src="https://img.shields.io/github/languages/top/Zlatovlaska11/Delta-Notes-Endpoint?style=default&color=0080ff" alt="repo-top-language">
	<img src="https://img.shields.io/github/languages/count/Zlatovlaska11/Delta-Notes-Endpoint?style=default&color=0080ff" alt="repo-language-count">
<p>
<p align="center">
	<!-- default option, no dependency badges. -->
</p>

<br><!-- TABLE OF CONTENTS -->
<details>
  <summary>Table of Contents</summary><br>

- [ Overview](#-overview)
- [ Features](#-features)
- [ Repository Structure](#-repository-structure)
- [ Modules](#-modules)
- [ Getting Started](#-getting-started)
  - [ Installation](#-installation)
  - [ Usage](#-usage)
  - [ Tests](#-tests)
- [ Project Roadmap](#-project-roadmap)
- [ Contributing](#-contributing)
- [ License](#-license)
- [ Acknowledgments](#-acknowledgments)
</details>
<hr>

##  Overview

The Delta-Notes-Endpoint project is an API server built using Rust and associated technologies such as Tide, Shuttle, and async libraries. Its core functionalities revolve around user authentication, file handling, and database management to serve PowerPoint presentations on demand based on user requests. This application allows users to login, register, and access their courses associated files while ensuring consistent testing for reliable file processing. Database interactions manage user registration and presentation data, enabling seamless communication between endpoints and the file handler component.

---

##  Features

|   |    Feature          | Description                                                                |
|----|----------------------|---------------------------------------------------------------------------|
| ⚙️  | Architecture      | Rust-based server using Shuttle web framework and Tide for routing. Depends on various async libraries and config files (`Cargo.toml`, `Shuttle.toml`).|
| 🔩 | Code Quality       | Adheres to Rust coding standards with proper use of comments and documentation. Has a modular structure with logical organization. |
| 📄 | Documentation      | Basic readme file present with limited information about project and setup instructions. |
| 🔌 | Integrations      | Utilizes Tide and Shuttle for routing, async libraries (Tokio), and database connections. |
| 🧩 | Modularity        | Well-structured with a logical folder hierarchy. Functions are grouped appropriately in `src/` subdirectories.|
| 🧪 | Testing           | Has basic testing functionality, utilizing `get_courses.rs` test utility file. |
| ⚡️  | Performance       | Performance information is not available from given context.              |
| 🛡️ | Security          | Database connections are made with no explicit encryption or secure transmission mentioned. |
| 📦 | Dependencies      | `Delta-Notes-Endpoint` relies on several libraries and dependencies.     |
| 🚀 | Scalability       | There is no evident discussion or design consideration for scaling the project. |

-----------------------------------------------------------------------------------
Dependency Description:
-----------------------
-------------------
[['rs', 'Cargo.toml'], ['rust'], ['json'], ['Cargo.lock'], ['toml'], ['lock']]

- Rust programming language and Cargo package manager configuration files.

---

##  Repository Structure

```sh
└── Delta-Notes-Endpoint/
    ├── Cargo.lock
    ├── Cargo.toml
    ├── Shuttle.toml
    ├── src
    │   ├── auth.rs
    │   ├── database.rs
    │   ├── enpoint.rs
    │   ├── filehalndler
    │   └── main.rs
    └── test.json
```

---

##  Modules

<details closed><summary>.</summary>

| File                                                                                           | Summary                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| ---                                                                                            | ---                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| [Cargo.toml](https://github.com/Zlatovlaska11/Delta-Notes-Endpoint/blob/master/Cargo.toml)     | In this Cargo.toml configuration file, the Delta-Notes-Endpoint project is set up for development with various dependencies. These include async libraries, environment management tools, serialization and database modules, and web frameworks like Tide and Shuttle. This foundation supports building an API endpoint server for Delta-Notes application.                                                                                                                                                                                                                                                                                                                                                                           |
| [Shuttle.toml](https://github.com/Zlatovlaska11/Delta-Notes-Endpoint/blob/master/Shuttle.toml) | Configures and manages Rocket.rs web application for Delta-Notes project, defining application name and other essential settings. Key components include routing, middleware, database connections, and API endpoints.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| [test.json](https://github.com/Zlatovlaska11/Delta-Notes-Endpoint/blob/master/test.json)       | The `test.json` file in the `Delta-Notes-Endpoint` repository serves as configuration data for testing various file handling scenarios in the application. It follows the JSON format and contains an array of test objects, each representing a single test case. The main purpose of this file is to facilitate consistent and automated testing of the file handler module in the context of the REST API (defined by `endpoint.rs`), ensuring reliable processing of user-uploaded files within the applications architecture. By providing test cases with defined file paths and filenames, developers can validate functionality related to handling various types of files, ensuring a stable and feature-rich user experience. |

</details>

<details closed><summary>src</summary>

| File                                                                                             | Summary                                                                                                                                                                                                                                                                         |
| ---                                                                                              | ---                                                                                                                                                                                                                                                                             |
| [auth.rs](https://github.com/Zlatovlaska11/Delta-Notes-Endpoint/blob/master/src/auth.rs)         | Authenticates user credentials in the application. Contains `login` and `register` functions to handle requests for user verification and registration, respectively. Uses database connection from `database.rs` and JSON body parsing from serde library.                     |
| [database.rs](https://github.com/Zlatovlaska11/Delta-Notes-Endpoint/blob/master/src/database.rs) | Manage database connections and interactions in the Delta-Notes-Endpoint repository. This module defines `get_connection` for establishing new connections, `insert_data` for inserting notes into the database, and `select` function for retrieving data based on file names. |
| [enpoint.rs](https://github.com/Zlatovlaska11/Delta-Notes-Endpoint/blob/master/src/enpoint.rs)   | Process requests for login and registration endpoints. Serve files and display PowerPoint presentations on demand. Connect to database via provided connection string.                                                                                                          |
| [main.rs](https://github.com/Zlatovlaska11/Delta-Notes-Endpoint/blob/master/src/main.rs)         | Auth, database, endpoint, and filehandler. Usher in the Shuttle runtime using `tide` function, and initiate the server with provided connection string.                                                                                                                         |

</details>

<details closed><summary>src.filehalndler</summary>

| File                                                                                                                  | Summary                                                                                                                                                                                                                                                                             |
| ---                                                                                                                   | ---                                                                                                                                                                                                                                                                                 |
| [file_list.rs](https://github.com/Zlatovlaska11/Delta-Notes-Endpoint/blob/master/src/filehalndler/file_list.rs)       | Navigate through directory structures, retrieving lists of PowerPoint files associated with specific courses. This Rust module, located at `src/filehalndler/file_list.rs`, provides functionality to identify and collect course-specific presentation files.                      |
| [file_serving.rs](https://github.com/Zlatovlaska11/Delta-Notes-Endpoint/blob/master/src/filehalndler/file_serving.rs) | Handles file serving for the Delta-Notes-Endpoint application. Fetches requested PPTX files based on their filenames and course IDs. Sends files with correct content types directly to clients or redirects them to MS Office online viewers.                                      |
| [get_courses.rs](https://github.com/Zlatovlaska11/Delta-Notes-Endpoint/blob/master/src/filehalndler/get_courses.rs)   | Navigate through directory structures in the Delta-Notes-Endpoint repository to locate and retrieve specific files. In `src/filehalndler/get_courses.rs`, develop functionality for accessing course directories by ID, and subsequently retrieving files within those directories. |
| [mod.rs](https://github.com/Zlatovlaska11/Delta-Notes-Endpoint/blob/master/src/filehalndler/mod.rs)                   | Courses` module to obtain filepaths.                                                                                                                                                                                                                                                |
| [test.rs](https://github.com/Zlatovlaska11/Delta-Notes-Endpoint/blob/master/src/filehalndler/test.rs)                 | Src/filehalndler/test.rs catalyzes functional verification of Delta-Notes-Endpoints filehandler component. It utilizes `get_courses` testing utilities for effective assessment within this repositorys architecture.                                                               |

</details>

---

##  Getting Started

**System Requirements:**

* **Rust**: `version x.y.z`

###  Installation

<h4>From <code>source</code></h4>

> 1. Clone the Delta-Notes-Endpoint repository:
>
> ```console
> $ git clone https://github.com/Zlatovlaska11/Delta-Notes-Endpoint
> ```
>
> 2. Change to the project directory:
> ```console
> $ cd Delta-Notes-Endpoint
> ```
>
> 3. Install the dependencies:
> ```console
> $ cargo build
> ```

###  Usage

<h4>From <code>source</code></h4>

> Run Delta-Notes-Endpoint using the command below:
> ```console
> $ cargo run
> ```

###  Tests

> Run the test suite using the command below:
> ```console
> $ cargo test
> ```

---

##  Project Roadmap

- [X] `► INSERT-TASK-1`
- [ ] `► INSERT-TASK-2`
- [ ] `► ...`

---

##  Contributing

Contributions are welcome! Here are several ways you can contribute:

- **[Report Issues](https://github.com/Zlatovlaska11/Delta-Notes-Endpoint/issues)**: Submit bugs found or log feature requests for the `Delta-Notes-Endpoint` project.
- **[Submit Pull Requests](https://github.com/Zlatovlaska11/Delta-Notes-Endpoint/blob/main/CONTRIBUTING.md)**: Review open PRs, and submit your own PRs.
- **[Join the Discussions](https://github.com/Zlatovlaska11/Delta-Notes-Endpoint/discussions)**: Share your insights, provide feedback, or ask questions.

<details closed>
<summary>Contributing Guidelines</summary>

1. **Fork the Repository**: Start by forking the project repository to your github account.
2. **Clone Locally**: Clone the forked repository to your local machine using a git client.
   ```sh
   git clone https://github.com/Zlatovlaska11/Delta-Notes-Endpoint
   ```
3. **Create a New Branch**: Always work on a new branch, giving it a descriptive name.
   ```sh
   git checkout -b new-feature-x
   ```
4. **Make Your Changes**: Develop and test your changes locally.
5. **Commit Your Changes**: Commit with a clear message describing your updates.
   ```sh
   git commit -m 'Implemented new feature x.'
   ```
6. **Push to github**: Push the changes to your forked repository.
   ```sh
   git push origin new-feature-x
   ```
7. **Submit a Pull Request**: Create a PR against the original project repository. Clearly describe the changes and their motivations.
8. **Review**: Once your PR is reviewed and approved, it will be merged into the main branch. Congratulations on your contribution!
</details>

<details closed>
<summary>Contributor Graph</summary>
<br>
<p align="center">
   <a href="https://github.com{/Zlatovlaska11/Delta-Notes-Endpoint/}graphs/contributors">
      <img src="https://contrib.rocks/image?repo=Zlatovlaska11/Delta-Notes-Endpoint">
   </a>
</p>
</details>

---

##  License

This project is protected under the [SELECT-A-LICENSE](https://choosealicense.com/licenses) License. For more details, refer to the [LICENSE](https://choosealicense.com/licenses/) file.

---

##  Acknowledgments

- List any resources, contributors, inspiration, etc. here.

[**Return**](#-overview)

---
