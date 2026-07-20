import type { Job } from "../types";

export default function JobCard({ job }: { job: Job }) {
  return (
    <div style={{ border: "1px solid #ccc", padding: 8 }}>
      {job.status === "completed" && job.result_url && (
        <img src={job.result_url} alt={job.prompt} style={{ width: "100%" }} />
      )}
      {job.status === "pending" && <p>Generating...</p>}
      {job.status === "failed" && <p style={{ color: "red" }}>{job.error_message ?? "Failed"}</p>}
      <p>{job.prompt}</p>
    </div>
  );
}