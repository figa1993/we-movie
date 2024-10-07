import React, { useEffect, useState } from 'react';

function UserHomePage() {
  const [content, setContent] = useState('');

  useEffect(() => {
    const fetchProtectedContent = async () => {
      const token = localStorage.getItem('token');
      const response = await fetch('http://localhost:8080/user_home_page', {
        headers: {
          'Authorization': `Bearer ${token}`
        }
      });

      if (response.ok) {
        const data = await response.text();
        setContent(data);
      } else {
        setContent('Failed to fetch protected content');
      }
    };

    fetchProtectedContent();
  }, []);

  return (
    <div>
      <h1>User Home Page</h1>
      <p>Welcome {content}</p>
    </div>
  );
}

export default UserHomePage;