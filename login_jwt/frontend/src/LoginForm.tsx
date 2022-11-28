import React from "react";
import axios from "axios";

import { useNavigate } from "react-router-dom";
import { TokensContext } from "./LoginPage";

axios.defaults.baseURL = "api/";

const LoginForm = (): JSX.Element => {
  const [username, setUsername] = React.useState("");
  const [password, setPassword] = React.useState("");

  const navigate = useNavigate();

  const tokensContext = React.useContext(TokensContext);

  const handleSubmit = () => {
    console.info(username, password);

    const url = "/token/";
    axios.post(url, { username, password }).then((res) => {
      tokensContext.setAccessToken(res.data.access);
      tokensContext.setRefreshToken(res.data.refresh);
      navigate("/login_succeeded");
    });
  };

  return (
    <div>
      <div>
        <div>username</div>
        <div>
          <input
            type="text"
            name="username"
            value={username}
            onChange={(e) => {
              setUsername(e.target.value);
            }}
          />
        </div>
      </div>
      <div>
        <div>password</div>
        <div>
          <input
            type="text"
            name="password"
            value={password}
            onChange={(e) => {
              setPassword(e.target.value);
            }}
          />
        </div>
      </div>
      <div>
        <button onClick={handleSubmit}>login</button>
      </div>
      <div>
        <p>access: {tokensContext.accessToken}</p>
        <p>refresh: {tokensContext.refreshToken}</p>
      </div>
    </div>
  );
};

export default LoginForm;
