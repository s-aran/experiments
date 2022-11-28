import React from "react";
import logo from "./logo.svg";
import "./App.css";
import LoginForm from "./LoginForm";
import { Route, Routes } from "react-router-dom";
import LoginSucceededPage from "./LoginSucceededPage";
import LoginPage, { TokensContext } from "./LoginPage";

function App() {
  const [accessToken, setAccessToken] = React.useState("");
  const [refreshToken, setRefreshToken] = React.useState("");

  return (
    <TokensContext.Provider
      value={{ accessToken, setAccessToken, refreshToken, setRefreshToken }}
    >
      <Routes>
        <Route
          path="/"
          element={
            <div className="App">
              <header className="App-header">
                <img src={logo} className="App-logo" alt="logo" />
                <p>
                  Edit <code>src/App.tsx</code> and save to reload.
                </p>
                <a
                  className="App-link"
                  href="https://reactjs.org"
                  target="_blank"
                  rel="noopener noreferrer"
                >
                  Learn React
                </a>
                <LoginForm />
              </header>
            </div>
          }
        />
        <Route path="/login" element={<LoginPage />} />
        <Route path="/login_succeeded" element={<LoginSucceededPage />} />
      </Routes>
    </TokensContext.Provider>
  );
}

export default App;
