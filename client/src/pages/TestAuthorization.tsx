import React from 'react'
import '../styles/Signup.css'



function Login() {  
    function getCookie(name) {
        const value = `; ${document.cookie}`;
        const parts = value.split(`; ${name}=`);
        if (parts.length === 2) {
            const cookieValue = parts.pop();
            if (cookieValue) {
                return cookieValue.split(';').shift();
            }
        }
    }
    
    function OnPress() {
        const headers = new Headers();
        headers.append('Authorization', `Bearer ${getCookie("token")}`)

        fetch("http://localhost:4000/test", {
            method: "GET",
            headers: headers
        }).then(response => response.text())
        .then(data => {
            alert(data)
        })
        
    }
    
    return (
        <div className='wrapper'>
            <div className='container'>
                <h1>Test Auth</h1>
                <br />
                <button onClick={OnPress}>Do the thing</button>
            </div>
        </div>
    )  
}

export default Login