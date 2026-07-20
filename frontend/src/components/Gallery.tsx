import { useEffect, useState, useCallback } from "react";
import { listJobs } from "../api/jobs";
import { useAuth } from "../context/AuthContext";
import type { Job } from "../types";
import JobCard from "./JobCard";

export default function Gallery({ refreshKey }: { refreshKey: number }) {
  const [jobs, setJobs] = useState<Job[]>([]);
  const { token } = useAuth();

  const fetchJobs = useCallback(async () => {
    if (!token) return;
    const data = await listJobs(token);
    setJobs(data);
  }, [token]);

  useEffect(() => {
    fetchJobs();
  }, [fetchJobs, refreshKey]);

  return (
    <div style={{ display: "grid", gridTemplateColumns: "repeat(auto-fill, minmax(200px, 1fr))", gap: 12 }}>
      {jobs.map((job) => (
        <JobCard key={job.id} job={job} />
      ))}
    </div>
  );
}