import React from 'react';
import { BrowserRouter as Router, Routes, Route } from 'react-router-dom'

import Signup from './pages/Signup.tsx'
import Login from './pages/Login.tsx'
import Home from './pages/Home.tsx'

function App() {
  return (
    <Router>
      <Routes>
        <Route path="/signup" element={<Signup/>}/>
        <Route path="/login" element={<Login/>}/>
        <Route path="/" element={<Home/>}/>
      </Routes>
    </Router>
  );
}

export default App;
