let tpsChart, latencyChart, failRateChart;
let currentData = [];

async function fetchPerfRuns() {
  const res = await fetch('/api/perf-runs');
  return await res.json();
}

function prepareData(data) {
  return data.map(row => ({
    ...row,
    x_label: row.release_tag + (row.row_no ? '-' + row.row_no : ''),
    avg_tps: Number(row.avg_tps),
    peak_tps: Number(row.peak_tps),
    baseline_avg_tps: Number(row.baseline_avg_tps),
    failed_txn_pct: Number(row.failed_txn_pct)
  }));
}

function showScenario(idx) {
  const card = document.getElementById('scenarioCard');
  const text = document.getElementById('scenarioText');
  if (currentData[idx] && currentData[idx].test_scenario) {
    text.textContent = currentData[idx].test_scenario;
    card.style.display = '';
  } else {
    text.textContent = '';
    card.style.display = 'none';
  }
}

function renderTPSChart(data) {
  const ctx = document.getElementById('tpsChart').getContext('2d');
  tpsChart = new Chart(ctx, {
    type: 'line',
    data: {
      labels: data.map(row => row.x_label),
      datasets: [
        {
          label: 'Average TPS',
          data: data.map(row => row.avg_tps),
          borderColor: '#8884d8',
          backgroundColor: 'rgba(136,132,216,0.1)',
          fill: false,
          tension: 0.1
        },
        {
          label: 'Peak TPS',
          data: data.map(row => row.peak_tps),
          borderColor: '#ff9800',
          backgroundColor: 'rgba(255,152,0,0.1)',
          fill: false,
          tension: 0.1
        },
        {
          label: 'Baseline Avg TPS',
          data: data.map(row => row.baseline_avg_tps),
          borderColor: '#43a047',
          backgroundColor: 'rgba(67,160,71,0.1)',
          borderDash: [6,3],
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
        x: { title: { display: true, text: 'Release Tag' } },
        y: { title: { display: true, text: 'TPS' } }
      },
      onClick: (evt, elements) => {
        if (elements.length) {
          const idx = elements[0].index;
          if (latencyChart && failRateChart) {
            latencyChart.setActiveElements([{datasetIndex: 0, index: idx}]);
            failRateChart.setActiveElements([{datasetIndex: 0, index: idx}]);
            latencyChart.tooltip.setActiveElements([{datasetIndex: 0, index: idx}], {x:0,y:0});
            failRateChart.tooltip.setActiveElements([{datasetIndex: 0, index: idx}], {x:0,y:0});
            latencyChart.update();
            failRateChart.update();
          }
          showScenario(idx);
        }
      }
    }
  });
}

function renderLatencyChart(data) {
  const ctx = document.getElementById('latencyChart').getContext('2d');
  latencyChart = new Chart(ctx, {
    type: 'line',
    data: {
      labels: data.map(row => row.x_label),
      datasets: [{
        label: 'P95 Latency (ms)',
        data: data.map(row => row.p95_latency_ms),
        borderColor: '#82ca9d',
        backgroundColor: 'rgba(130,202,157,0.1)',
        fill: true,
        tension: 0.1
      }]
    },
    options: {
      responsive: true,
      plugins: {
        legend: { position: 'top' },
        title: { display: false }
      },
      scales: {
        x: { title: { display: true, text: 'Release Tag' } },
        y: { title: { display: true, text: 'Latency (ms)' } }
      }
    }
  });
}

function renderFailRateChart(data) {
  const ctx = document.getElementById('failRateChart').getContext('2d');
  failRateChart = new Chart(ctx, {
    type: 'line',
    data: {
      labels: data.map(row => row.x_label),
      datasets: [{
        label: 'Failed Rate (%)',
        data: data.map(row => row.failed_txn_pct * 100),
        borderColor: '#e53935',
        backgroundColor: 'rgba(229,57,53,0.1)',
        fill: true,
        tension: 0.1
      }]
    },
    options: {
      responsive: true,
      plugins: {
        legend: { position: 'top' },
        title: { display: false }
      },
      scales: {
        x: { title: { display: true, text: 'Release Tag' } },
        y: { title: { display: true, text: 'Failed Rate (%)' } }
      }
    }
  });
}

fetchPerfRuns().then(raw => {
  const data = prepareData(raw);
  currentData = data;
  renderTPSChart(data);
  renderLatencyChart(data);
  renderFailRateChart(data);
});
