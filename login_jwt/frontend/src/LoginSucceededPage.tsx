import axios from "axios";
import React from "react";
import { TokensContext } from "./LoginPage";

const LoginSucceededPage: React.FC = () => {
  const tokenContext = React.useContext(TokensContext);

  const [hello, setHello] = React.useState("");

  const handleSayHello = () => {
    const url = "/hello/say/";
    axios.post(url, {}).then((res) => {
      setHello(res.data);
    });
  };

  const handleSecretSayHello = () => {
    const url = "/hello/secret_say";
    axios
      .post(
        url,
        {},
        {
          headers: {
            Authorization: `JWT ${tokenContext.accessToken}`,
          },
        }
      )
      .then((res) => {
        setHello(res.data);
      });
  };

  return (
    <div>
      <div>Hello</div>
      <div>
        <div>access</div>
        <div>{tokenContext.accessToken}</div>
      </div>
      <div>
        <div>refresh</div>
        <div>{tokenContext.refreshToken}</div>
      </div>
      <div>
        <button onClick={handleSayHello}>Say Hello</button>
        <button onClick={handleSecretSayHello}>Secret Say Hello</button>
        {hello}
      </div>
    </div>
  );
};

export default LoginSucceededPage;
