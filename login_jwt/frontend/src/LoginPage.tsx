import React from "react";
import LoginForm from "./LoginForm";

export interface TokensContextType {
  accessToken: string;
  setAccessToken: React.Dispatch<React.SetStateAction<string>>;
  refreshToken: string;
  setRefreshToken: React.Dispatch<React.SetStateAction<string>>;
}

export const TokensContext = React.createContext<TokensContextType>({
  accessToken: "",
  setAccessToken: () => {},
  refreshToken: "",
  setRefreshToken: () => {},
});

const LoginPage = (): JSX.Element => {
  return <LoginForm />;
};

export default LoginPage;
