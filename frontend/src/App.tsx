import React from 'react';
import './Styles/Imports.scss';
import Axios from 'axios';
import { AuthProvider } from './Helpers/AuthContext';
import AppRouter from './AppRouter';

console.log(process.env);

Axios.defaults.baseURL = process.env.DEV === "0" ? 'http://clockinout.net/' : 'http://localhost:8000/';

function App() {
  return (
    <AuthProvider>
      <AppRouter />
    </AuthProvider>
  );
}

export default App;
