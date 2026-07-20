export interface Job {
  id: string;
  job_type: string;
  model: string;
  prompt: string;
  status: "pending" | "completed" | "failed";
  result_url: string | null;
  error_message: string | null;
  created_at: string;
  updated_at: string;
}

export interface AuthResponse {
  token: string;
}

export interface ApiError {
  error: string;
}