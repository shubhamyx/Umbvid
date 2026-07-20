import request from "./client";
import type { AuthResponse } from "../types";

export function signup(email: string, password: string) {
  return request<AuthResponse>("/signup", {
    method: "POST",
    body: JSON.stringify({ email, password }),
  });
}

export function login(email: string, password: string) {
  return request<AuthResponse>("/login", {
    method: "POST",
    body: JSON.stringify({ email, password }),
  });
}