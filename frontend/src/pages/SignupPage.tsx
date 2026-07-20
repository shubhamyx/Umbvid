import { useState, FormEvent } from "react";
import { signup } from "../api/auth";
import { useAuth } from "../context/AuthContext";
import { ApiClientError } from "../api/client";

export default function SignupPage({ onSwitch }: { onSwitch: () => void }) {
  const [email, setEmail] = useState("");
  const [password, setPassword] = useState("");
  const [error, setError] = useState<string | null>(null);
  const [loading, setLoading] = useState(false);
  const { setToken } = useAuth();

  async function handleSubmit(e: FormEvent) {
    e.preventDefault();
    setError(null);
    setLoading(true);
    try {
      const res = await signup(email, password);
      setToken(res.token);
    } catch (err) {
      setError(err instanceof ApiClientError ? err.message : "Signup failed");
    } finally {
      setLoading(false);
    }
  }

  return (
    <form onSubmit={handleSubmit}>
      <h2>Sign up</h2>
      <input
        type="email"
        placeholder="Email"
        value={email}
        onChange={(e) => setEmail(e.target.value)}
        required
      />
      <input
        type="password"
        placeholder="Password"
        value={password}
        onChange={(e) => setPassword(e.target.value)}
        required
      />
      {error && <p style={{ color: "red" }}>{error}</p>}
      <button type="submit" disabled={loading}>
        {loading ? "Signing up..." : "Sign up"}
      </button>
      <p>
        Have an account? <button type="button" onClick={onSwitch}>Log in</button>
      </p>
    </form>
  );
}