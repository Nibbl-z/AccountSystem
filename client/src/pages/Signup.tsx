import React, { useState } from 'react'
import '../styles/Style.css'
import Navbar from '../components/Navbar.tsx'

function Signup() {
    const [statusMessage, setStatusMessage] = useState("")
    const [statusColor, setStatusColor] = useState("statusSuccess")
    
    function OnSignup(event) {
        event.preventDefault()
        
        const formData = new FormData(event.target)
        const username = formData.get("username")
        const password = formData.get("password")
    
        const headers = new Headers();
        headers.append('Content-Type', 'application/json')
        
        
        fetch('https://accountsystembackend.nibbles.hackclub.app/api/signup', {
            "headers" : headers,
            "method" : "POST",
            "body" : JSON.stringify({'username' : username, 'password' : password})
        }).then(response => { 
            response.text().then(result => {
                setStatusColor(response.ok ? "statusSuccess" : "statusError")
                setStatusMessage(result)
                
                if (response.ok) {
                    setTimeout(() => {
                        window.location.replace("/login")
                    }, 2000)
                }
            })
        })
    }
    
    return (
        <>
        <Navbar/>
        <div className='wrapper'>
            <div className='container'>
                <h1>Signup</h1>
                <br />
                <form onSubmit={OnSignup}>
                    <label htmlFor="username">Username:</label>
                    <input type="text" name="username"/>
                    <label htmlFor="password">Password:</label>
                    <input type="password" name="password"/>
                    {statusMessage !== "" && <label htmlFor="submit" className={statusColor}>{statusMessage}</label>}
                    <input type="submit" value="Signup" className="submitButton"/>
                </form>
            </div>
        </div>
        </>
    )  
}

export default Signup