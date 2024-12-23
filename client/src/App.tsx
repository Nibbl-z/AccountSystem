import React from 'react';
import { BrowserRouter as Router, Routes, Route } from 'react-router-dom'

import Signup from './pages/Signup.tsx'
import Login from './pages/Login.tsx'
import TestAuthorization from './pages/TestAuthorization.tsx'

function App() {
  return (
    <Router>
      <Routes>
        <Route path="/" element={<Signup/>}/>
        <Route path="/login" element={<Login/>}/>
        <Route path="/test" element={<TestAuthorization/>}/>
      </Routes>
    </Router>
  );
}

export default App;
