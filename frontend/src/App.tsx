import React from 'react';
import './Styles/Imports.scss';
import Axios from 'axios';
import { AuthProvider } from './Helpers/AuthContext';
import AppRouter from './AppRouter';

Axios.defaults.baseURL = 'http://localhost:8000/';

function App() {
  return (
    <AuthProvider>
      <AppRouter />
    </AuthProvider>
  );
}

export default App;
