// Local-only presentation controls. Every report remains readable without scripts.
(() => {
  const themeButton = document.querySelector('#theme-toggle');
  const updateThemeButton = () => {
    const dark = document.documentElement.dataset.theme === 'dark';
    themeButton.setAttribute('aria-label', `Switch to ${dark ? 'light' : 'dark'} theme`);
    themeButton.title = themeButton.getAttribute('aria-label');
    themeButton.querySelector('i').className = `fa-solid fa-${dark ? 'sun' : 'moon'}`;
  };
  updateThemeButton();
  themeButton.addEventListener('click', () => {
    const theme = document.documentElement.dataset.theme === 'dark' ? 'light' : 'dark';
    document.documentElement.dataset.theme = theme;
    try { localStorage.setItem('lcc-theme', theme); } catch (_) { /* Theme still works. */ }
    updateThemeButton();
  });

  // Reset fires before native form values change.
  const bindFilters = (form, update) => {
    form.addEventListener('input', update);
    form.addEventListener('change', update);
    form.addEventListener('submit', event => event.preventDefault());
    form.addEventListener('reset', () => setTimeout(update, 0));
    update();
  };
  const releaseForm = document.querySelector('#release-filters');
  if (releaseForm) {
    const cards = [...document.querySelectorAll('[data-release]')];
    const update = () => {
      const f = new FormData(releaseForm);
      const query = f.get('query').trim().toLowerCase();
      const from = f.get('from'), to = f.get('to'), min = f.get('min'), max = f.get('max');
      const error = (from && to && from > to) ? 'The start date must be on or before the end date.'
        : ((min && Number(min) < 0) || (max && Number(max) < 0)) ? 'Test counts cannot be negative.'
        : (min && max && Number(min) > Number(max)) ? 'The minimum count must not exceed the maximum.' : '';
      document.querySelector('#filter-error').textContent = error;
      document.querySelector('#filter-error').hidden = !error;
      let count = 0;
      cards.forEach(card => {
        const d = card.dataset;
        const match = !error && (f.get('channel') === 'All' || d.channel === f.get('channel'))
          && (!query || (d.channel === 'Nightly' && d.search.toLowerCase().includes(query)))
          && (!from || d.date >= from) && (!to || d.date <= to)
          && (!min || Number(d.passed) >= Number(min)) && (!max || Number(d.passed) <= Number(max));
        card.hidden = !match;
        if (match) count++;
      });
      document.querySelector('#release-count').textContent = `${count} demonstration release${count === 1 ? '' : 's'}`;
      document.querySelector('#no-releases').hidden = count > 0 || !!error;
    };
    bindFilters(releaseForm, update);
    document.querySelector('[data-reset-releases]').addEventListener('click', () => releaseForm.reset());
  }

  const testForm = document.querySelector('#test-filters');
  if (testForm) {
    const groups = [...document.querySelectorAll('.test-group')];
    const rows = new Map(groups.map(group => [group, [...group.querySelectorAll('[data-test]')].map(row => ({row, search: row.textContent.toLowerCase()}))]));
    bindFilters(testForm, () => {
      const f = new FormData(testForm), query = f.get('query').trim().toLowerCase();
      let count = 0;
      groups.forEach(group => {
        let visible = 0;
        rows.get(group).forEach(({row, search}) => {
          const match = (f.get('suite') === 'All' || f.get('suite') === group.dataset.suite)
            && (f.get('result') === 'All' || f.get('result') === row.dataset.result)
            && (!query || search.includes(query));
          row.hidden = !match;
          if (match) visible++;
        });
        group.hidden = !visible;
        group.open = visible > 0 && (query !== '' || f.get('result') !== 'All');
        count += visible;
      });
      document.querySelector('#test-count').textContent = `${count.toLocaleString('en-US')} matching tests`;
      document.querySelector('#no-tests').hidden = count > 0;
    });
  }
  const featureForm = document.querySelector('#feature-filters');
  if (featureForm) {
    const rows = [...document.querySelectorAll('[data-feature]')];
    bindFilters(featureForm, () => {
      const f = new FormData(featureForm), query = f.get('query').trim().toLowerCase();
      let count = 0;
      rows.forEach(row => {
        const match = (f.get('standard') === 'All' || row.dataset.standards.split(' ').includes(f.get('standard')))
          && (f.get('status') === 'All' || f.get('status') === row.dataset.status)
          && row.textContent.toLowerCase().includes(query);
        row.hidden = !match;
        if (match) count++;
      });
      document.querySelector('#feature-count').textContent = `${count} feature ${count === 1 ? 'entry' : 'entries'}`;
      document.querySelector('#no-features').hidden = count > 0;
    });
  }
})();
