import React from "react";
import { TokensContext } from "./LoginPage";

const LoginSucceededPage: React.FC = () => {
  const tokenContext = React.useContext(TokensContext);

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
    </div>
  );
};

export default LoginSucceededPage;
