import React, { useState, useEffect } from "react";
import { fetchSummary } from "../api";

function ExpenseSummary() {
  const [summary, setSummary] = useState(0);
  const [startDate, setStartDate] = useState("");
  const [endDate, setEndDate] = useState("");

  useEffect(() => {
    async function loadSummary() {
      const data = await fetchSummary(startDate, endDate);
      setSummary(data.total);
    }
    loadSummary();
  }, [startDate, endDate]);

  return (
    <div className="container mt-4">
      <h2>Expense Summary</h2>
      <div className="mb-3">
        <label className="form-label">Start Date:</label>
        <input
          type="date"
          className="form-control"
          value={startDate}
          onChange={(e) => setStartDate(e.target.value)}
        />
      </div>
      <div className="mb-3">
        <label className="form-label">End Date:</label>
        <input
          type="date"
          className="form-control"
          value={endDate}
          onChange={(e) => setEndDate(e.target.value)}
        />
      </div>
      <p>Total Expenses: {summary}</p>
    </div>
  );
}

export default ExpenseSummary;