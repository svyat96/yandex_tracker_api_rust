# Yandex Tracker API Integration

**Version: 0.1.1**

Author: Svyatoslav Sporyhin  
Email: svyat1996@gmail.com

## Overview

This project provides a Rust-based solution for integrating with the Yandex Tracker API. It includes functionality for authentication, task management (creating, updating, deleting tasks), and batch processing of tasks using Yandex Tracker.

## Features

- **Authentication**: Handles OAuth 2.0 authentication with Yandex, including token management.
- **Task Management**: Create, update, and delete tasks in Yandex Tracker.
- **Batch Processing**: Process tasks in batches from a JSON file.
- **Configuration**: Load configuration from a TOML file.

## Getting Started

### Prerequisites

- Rust (latest stable version)
- Yandex account with Tracker API access
- `cargo` for Rust package management

### Installation

1. Clone the repository:
```bash
    git clone https://github.com/yourusername/yandex-tracker-api-integration.git
    cd yandex-tracker-api-integration
```

2. Install dependencies:
```bash
    cargo build
```

3. Create a `config.toml` file in the project root with the following content:
```toml
    organization_id = "your_org_id"
    yandex_client_id = "your_client_id"
    yandex_client_secret = "your_client_secret"
    redirect_uri = "http://localhost:8080/redirect"
```

4. Create a `tasks.json` file in the project root with the desired tasks to be processed:

```json
    {
        "created": [
            {
                "queue": "YOUR_QUEUE_NAME",
                "summary": "Example task",
                "description": "This is an example task",
                "type": "task",
                "assignee": "assignee_email",
                "priority": "normal"
            }
        ],
        "updated": [
            {
                "issue_id": "TASK-123",
                "mut_task": {
                    "summary": "Updated task summary",
                    "description": "Updated description",
                    "type": "task",
                    "assignee": "new_assignee_email",
                    "priority": "high"
                }
            }
        ],
        "deleted": [
            "TASK-124"
        ]
    }
```

### Usage

To run the project:

```bash
cargo run
```

The application will:

1. Load the configuration from `config.toml`.
2. Check for a local token or initiate the OAuth flow to obtain a new token.
3. Process tasks specified in `tasks.json`.

### Project Structure

- **main.rs**: Entry point of the application.
- **config.rs**: Handles configuration loading.
- **modules/authorization**: Contains authentication-related functionality.
    - `auth_error.rs`: Defines errors related to authentication.
    - `token_response.rs`: Manages token saving/loading.
- **modules/task**: Contains task management functionality.
    - `task_batch.rs`: Manages batch processing of tasks.
    - `task_manager.rs`: Handles creating, updating, and deleting tasks.

### Example

To add a new task, update the `tasks.json` file as follows:

```json
{
    "created": [
        {
            "queue": "YOUR_QUEUE_NAME",
            "summary": "New task example",
            "description": "Description of the new task",
            "type": "task",
            "assignee": "assignee_email",
            "priority": "high"
        }
    ],
    "updated": [],
    "deleted": []
}
```

To update an existing task, update the `tasks.json` file as follows:

```json
{
    "created": [],
    "updated": [
        {
            "issue_id": "TASK-123",
            "mut_task": {
                "summary": "Updated task summary",
                "description": "Updated description",
                "type": "task",
                "assignee": "new_assignee_email",
                "priority": "high"
            }
        }
    ],
    "deleted": []
}
```

To delete a task, update the `tasks.json` file as follows:

```json
{
    "created": [],
    "updated": [],
    "deleted": [
        "TASK-124"
    ]
}
```

Run the application to process the tasks.

## Future Plans

- **Enhanced Error Handling**: Improve error handling and logging throughout the application.
- **Unit Tests**: Add comprehensive unit tests for all modules.
- **Configurable Task Attributes**: Allow more customizable task attributes based on user requirements.
- **Continuous Integration**: Set up CI/CD pipelines for automated testing and deployment.
- **Additional API Endpoints**: Expand functionality to cover more Yandex Tracker API endpoints.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request on GitHub.

## License

This project is licensed under the MIT License.

## Contact

For questions or suggestions, please contact Svyatoslav Sporyhin at svyat1996@gmail.com.
