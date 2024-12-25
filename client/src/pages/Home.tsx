import React, { useEffect, useState } from 'react'
import '../styles/Signup.css'
import Navbar from '../components/Navbar.tsx';



function Home() {  
    const [image, imageSetter] = useState("")
    const [message, messageSetter] = useState("Loading...")
    
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
        
        fetch("http://localhost:4001/api/home", {
            method: "GET",
            headers: headers
        }).then(response => {
            if (response.ok) {
                messageSetter("Welcome back, " + response.headers.get("X-Username") + "!")
                response.blob().then(blob => {
                    imageSetter(URL.createObjectURL(blob))
                })
            } else {
                messageSetter("Hai! This is an account system made with TypeScript + React for the frontend and Rust + ActixWeb for the backend. The users are stored in a Postgresql database, and the passwords are secured with bcrypt hashing. When you login, an authentication token will be saved to your cookies, which will allow you to see an amazing image of a green goose when you return to this page. I hope you enjoy!!!!!")
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
                {image && <img src={image} alt="Green Goose"/>}
            </div>
        </div>
        </>
        
    )  
}

export default Home