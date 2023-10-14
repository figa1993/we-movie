// import React, { useEffect, useState } from 'react';
// import axios from 'axios';

// function App() {
//     const [message, setMessage] = useState("");
//     const [inputValue, setInputValue] = useState('');

//     // An event handler for when the submit button is pressed
//     const handleSubmit = async (event) => {
//       event.preventDefault();
//       try {
//         const response = await axios.post('http://localhost:8080/create-user', { data: inputValue });
//         console.log(response.data);
//         // handle response here
//       } catch (error) {
//         console.error(error);
//         // handle error here
//       }
//     };

//     useEffect(() => {
//         axios.get('http://localhost:8080')
//             .then(response => {
//                 setMessage(response.data);
//             });
//     }, []);

//     return (
//       <form onSubmit={handleSubmit} style={{ display: 'flex', flexDirection: 'column', alignItems: 'center' }}>
//         <h1>We Movie Sign Up</h1>
//         <label>Select Username</label>
//         <input
//           type="text"
//           value={inputValue}
//           onChange={e => setInputValue(e.target.value)}
//           style={{ margin:'10px 0'}}
//         />
//         <button type="submit">Submit</button>
//       </form>
//     );
// }

import React, { useState, useEffect } from 'react';

const Users = () => {

  const [users, setUsers] = useState([]);
  const [jsonResponse, setJsonResponse] = useState('');

  useEffect(() => {
    fetch('http://localhost:8080/users')
      .then(response => response.json())
      .then(data => {
        setUsers(data);
        setJsonResponse(JSON.stringify(data, null, 2));
      });
  }, []);

  return (
    <ul>
      {users.map((user, index) => (
        <li key={index}>{user}</li>
      ))}
    </ul>
  );
};

export default Users;