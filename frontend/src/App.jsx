import React, { useEffect, useState } from 'react';

export default function App() {
  const [status, setStatus] = useState(null);

  useEffect(() => {
    fetch("/api/status").then(res => res.json()).then(data => setStatus(data));
  }, []);

  return (
    <div className="p-6 font-mono">
      <h1 className="text-2xl font-bold mb-4">Abyssal Watcher Dashboard</h1>
      <div className="bg-gray-800 p-4 rounded-lg shadow-lg">
        {status ? <pre>{JSON.stringify(status, null, 2)}</pre> : "Loading..."}
      </div>
    </div>
  );
}
