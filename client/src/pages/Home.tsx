import React, { useEffect, useState } from 'react'
import '../styles/Signup.css'
import Navbar from '../components/Navbar.tsx';



function Home() {  
    const [image, imageSetter] = useState("")
    const [message, messageSetter] = useState(
        "Hai! This is an account system made with TypeScript + React for the frontend and Rust + ActixWeb for the backend. The users are stored in a Postgresql database, and the passwords are secured with AES encryption. When you login, an authentication token will be saved to your cookies, which will allow you to see an amazing image of a green goose when you return to this page. I hope you enjoy!!!!!"
    )
    
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

    useEffect(() => {
        const headers = new Headers();
        headers.append('Authorization', `Bearer ${getCookie("token")}`)
        
        fetch("https://accountsystembackend.nibbles.hackclub.app/api/home", {
            method: "GET",
            headers: headers
        }).then(response => {
            if (response.ok) {
                response.json().then(data => {
                    imageSetter(data.goose)
                    messageSetter("Welcome back, " + data.username + "!")
                }) 
            }
        })
    }, []) // what the react
    
    return (
        <>
        <Navbar/>
        <div className='wrapper'>
            
            <div className='container'>
                <h1>Home</h1>

                <p>{message}</p>
                {image && <img src={`data:image/jpeg;base64,${image}`} alt="Green Goose"/>}
            </div>
        </div>
        </>
        
    )  
}

export default Home