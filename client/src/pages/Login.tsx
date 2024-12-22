import React, { useState } from 'react'
import '../styles/Signup.css'



function Login() {
    const [statusMessage, setStatusMessage] = useState("")
    const [statusColor, setStatusColor] = useState("statusSuccess")

    function OnLogin(event) {
        event.preventDefault()
        
        const formData = new FormData(event.target)
        const username = formData.get("username")
        const password = formData.get("password")
    
        const headers = new Headers();
        headers.append('Content-Type', 'application/json')
        
        
        fetch('http://localhost:4000/login', {
            "headers" : headers,
            "method" : "POST",
            "body" : JSON.stringify({'username' : username, 'password' : password})
        }).then(response => { 
            response.text().then(result => {
                setStatusColor(response.ok ? "statusSuccess" : "statusError")
                setStatusMessage(result)
            })
        })
    }
    
    return (
        <div className='wrapper'>
            <div className='container'>
                <h1>Login</h1>
                <br />
                <form onSubmit={OnLogin}>
                    <label htmlFor="username">Username:</label>
                    <input type="text" name="username"/>
                    <label htmlFor="password">Password:</label>
                    <input type="password" name="password"/>
                    <label htmlFor="submit" className={statusColor}>{statusMessage}</label>
                    <input type="submit" value="Login" className="submitButton"/>
                </form>
            </div>
        </div>
    )  
}

export default Login