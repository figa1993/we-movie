import React, { useEffect } from 'react';

// useHistory is used for easy redirection
import { useHistory } from 'react-router-dom';

function JWTCallback() {
  const history = useHistory();

  useEffect(() => {
    // Check if the ID Token is set in cookies
    const token = document.cookie.split('; ').find(row => row.startsWith('id_token='));
    if (token) {
      const id_token = token.split('=')[1];
      localStorage.setItem('id_token', id_token); // Optionally store in local storage for easier access
      history.push('/protected'); // redirect to the User Home Page
    } else {
      console.error('No token found in cookies');
    }
  }, [history]);

  return <div>Loading...</div>;
}

export default JWTCallback;