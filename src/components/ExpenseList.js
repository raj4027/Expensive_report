import React, { useEffect, useState } from "react";
import { fetchExpenses, deleteExpense } from "../api";

function ExpenseList() {
  const [expenses, setExpenses] = useState([]);

  useEffect(() => {
    async function loadExpenses() {
      const data = await fetchExpenses();
      setExpenses(data);
    }
    loadExpenses();
  }, []);

  async function handleDelete(id) {
    await deleteExpense(id);
    setExpenses(expenses.filter((expense) => expense.id !== id));
  }

  return (
    <div>
      <h2 className="text-red-500">Expenses</h2>
      <ul>
        {expenses.map((expense) => (
          <li key={expense.id}>
            {expense.amount} - {expense.category} - {expense.description} -{" "}
            {expense.expense_date}
            <button onClick={() => handleDelete(expense.id)}>Delete</button>
          </li>
        ))}
      </ul>
    </div>
  );
}

export default ExpenseList;

