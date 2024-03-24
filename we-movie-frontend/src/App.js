// src/components/GoogleOAuthLoginPage.js
import React from 'react';

const GoogleOAuthLoginPage = () => {
 const clientId = process.env.REACT_APP_GOOGLE_OAUTH_CLIENT_ID;
 const redirectUri = encodeURIComponent('http://localhost:8080/oauth_callback');
 const scope = encodeURIComponent('email profile');
 const responseType = 'code';
 const state = process.env.REACT_APP_GOOGLE_OAUTH_CLIENT_SECRET;

 const handleLogin = () => {
   console.log("Google Client ID:" + clientId);
   console.log(state);
   const url = `https://accounts.google.com/o/oauth2/v2/auth?client_id=${clientId}&redirect_uri=${redirectUri}&scope=${scope}&response_type=${responseType}&state=${state}`;
    window.location.href = url;
 };

 return (
    <div>
      <h2>Login with Google</h2>
      <button onClick={handleLogin}>Login</button>
    </div>
 );
};

export default GoogleOAuthLoginPage;