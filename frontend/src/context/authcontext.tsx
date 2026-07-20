import { createContext, useContext, useState, ReactNode } from "react";

interface AuthContextValue {
  token: string | null;
  setToken: (token: string | null) => void;
  isAuthenticated: boolean;
}

const AuthContext = createContext<AuthContextValue | undefined>(undefined);

export function AuthProvider({ children }: { children: ReactNode }) {
  const [token, setToken] = useState<string | null>(null);

  return (
    <AuthContext.Provider
      value={{ token, setToken, isAuthenticated: token !== null }}
    >
      {children}
    </AuthContext.Provider>
  );
}

export function useAuth() {
  const ctx = useContext(AuthContext);
  if (!ctx) throw new Error("useAuth must be used within AuthProvider");
  return ctx;
}