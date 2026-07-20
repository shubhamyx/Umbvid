import { useState, FormEvent } from "react";
import { generate } from "../api/jobs";
import { useAuth } from "../context/AuthContext";
import { ApiClientError } from "../api/client";

export default function PromptForm({ onGenerated }: { onGenerated: () => void }) {
  const [prompt, setPrompt] = useState("");
  const [error, setError] = useState<string | null>(null);
  const [loading, setLoading] = useState(false);
  const { token } = useAuth();

  async function handleSubmit(e: FormEvent) {
    e.preventDefault();
    if (!token) return;
    setError(null);
    setLoading(true);
    try {
      await generate(prompt, token);
      setPrompt("");
      onGenerated();
    } catch (err) {
      setError(err instanceof ApiClientError ? err.message : "Generation failed");
    } finally {
      setLoading(false);
    }
  }

  return (
    <form onSubmit={handleSubmit}>
      <input
        type="text"
        placeholder="Describe the image..."
        value={prompt}
        onChange={(e) => setPrompt(e.target.value)}
        required
      />
      <button type="submit" disabled={loading || !prompt.trim()}>
        {loading ? "Generating..." : "Generate"}
      </button>
      {error && <p style={{ color: "red" }}>{error}</p>}
    </form>
  );
}