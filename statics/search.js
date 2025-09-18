async function fetchPerfRuns() {
  const res = await fetch('/api/perf-runs');
  return await res.json();
}

function renderTable(data) {
  const tableHeader = document.getElementById('tableHeader');
  const tableBody = document.getElementById('tableBody');
  tableHeader.innerHTML = '';
  tableBody.innerHTML = '';
  if (!data.length) {
    tableBody.innerHTML = '<tr><td colspan="99" class="text-center">No data</td></tr>';
    return;
  }
  // Render headers
  Object.keys(data[0]).forEach(key => {
    const th = document.createElement('th');
    th.textContent = key;
    tableHeader.appendChild(th);
  });
  // Render rows
  data.forEach(row => {
    const tr = document.createElement('tr');
    Object.keys(row).forEach(key => {
      const td = document.createElement('td');
      let value = row[key];
      if (value === null || value === undefined || value === 'null') value = '';
      td.innerHTML = String(value).replace(/\n/g, '<br>');
      td.style.whiteSpace = 'pre-line';
      tr.appendChild(td);
    });
    tableBody.appendChild(tr);
  });
  // Init DataTable (destroy previous if exists)
  if ($.fn.DataTable.isDataTable('#perfTable')) {
    $('#perfTable').DataTable().destroy();
  }
  $('#perfTable').DataTable({
    paging: true,
    searching: false,
    info: true,
    scrollX: true,
    colReorder: true,
    colResize: true,
    autoWidth: false
  });
}

function filterData(data, filters) {
  return data.filter(row => {
    if (filters.release_tag && !String(row.release_tag).includes(filters.release_tag)) return false;
    if (filters.avg_tps_min && Number(row.avg_tps) < Number(filters.avg_tps_min)) return false;
    if (filters.avg_tps_max && Number(row.avg_tps) > Number(filters.avg_tps_max)) return false;
    if (filters.p95_latency_max && Number(row.p95_latency_ms) > Number(filters.p95_latency_max)) return false;
    if (filters.failed_txn_pct_max && Number(row.failed_txn_pct) * 100 > Number(filters.failed_txn_pct_max)) return false;
    return true;
  });
}

document.addEventListener('DOMContentLoaded', async () => {
  let allData = await fetchPerfRuns();
  renderTable(allData);

  document.getElementById('filterForm').addEventListener('submit', function(e) {
    e.preventDefault();
    const form = e.target;
    const filters = {
      release_tag: form.release_tag.value.trim(),
      avg_tps_min: form.avg_tps_min.value,
      avg_tps_max: form.avg_tps_max.value,
      p95_latency_max: form.p95_latency_max.value,
      failed_txn_pct_max: form.failed_txn_pct_max.value
    };
    const filtered = filterData(allData, filters);
    renderTable(filtered);
  });
});
