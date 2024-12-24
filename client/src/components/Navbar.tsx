import React from 'react'
import '../styles/Navbar.css'

function Navbar() {
    return <ul>
        <li className="left"><a href="/">Home</a></li>
        <li className="right"><a href="/signup">Signup</a></li>
        <li className="right"><a href="/login">Login</a></li>
    </ul>
}

export default Navbar