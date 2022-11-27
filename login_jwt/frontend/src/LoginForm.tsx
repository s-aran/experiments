import React from "react";
import axios from "axios";

axios.defaults.baseURL = "api/";

const LoginForm = (): JSX.Element => {
  const [username, setUsername] = React.useState("");
  const [password, setPassword] = React.useState("");

  const [accessToken, setAccessToken] = React.useState("");
  const [refreshToken, setRefreshToken] = React.useState("");

  const handleSubmit = () => {
    console.info(username, password);

    const url = "/token/";
    axios.post(url, { username, password }).then((res) => {
      setAccessToken(res.data.access);
      setRefreshToken(res.data.refresh);
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
        <p>access: {accessToken}</p>
        <p>refresh: {refreshToken}</p>
      </div>
    </div>
  );
};

export default LoginForm;
