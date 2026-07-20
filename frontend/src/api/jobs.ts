import request from "./client";
import type { Job } from "../types";

export function generate(prompt: string, token: string) {
  return request<{ image_url: string }>(
    "/generate",
    { method: "POST", body: JSON.stringify({ prompt }) },
    token
  );
}

export function listJobs(token: string) {
  return request<Job[]>("/jobs", { method: "GET" }, token);
}

export function getJob(id: string, token: string) {
  return request<Job>(`/jobs/${id}`, { method: "GET" }, token);
}