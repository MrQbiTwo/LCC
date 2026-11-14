// Apply the saved or system theme before styles paint; storage may be unavailable.
(() => {
  let saved;
  try { saved = localStorage.getItem('lcc-theme'); } catch (_) { /* System fallback. */ }
  const systemDark = window.matchMedia('(prefers-color-scheme: dark)').matches;
  document.documentElement.dataset.theme = saved === 'light' || saved === 'dark'
    ? saved : systemDark ? 'dark' : 'light';
})();
