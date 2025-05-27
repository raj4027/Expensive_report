import React, { useState } from "react";
import { createExpense, updateExpense } from "../api";

function ExpenseForm({ expense, onSave }) {
  const [formData, setFormData] = useState(
    expense || {
      amount: "",
      description: "",
      category: "Others",
      expense_date: new Date().toISOString().split("T")[0],
    }
  );

  function handleChange(e) {
    const { name, value } = e.target;
    setFormData({ ...formData, [name]: value });
  }

  async function handleSubmit(e) {
    e.preventDefault();
    if (expense) {
      await updateExpense(expense.id, formData);
    } else {
      await createExpense(formData);
    }
    onSave();
  }

  return (
    <form onSubmit={handleSubmit}>
      <label>
        Amount:
        <input
          type="number"
          name="amount"
          value={formData.amount}
          onChange={handleChange}
          required
        />
      </label>
      <label>
        Description:
        <textarea
          name="description"
          value={formData.description}
          onChange={handleChange}
        />
      </label>
      <label>
        Category:
        <select
          name="category"
          value={formData.category}
          onChange={handleChange}
        >
          <option>Work</option>
          <option>Personal</option>
          <option>Food</option>
          <option>Transport</option>
          <option>Utilities</option>
          <option>Entertainment</option>
          <option>Others</option>
        </select>
      </label>
      <label>
        Date:
        <input
          type="date"
          name="expense_date"
          value={formData.expense_date}
          onChange={handleChange}
          required
        />
      </label>
      <button type="submit">Save</button>
    </form>
  );
}

export default ExpenseForm;