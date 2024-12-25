# Account System

This is a website that contains all the necessary features of an account system, including:
- Signing up
- Logging in
- Storing users in database
- Hashing passwords
- Storing a Json Web Token in your cookies in order to stay logged in

### Technologies Used
- Frontend: TypeScript with React
- Backend: Rust with Actix Web
- Database: PostgreSQL

### Self-Hosting (Frontend)
1. Install NodeJS
2. (Optional) Create a `.env` file to specify the port, eg. `PORT=3001`
3. Run `npm start` in the `/client` directory

### Self-Hosting (Backend)
1. Install Rust
2. Create a PostgreSQL database with a table called users, with the columns `id` (integer + auto-incrementing), `username` (text), and `password` (text)
3. Create a `.env` file to specify the URL to the PostgreSQL database, and a port, with the variable names `DATABASE_URL` and `PORT`.
4. In the frontend, replace all `fetch` URLs with `http://localhost:(PORT)` or whatever URL the backend is running on