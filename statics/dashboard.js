// dashboard.js
async function fetchPerfRuns() {
  const res = await fetch('/api/perf-runs');
  return await res.json();
}

function renderChart(data) {
  const ctx = document.getElementById('perfChart').getContext('2d');
  const chart = new Chart(ctx, {
    type: 'line',
    data: {
      labels: data.map(row => row.row_no),
      datasets: [
        {
          label: 'Average TPS',
          data: data.map(row => row.avg_tps),
          borderColor: '#8884d8',
          fill: false,
          tension: 0.1
        },
        {
          label: 'P95 Latency (ms)',
          data: data.map(row => row.p95_latency_ms),
          borderColor: '#82ca9d',
          fill: false,
          tension: 0.1
        }
      ]
    },
    options: {
      responsive: true,
      plugins: {
        legend: { position: 'top' },
        title: { display: false }
      },
      scales: {
        x: { title: { display: true, text: 'Row No' } },
        y: { title: { display: true, text: 'Value' } }
      }
    }
  });
}

fetchPerfRuns().then(renderChart);
