import { useState } from "react";
import { useAuth } from "./context/AuthContext";
import LoginPage from "./pages/LoginPage";
import SignupPage from "./pages/SignupPage";
import PromptForm from "./components/PromptForm";
import Gallery from "./components/Gallery";

export default function App() {
  const { isAuthenticated, setToken } = useAuth();
  const [showSignup, setShowSignup] = useState(false);
  const [refreshKey, setRefreshKey] = useState(0);

  if (!isAuthenticated) {
    return showSignup ? (
      <SignupPage onSwitch={() => setShowSignup(false)} />
    ) : (
      <LoginPage onSwitch={() => setShowSignup(true)} />
    );
  }

  return (
    <div>
      <button onClick={() => setToken(null)}>Log out</button>
      <PromptForm onGenerated={() => setRefreshKey((k) => k + 1)} />
      <Gallery refreshKey={refreshKey} />
    </div>
  );
}