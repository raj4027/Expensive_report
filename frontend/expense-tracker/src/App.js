import React from "react";
import { BrowserRouter as Router, Routes, Route, Link } from "react-router-dom";
import Homepage from "../src/pages/Homepage";
import ExpensePage from "../src/pages/ExpensePage";

import ExpenseSummary from "../../../src/components/ExpenseSummary"

function App() {
  return (
    <Router>
      <nav>
        <Link to="/">Home</Link> | <Link to="/add-expense">Add Expense</Link> |{" "}
        <Link to="/summary">Summary</Link>
      </nav>
      <Routes>
        <Route path="/" element={<Homepage />} />
        <Route path="/add-expense" element={<ExpensePage />} />
        <Route path="/summary" element={<ExpenseSummary />} />
      </Routes>
    </Router>
  );
}

export default App;