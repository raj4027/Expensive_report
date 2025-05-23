Expense Tracker

Objective
Build a full-stack personal expense tracker application using Rust to manage user expenses with CRUD functionality, supporting expense creation, reading, updating, and deletion. The app includes a frontend for user interaction and a backend for data management, providing insights into spending habits.
Tech Stack
Backend: Actix-web (RESTful APIs)
Frontend: Yew (Rust-based frontend framework)
Database: SQLite with SQLx (ORM for database interactions)
Styling: TailwindCSS (responsive design)
Validation: validator crate
HTTP Client: reqwest (for frontend-backend communication)

Requirements
Frontend:
Homepage: Display a list of all recorded expenses, showing amount, category, description (if any), and date.
Expense Form: A dedicated form for creating new expenses and editing existing ones.
Fields: Amount (required, positive decimal number), Description (optional, text area), Category (dropdown with options: Work, Personal, Food, Transport, Utilities, Entertainment, Others), Date (required, defaults to current date).
Actions: Buttons or links to edit or delete individual expense records from the list.
Search/Filter: A search bar to filter expenses by description or category.
Summary View: Display the total expenses for the current month or a user-selected period.

Backend:
RESTful APIs: Implement endpoints for all CRUD operations on expense records.
POST /expenses: Create a new expense.
GET /expenses: Retrieve all expenses, with optional query parameters for filtering.
GET /expenses/{id}: Retrieve a single expense by ID.
PUT /expenses/{id}: Update an existing expense.
DELETE /expenses/{id}: Delete an expense.
GET /expenses/summary: Calculate and return total expenses for a given period or category.
Input Validation: Validate incoming JSON payloads for expense creation/updates.
Error Handling: Provide meaningful JSON error responses (e.g., "Expense not found", "Invalid input data").
JSON Handling: Efficiently parse incoming JSON requests and serialize outgoing JSON responses.

Database:
SQLite Database: Utilize a SQLite database file.
expenses Table:
id (UUID, Primary Key)
amount (REAL, NOT NULL)
description (TEXT, NULLABLE)
category (TEXT, NOT NULL, default: "Others")
expense_date (TEXT, NOT NULL, stores date in ISO 8601 format like 'YYYY-MM-DD')
created_at (TEXT, NOT NULL, timestamp of creation)
updated_at (TEXT, NOT NULL, timestamp of last update)
Features:
Validation:
Amount: Must be a positive number.
Category: Must be one of the predefined options (e.g., "Food", "Transport", "Utilities", "Entertainment", "Others").
Expense Date: Required and must be a valid date format.
Sorting: Expenses displayed on the homepage should be sorted by expense_date in descending order (latest first).
Filtering: Allow users to filter the list of expenses by category or by searching for keywords within the description.
Error Handling: Robust error handling for invalid inputs, database failures, and "not found" scenarios, providing clear messages to the user.
Expense Summary: Calculate and display the sum of expenses for the current month by default, with an option to select a custom date range for the summary.

Bonus:
Visualization: Implement a simple bar chart or pie chart (using a Yew-compatible charting library or custom SVG rendering) to visualize the distribution of expenses across different categories.
Styling:
Responsive Design: Utilize TailwindCSS to ensure the application's layout and components adapt gracefully to various screen sizes (mobile, tablet, desktop).
