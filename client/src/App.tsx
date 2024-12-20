import React from 'react';
import { BrowserRouter as Router, Routes, Route } from 'react-router-dom'

import Signup from './pages/Signup.tsx'

function App() {
  return (
    <Router>
      <Routes>
        <Route path="/" element={<Signup/>}/>
      </Routes>
    </Router>
  );
}

export default App;
