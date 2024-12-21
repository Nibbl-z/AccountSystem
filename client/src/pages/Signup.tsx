import React from 'react'
import '../styles/Signup.css'



function Signup() {
    function OnSignup(formData) {
        const username = formData.get("username")
        const password = formData.get("password")
    
        const headers = new Headers();
        headers.append('Content-Type', 'application/json');
        
        fetch('http://localhost:4000/signup', {
            "headers" : headers,
            "method" : "POST",
            "body" : JSON.stringify({'username' : username, 'password' : password})
        }).then(response => {
            console.log(response)
        })
    }
    
    return (
        <div className='wrapper'>
            <div className='container'>
                <h1>Signup</h1>
                <br />
                <form action={OnSignup}>
                    <label htmlFor="username">Username:</label>
                    <input type="text" name="username"/>
                    <label htmlFor="password">Password:</label>
                    <input type="password" name="password"/>
                    <input type="submit" value="Submit" className="submitButton"/>
                </form>
            </div>
        </div>
    )  
}

export default Signup