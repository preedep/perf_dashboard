async function fetchPerfRuns() {
  const res = await fetch('/api/perf-runs');
  return await res.json();
}

async function fetchReleaseTags() {
  const res = await fetch('/api/perf-runs');
  const data = await res.json();
  // get unique release_tag, remove null/empty
  const tags = [...new Set(data.map(r => r.release_tag).filter(Boolean))];
  return tags.sort();
}

function buildTabulatorColumns(data) {
  if (!data.length) return [];
  return Object.keys(data[0]).map(key => ({
    title: key,
    field: key,
    headerFilter: false,
    widthGrow: 1,
    resizable: true,
    headerSort: true,
    formatter: function(cell) {
      let value = cell.getValue();
      if (value === null || value === undefined || value === 'null') value = '';
      return String(value).replace(/\n/g, '<br>');
    }
  }));
}

async function fetchPerfRunsServer(filters = {}) {
  const params = new URLSearchParams();
  if (filters.release_tag) params.append('release_tag', filters.release_tag);
  if (filters.avg_tps_min) params.append('min_avg_tps', filters.avg_tps_min);
  if (filters.avg_tps_max) params.append('max_avg_tps', filters.avg_tps_max);
  if (filters.p95_latency_max) params.append('max_p95_latency_ms', filters.p95_latency_max);
  if (filters.failed_txn_pct_max) params.append('max_failed_txn_pct', filters.failed_txn_pct_max);
  // test_scenario, min_p95_latency_ms, etc. สามารถเพิ่มได้
  const url = '/api/perf-runs' + (params.toString() ? '?' + params.toString() : '');
  const res = await fetch(url);
  return await res.json();
}

function calcTableHeight() {
  // ให้ table สูงประมาณ 80% ของ viewport
  return Math.floor(window.innerHeight * 0.8) + 'px';
}

document.addEventListener('DOMContentLoaded', async () => {
  document.getElementById('filterForm').reset();
  await populateReleaseTagDropdown();
  let allData = await fetchPerfRunsServer();
  const columns = buildTabulatorColumns(allData);
  tabulatorTable = new Tabulator('#perfTable', {
    data: allData,
    columns: columns,
    layout: 'fitColumns',
    movableColumns: true,
    resizableColumns: true,
    height: calcTableHeight(),
    pagination: true,
    paginationSize: 20,
    paginationSizeSelector: [10, 20, 50, 100],
    placeholder: 'No data',
    autoColumns: false,
    columnDefaults: { resizable: true, headerSort: true },
    responsiveLayout: 'collapse',
    responsiveLayoutCollapseStartOpen: false
  });

  window.addEventListener('resize', () => {
    tabulatorTable.setHeight(calcTableHeight());
  });

  document.getElementById('filterForm').addEventListener('submit', async function(e) {
    e.preventDefault();
    const form = e.target;
    const filters = {
      release_tag: form.release_tag.value.trim(),
      avg_tps_min: form.avg_tps_min.value,
      avg_tps_max: form.avg_tps_max.value,
      p95_latency_max: form.p95_latency_max.value,
      failed_txn_pct_max: form.failed_txn_pct_max.value
    };
    const filtered = await fetchPerfRunsServer(filters);
    tabulatorTable.setData(filtered);
  });
});

async function populateReleaseTagDropdown() {
  const select = document.getElementById('releaseTagSelect');
  const tags = await fetchReleaseTags();
  tags.forEach(tag => {
    const opt = document.createElement('option');
    opt.value = tag;
    opt.textContent = tag;
    select.appendChild(opt);
  });
}
