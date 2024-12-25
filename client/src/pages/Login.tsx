import React, { useState } from 'react'
import '../styles/Style.css'
import Navbar from '../components/Navbar.tsx'



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
        
        fetch('https://accountsystembackend.nibbles.hackclub.app/api/login', {
            "headers" : headers,
            "method" : "POST",
            "body" : JSON.stringify({'username' : username, 'password' : password})
        }).then(response => { 
            response.json().then(result => {
                setStatusColor(response.ok ? "statusSuccess" : "statusError")
                
                if (response.ok) {
                    document.cookie = `token=${result}; path=/;`
                    setStatusMessage("Logged in successfully!")

                    setTimeout(() => {
                        window.location.replace("/")
                    }, 2000)
                } else {
                    setStatusMessage(result)
                }
            })
        })
    }
    
    return (
        <>
        <Navbar/>
        <div className='wrapper'>
            <div className='container'>
                <h1>Login</h1>
                <br />
                <form onSubmit={OnLogin}>
                    <label htmlFor="username">Username:</label>
                    <input type="text" name="username"/>
                    <label htmlFor="password">Password:</label>
                    <input type="password" name="password"/>
                    {statusMessage !== "" && <label htmlFor="submit" className={statusColor}>{statusMessage}</label>}
                    <input type="submit" value="Login" className="submitButton"/>
                </form>
            </div>
        </div>
        </>
    )  
}

export default Login