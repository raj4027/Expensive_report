const API_BASE_URL = "http://127.0.0.1:8080"; // Backend URL

export async function fetchExpenses() {
  const response = await fetch(`${API_BASE_URL}/expenses`);
  return response.json();
}

export async function createExpense(expense) {
  const response = await fetch(`${API_BASE_URL}/expenses`, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify(expense),
  });
  return response.json();
}

export async function updateExpense(id, expense) {
  const response = await fetch(`${API_BASE_URL}/expenses/${id}`, {
    method: "PUT",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify(expense),
  });
  return response.json();
}

export async function deleteExpense(id) {
  const response = await fetch(`${API_BASE_URL}/expenses/${id}`, {
    method: "DELETE",
  });
  return response.ok;
}

export async function fetchSummary(startDate, endDate) {
  const response = await fetch(
    `${API_BASE_URL}/expenses/summary?start_date=${startDate}&end_date=${endDate}`
  );
  return response.json();
}