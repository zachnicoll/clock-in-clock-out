import React from 'react';
import './Styles/Imports.scss';
import { HomePage, RegisterPage, LandingPage } from './Pages/Pages';
import Header from './Components/Header';
import {
  BrowserRouter as Router,
  Switch,
  Route,
  Link
} from "react-router-dom";
import Axios from 'axios';

Axios.defaults.baseURL = 'http://localhost:8000/';
Axios.defaults.headers = {
  "Access-Control-Allow-Origin":"*"
};

function App() {
  return (
    <Router>
      <Header />
      
      <Switch>
        <Route exact path="/">
          <LandingPage />
        </Route>
        <Route exact path="/home">
          <HomePage />
        </Route>
        <Route exact path="/register">
          <RegisterPage />
        </Route>
      </Switch>
    </Router>
  );
}

export default App;
