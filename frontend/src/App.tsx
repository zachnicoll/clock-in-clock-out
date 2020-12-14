import React from 'react';
import './Styles/Imports.scss';
import Axios from 'axios';
import { AuthProvider } from './Helpers/AuthContext';
import AppRouter from './AppRouter';

Axios.defaults.baseURL = process.env.NODE_ENV === 'production' ? 'http://clockinout.net/' : 'http://localhost:8000/';

function App() {
  return (
    <AuthProvider>
      <AppRouter />
    </AuthProvider>
  );
}

export default App;
